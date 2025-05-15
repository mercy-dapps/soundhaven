use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, token::{Mint, TokenAccount, Token}
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeTokenVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_shn: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE
       )]
    
    pub config: Account<'info, Config>,

       #[account(
        init,
        payer = admin,
        associated_token::mint = mint_shn,
        associated_token::authority = config,
    )]
    pub vault_token: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>
}

impl<'info> InitializeTokenVault<'info> {
    pub fn initialize_token_vault(
        &mut self, 
        seed: u64,
        bumps: &InitializeTokenVaultBumps
    ) -> Result<()> {
       
       self.config.set_inner(Config { 
        seed, 
        admin: self.admin.key(), 
        mint_shn: self.mint_shn.key(), 
        bump: bumps.config
    });

        Ok(())
    }
}