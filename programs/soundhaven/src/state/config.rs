use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64,
    pub admin: Pubkey,
    pub mint_shn: Pubkey,
    pub bump: u8,
}
