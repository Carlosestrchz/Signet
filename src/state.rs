use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("GEtkYSQLhiSALNT7YRMmKFnbZXd44aZWfScD63edGgoP");

#[program]
pub mod signet {
    use super::*;

    pub fn register_content(ctx: Context<RegisterContent>, file_hash: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let clock = Clock::get()?;

        registry.author = ctx.accounts.author.key();
        registry.file_hash = file_hash;
        registry.timestamp = clock.unix_timestamp;
        registry.is_revoked = false;
        registry.revocation_reason = None;

        Ok(())
    }

    pub fn revoke_content(ctx: Context<RevokeContent>, file_hash: String, reason: String) -> Result<()> {
        require!(reason.len() <= 50, CustomError::ReasonTooLong);

        let registry = &mut ctx.accounts.registry;
        registry.is_revoked = true;
        registry.revocation_reason = Some(reason);

        Ok(())
    }

    pub fn delete_content(_ctx: Context<DeleteContent>, _file_hash: String) -> Result<()> {
        // Rust y Anchor manejan el cierre de la cuenta automáticamente
        // a través del macro `close = author` definido en instructions.rs
        Ok(())
    }
}