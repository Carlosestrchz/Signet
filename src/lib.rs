use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

// ID temporal. Anchor pondrá el real cuando compile
declare_id!("11111111111111111111111111111111");

#[program]
pub mod signet {
    use super::*;

    // CREATE
    pub fn register_content(ctx: Context<RegisterContentContext>, file_hash: String) -> Result<()> {
        instructions::register_content(ctx, file_hash)
    }

    // UPDATE
    pub fn revoke_content(ctx: Context<RevokeContentContext>, file_hash: String, reason: String) -> Result<()> {
        instructions::revoke_content(ctx, file_hash, reason)
    }

    // DELETE
    pub fn delete_content(ctx: Context<DeleteContentContext>, file_hash: String) -> Result<()> {
        instructions::delete_content(ctx, file_hash)
    }
}