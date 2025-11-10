use anchor_lang::constant;
#[constant]
pub const AUTH_SEED: &[u8] = b"vault_auth_seed";

#[constant]
pub const GLOBAL_CONFIG_SEED: &[u8] = b"global_config";

#[constant]
pub const POOL_SEED: &[u8] = b"pool";

#[constant]
pub const POOL_VAULT_SEED: &[u8] = b"pool_vault";

#[constant]
pub const METADATA_SEED: &[u8] = b"metadata";

#[constant]
pub const EVENT_AUTHORITY:&[u8] = b"__event_authority";

#[constant]
pub const PLATFORM_CONFIG_SEED:&[u8] = b"platform_config";