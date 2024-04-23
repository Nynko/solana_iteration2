use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TwoAuthParameters {
    pub functions: Vec<TwoAuthFunction>, // 4 + 11* len
    pub two_auth_entity: Pubkey,         // 32 - Also called Insurance
    pub allowed_issuers: Vec<Pubkey>,    // 4 + 32 * len
}

impl TwoAuthParameters {
    pub fn get_init_len(functions: &Vec<TwoAuthFunction>, allowed_issuers: &Vec<Pubkey>) -> usize {
        return 8 + 4 + 11 * functions.len() + 32 + 4 + 32 * allowed_issuers.len();
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TwoAuthFunction {
    // 1 + MAX(all fields)  = 1 + 8 + space(Duration) = 9 + 2 = 11
    Always,
    OnMax {
        max: u64, 
    },
    Random,
    CounterResetOnMax {
        max: u64, 
        counter: u64, // Make sure to handle the overflow case
    },
    CounterResetOnTime {
        // Usually the time is a day
        max: u64,
        duration: Duration,
        counter: u64, // Make sure to handle the overflow case
        reset_time: i64,
    },
    CounterWithTimeWindow {
        // Usually the time is a month (30 days)
        max: u64,
        window: CircularTimeWindow, 
    },
    // DeactivateForGeneralWhiteList, // This white list is derived from the receiver address: the insurance has to add their addresss to the white list (to white list the receiver token account)
    DeactivateForUserSpecificWhiteList {
        white_list: Vec<Pubkey>,
    },
}

impl TwoAuthFunction {
    pub fn get_init_len(&self) -> usize {
        match self {
            TwoAuthFunction::Always => 1,
            TwoAuthFunction::OnMax { max } => 1 + 8,
            TwoAuthFunction::Random => 1,
            TwoAuthFunction::CounterResetOnMax { max, counter } => 1 + 8 + 8,
            TwoAuthFunction::CounterResetOnTime { max, duration, counter, reset_time } => 1 + 8 + Duration::LEN + 8 + 8,
            TwoAuthFunction::CounterWithTimeWindow { max, window } => 1 + 8 + window.get_init_len(),
            TwoAuthFunction::DeactivateForUserSpecificWhiteList {white_list} => 1 + 4 + 32 * white_list.len(),
        }
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CircularTimeWindow {
    start_index: u8,
    window: Vec<u64>,
    duration: Duration, // A duration of 30 days will hold a 30 value windows of a day
    last_value_time: i64,
}

impl CircularTimeWindow {

    pub fn get_init_len(&self) -> usize {
        return 1 + 4+  self.window.len() * 8 + Duration::LEN + 8;
    }

    pub fn new(duration: Duration, time: i64) -> Self {
        CircularTimeWindow {
            window: vec![0; duration.get() as usize],
            start_index: 0,
            duration: duration,
            last_value_time: time,
        }
    }

    pub fn add(&mut self, time: i64, value: u64) {
        let diff = self.get_time_difference_duration(time);
        if diff > self.duration.get() {
            self.start_index = 0;
            self.window = vec![0; self.duration.get() as usize];
            self.window[self.start_index as usize] = value;
            
        }  else {
            let new_index = (self.start_index as usize + diff as usize) % self.duration.get() as usize;
            if new_index < self.start_index as usize {
                for i in 0..new_index { // reset all the values that are no longer in the window
                    self.window[i] = 0;
                }
            }
            self.start_index = new_index as u8;
            self.window[self.start_index as usize] = value;  
        }
        self.last_value_time = time;

    }

    pub fn get(&self, index: u8) -> u64 {
        self.window[(self.start_index + index) as usize % self.duration.get() as usize]
    }

    pub fn get_count(&self) -> u64 {
        return self.window.iter().sum();
    }


    fn get_time_difference_duration(&self, time: i64) -> u8 {
        let diff = time - self.last_value_time;
        if diff < 0 { // We considere that it is during the same period of time, we don't want an error 
            return 0;
        }
        match self.duration {
            Duration::Seconds(t) => Self::u8_with_overflow(diff,t),
            Duration::Minutes(t) => Self::u8_with_overflow(diff / 60,t),
            Duration::Hours(t) => Self::u8_with_overflow(diff / 3600,t),
            Duration::Days(t) => Self::u8_with_overflow(diff / 86400,t),
            Duration::Weeks(t) => Self::u8_with_overflow(diff / 604800,t),
        }
    }

    fn u8_with_overflow(time_diff: i64, overflow_value: u8) -> u8 {
        if time_diff > overflow_value as i64 { // Overflow case, it means we return to the start of the windows
            return overflow_value + 1;
        }
        else {
            return time_diff as u8;
        }
    }
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Duration {
    // Space = 1 + 1 = 2
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
    Days(u8),
    Weeks(u8),
}

impl Duration {
    pub const LEN : usize = 2;

    pub fn get(&self) -> u8 {
        match self {
            Duration::Seconds(t)
            | Duration::Minutes(t)
            | Duration::Hours(t)
            | Duration::Days(t)
            | Duration::Weeks(t) => *t,
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn circular_time_window() {
        let mut window = super::CircularTimeWindow::new(super::Duration::Days(30), 0);
        assert_eq!(window.get_count(), 0);
        assert_eq!(window.window.len(), 30);
    }
    #[test]
    fn circular_time_window_add() {
        let day = 86400;
        let mut window = super::CircularTimeWindow::new(super::Duration::Days(30), 0);
        for i in 0..35 {
            window.add(i*day, 1);
        }
        assert!(window.get_count() == 30);
        window.add(100*day, 0);
        assert!(window.get_count() == 0);
    }
}