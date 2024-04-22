#!/bin/bash


# Exit if any command fails
set -e


# Remove current idl
rm /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/target/deploy/handmade_naive-keypair.json

# Build with anchor
anchor build

# Generate new pubkey
new_id=$(solana-keygen pubkey /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/target/deploy/handmade_naive-keypair.json)

# Replace in lib.rs
sed -i '' "s/declare_id!(\".*\")/declare_id!(\"$new_id\")/g" /Users/nicolasbeaudouin/Documents/Project1/handmade_naive/programs/handmade_naive/src/lib.rs
