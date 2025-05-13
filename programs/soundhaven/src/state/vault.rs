use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub state_bump: u8,
    pub vault_bump: u8
}