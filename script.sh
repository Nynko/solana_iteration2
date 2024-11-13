#!/bin/bash


# Exit if any command fails
set -e


# Remove current idl
rm -f /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/target/deploy/handmade_naive-keypair.json
rm -f /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/tests/data/handmade_naive_local.json

# Build with anchor
anchor build

# Generate new pubkey
new_id=$(solana-keygen pubkey /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/target/deploy/handmade_naive-keypair.json)

# Replace in lib.rs
sed -i '' "s/declare_id!(\".*\")/declare_id!(\"$new_id\")/g" /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/programs/handmade_naive/src/lib.rs


# Define the path to your config file
config_file="/Users/nicolasbeaudouin/Documents/Project1/handmade_naive/Anchor.toml"

# Replace in config.toml for localnet
awk -v id="$new_id" '
    /^\[programs\.localnet\]$/ { f = 1 }
    f && /handmade_naive =/ { sub(/=.*/, "= \"" id "\""); f = 0 }
    1' "$config_file" > tmp && mv tmp "$config_file"

# Replace in config.toml for devnet
awk -v id="$new_id" '
    /^\[programs\.devnet\]$/ { f = 1 }
    f && /handmade_naive =/ { sub(/=.*/, "= \"" id "\""); f = 0 }
    1' "$config_file" > tmp && mv tmp "$config_file"