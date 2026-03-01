use anchor_lang::prelude::*;
use crate::state::ContentRegistry;

//Definimos las cuentas que participan en la transacción
#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct RegisterContentContext<'info> {

    #[account(mut)]
    pub author: Signer<'info>,


    #[account(
        init,
        payer = author,

        space = 8 + 32 + (4 + 64) + 8 + 1 + (1 + 4 + 50),
        seeds = [b"signet_file", file_hash.as_bytes()],
        bump
    )]
    pub registry: Account<'info, ContentRegistry>,

    pub system_program: Program<'info, System>,
}

//función que ejecuta la lógica
pub fn register_content(ctx: Context<RegisterContentContext>, file_hash: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    let clock = Clock::get()?;


    registry.author = ctx.accounts.author.key();
    registry.file_hash = file_hash;
    registry.timestamp = clock.unix_timestamp;
    registry.is_revoked = false;
    registry.revocation_reason = None;

    msg!("Signet: Archivo validado y registrado exitosamente.");
    Ok(())
}