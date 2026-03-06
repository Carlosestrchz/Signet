use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct RegisterContent<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + 200, // ~167 bytes, ajustado a los requerimientos
        // La clave del éxito: prefijo, autor y los primeros 32 bytes del hash
        seeds = [b"registry", author.key().as_ref(), file_hash.as_bytes()[..32].as_ref()],
        bump
    )]
    pub registry: Account<'info, ContentRegistry>,

    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct RevokeContent<'info> {
    #[account(
        mut,
        seeds = [b"registry", author.key().as_ref(), file_hash.as_bytes()[..32].as_ref()],
        bump,
        has_one = author
    )]
    pub registry: Account<'info, ContentRegistry>,

    #[account(mut)]
    pub author: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct DeleteContent<'info> {
    #[account(
        mut,
        close = author,
        seeds = [b"registry", author.key().as_ref(), file_hash.as_bytes()[..32].as_ref()],
        bump,
        has_one = author
    )]
    pub registry: Account<'info, ContentRegistry>,

    #[account(mut)]
    pub author: Signer<'info>,
}

#[error_code]
pub enum CustomError {
    #[msg("La razon de revocacion no puede exceder los 50 caracteres.")]
    ReasonTooLong,
}