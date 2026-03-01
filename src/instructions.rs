use anchor_lang::prelude::*;
use crate::state::ContentRegistry;

//definimos las cuentas que participan en la transacción
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

//funcion que ejecuta la logica
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


//UPDATE: LOGICA PARA REVOCAR ARCHIVO

#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct RevokeContentContext<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        seeds = [b"signet_file", file_hash.as_bytes()],
        bump,

        //verifica que el que firma sea el autor original
        has_one = author
    )]
    pub registry: Account<'info, ContentRegistry>,
}

pub fn revoke_content(ctx: Context<RevokeContentContext>, _file_hash: String, reason: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;

    //validacion: maximo 50 caracteres
    if reason.chars().count() > 50 {
        return err!(ErrorCode::ReasonTooLong);
    }

    registry.is_revoked = true;
    registry.revocation_reason = Some(reason);

    msg!("Signet: Archivo revocado exitosamente.");
    Ok(())
}


//DELETE: ELIMINAR REGISTRO Y RECUPERAR "RENT"

#[derive(Accounts)]
#[instruction(file_hash: String)]
pub struct DeleteContentContext<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,

        //destruye la cuenta y devuelve los SOL al autor
        close = author,
        seeds = [b"signet_file", file_hash.as_bytes()],
        bump,
        has_one = author
    )]
    pub registry: Account<'info, ContentRegistry>,
}

pub fn delete_content(_ctx: Context<DeleteContentContext>, _file_hash: String) -> Result<()> {
    msg!("Signet: Registro destruido permanentemente y fondos devueltos.");
    Ok(())
}


//MANEJO DE ERRORES

#[error_code]
pub enum ErrorCode {
    #[msg("La razon de revocacion no puede exceder los 50 caracteres.")]
    ReasonTooLong,
}