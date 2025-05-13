use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeTokenVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_shn: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_shn,
        associated_token::authority = admin,
        associated_token::token_program = token_program
    )]
    pub admin_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE
       )]
       pub config: Account<'info, Config>,

       #[account(
        init,
        payer = admin,
        associated_token::mint = mint_shn,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>
}

impl<'info> InitializeTokenVault<'info> {
    pub fn initialize_vault(
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