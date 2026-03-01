use anchor_lang::prelude::*;

#[account]
pub struct ContentRegistry {
    pub author: Pubkey,

    //4 + 64 bytes SHA-256 en formato hexadecimal
    pub file_hash: String,

    //Fecha y hora
    pub timestamp: i64,

    pub is_revoked: bool,

    //Máximo 50 caracteres
    pub revocation_reason: Option<String>,
}