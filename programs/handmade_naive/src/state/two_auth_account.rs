use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TwoAuthParameters {
    pub functions: Vec<TwoAuthFunction>, // TwoAuthFunction::get_init_len()
    pub two_auth_entity: Pubkey, // 32 - Also called Insurance // We could had several keys for several levels of insurance (In case our keys to be stolen)
    pub allowed_issuers: Vec<Pubkey>, // 4 + 32 * len
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TwoAuthArgs {
    pub functions: Vec<TwoAuthFunction>,
    pub allowed_issuers: Vec<Pubkey>,
}

#[account]
pub struct TwoAuth {
    pub two_auth: Option<TwoAuthParameters>,
}

impl TwoAuth {
    pub fn get_init_len(two_auth_args: &Option<TwoAuthArgs>) -> usize {
        match two_auth_args {
            Some(TwoAuthArgs {
                functions,
                allowed_issuers,
            }) => {
                let functions_space = functions.iter().map(|f| f.get_init_len()).sum::<usize>();
                return 8 + 1 + 4 + functions_space + 32 + 4 + 32 * allowed_issuers.len();
            }
            None => 8 + 1,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TwoAuthFunction {
    Always,
    OnMax {
        max: u64,
    },
    CounterResetOnMax {
        max: u64,
        counter: u64,
    },
    CounterResetOnTime {
        // Usually the time is a day
        max: u64,
        duration: Duration,
        counter: u64,
        last_reset_time: i64,
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
            TwoAuthFunction::OnMax { .. } => 1 + 8,
            TwoAuthFunction::CounterResetOnMax { .. } => 1 + 8 + 8,
            TwoAuthFunction::CounterResetOnTime { .. } => 1 + 8 + Duration::LEN + 8 + 8,
            TwoAuthFunction::CounterWithTimeWindow { max: _, window } => {
                1 + 8 + window.get_init_len()
            }
            TwoAuthFunction::DeactivateForUserSpecificWhiteList { white_list } => {
                1 + 4 + 32 * white_list.len()
            }
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
        return 4 + 1 + 4 + (self.duration.get() as usize) * 8 + Duration::LEN + 8;
    }

    pub fn new(duration: Duration, time: i64) -> Self {
        CircularTimeWindow {
            window: vec![0; duration.get() as usize],
            start_index: 0,
            duration: duration,
            last_value_time: time,
        }
    }

    pub fn get_duration(&self) -> Duration {
        self.duration.clone()
    }

    pub fn add(&mut self, time: i64, value: u64) {
        let diff = self.get_time_difference_duration(time);
        if diff == 0 {
            self.window[self.start_index as usize] += value;
        } else {
            let new_index = (self.start_index as usize + diff as usize) % self.window.len();
            self.circular_reset_values_between_indexes(self.start_index as usize, new_index);
            self.start_index = new_index as u8;
            self.window[self.start_index as usize] = value;
        }
        self.last_value_time = time;
    }

    pub fn get(&self, index: u8) -> u64 {
        self.window[(self.start_index + index) as usize % self.window.len()]
    }

    pub fn get_count(&self) -> u64 {
        return self.window.iter().sum();
    }

    /*
        Reset to 0 the values between the two indexes
        Index1 and Index2 are not included in the reset !
    */
    fn circular_reset_values_between_indexes(&mut self, index1: usize, index2: usize) {
        if index1 < index2 {
            for i in (index1 + 1)..index2 {
                self.window[i] = 0;
            }
        } else {
            for i in (index1 + 1)..self.window.len() {
                self.window[i] = 0;
            }
            for i in 0..index2 {
                self.window[i] = 0;
            }
        }
    }

    fn get_time_difference_duration(&self, time: i64) -> u8 {
        let diff = time - self.last_value_time;
        if diff < 0 {
            // We considere that it is during the same period of time, we don't want an error raised
            return 0;
        }
        match self.duration {
            Duration::Seconds(t) => Self::u8_with_overflow(diff, t),
            Duration::Minutes(t) => Self::u8_with_overflow(diff / 60, t),
            Duration::Hours(t) => Self::u8_with_overflow(diff / 3600, t),
            Duration::Days(t) => Self::u8_with_overflow(diff / 86400, t),
            Duration::Weeks(t) => Self::u8_with_overflow(diff / 604800, t),
        }
    }

    fn u8_with_overflow(time_diff: i64, overflow_value: u8) -> u8 {
        if time_diff > overflow_value as i64 {
            // Overflow case, it means we return to the start of the windows
            return overflow_value;
        } else {
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
    pub const LEN: usize = 2;

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
        let window = super::CircularTimeWindow::new(super::Duration::Days(30), 0);
        assert_eq!(window.get_count(), 0);
        assert_eq!(window.window.len(), 30);
    }
    #[test]
    fn circular_time_window_get_count() {
        let day = 86400;
        let mut window = super::CircularTimeWindow::new(super::Duration::Days(30), 0);
        for i in 0..35 {
            window.add(i * day, 1);
        }
        assert!(window.get_count() == 30);
        window.add(100 * day, 0);
        assert!(window.get_count() == 0);
    }

    #[test]
    fn circular_time_window_2_add() {
        let day = 86400;
        let mut window = super::CircularTimeWindow::new(super::Duration::Days(2), 0);
        window.add(0 * day, 4); // |4|0|
        window.add(1 * day, 1); // |4|1|
        window.add(2 * day, 1); // |1|1|
        assert!(window.get_count() == 2);
    }

    #[test]
    fn circular_time_window_in_bewteen_values() {
        let day = 86400;
        let mut window = super::CircularTimeWindow::new(super::Duration::Days(3), 0);
        for i in 0..3 {
            window.add(i * day, 1);
        }
        window.add(3 * day, 1); // |1|1|1|
                                //  ^
        assert_eq!(window.start_index, 0);
        assert_eq!(window.get_count(), 3);

        window.add(5 * day, 1); // |1|1|1| => |1|0|1|
                                //    x ^          ^
        assert_eq!(window.get_count(), 2);
        assert_eq!(window.window[1], 0);
    }
}
