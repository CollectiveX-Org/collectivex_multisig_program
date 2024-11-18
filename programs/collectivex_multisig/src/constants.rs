use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "collectiveX";
pub const DISCRIMINATOR : usize = 8;
pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const PROGRAM_CONFIG_SEED: &[u8] = b"program_config";