use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

// ID temporal. Anchor pondrá el real cuando compile
declare_id!("11111111111111111111111111111111");

#[program]
pub mod signet {
    use super::*;

    pub fn register_content(ctx: Context<RegisterContentContext>, file_hash: String) -> Result<()> {
        instructions::register_content(ctx, file_hash)
    }
}