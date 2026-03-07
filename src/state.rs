use anchor_lang::prelude::*;

#[account]
pub struct ContentRegistry {
    pub author: Pubkey,
    pub file_hash: String,
    pub timestamp: i64,
    pub is_revoked: bool,
    pub revocation_reason: Option<String>,
}
