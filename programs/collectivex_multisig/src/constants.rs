use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "collectiveX";
pub const DISCRIMINATOR : usize = 8;
pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const PROGRAM_CONFIG_SEED: &[u8] = b"program_config";
pub const SPENDING_LIMIT_SEED: &[u8] = b"spending_limit";
pub const TRANSACTION_SEED: &[u8] = b"transaction";