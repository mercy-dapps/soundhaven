use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};

use crate::state::*;

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"state", admin.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Pay<'info> {
    pub fn pay(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let amount= 1 * LAMPORTS_PER_SOL;

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}