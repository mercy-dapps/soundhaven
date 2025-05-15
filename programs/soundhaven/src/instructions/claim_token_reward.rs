use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount}
};

use crate::state::*;

#[derive(Accounts)]
pub struct ClaimTokenReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_shn: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_shn,
        associated_token::authority = user,
    )]
    pub user_shn_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
       
       )]
       pub config: Account<'info, Config>,
    #[account(
        mut,
        associated_token::mint = mint_shn,
        associated_token::authority = config,
    )]
    pub vault_token: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimTokenReward<'info>  {
    pub fn claim(&mut self, amount: u64) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint_shn.to_account_info(),
            to: self.user_shn_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seeds = &[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)
    }
}