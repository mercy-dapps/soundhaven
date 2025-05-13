use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DeleteProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user
    )]
    pub profile: Account<'info, Profile>,
     
    pub system_program: Program<'info, System>
}

impl<'info> DeleteProfile<'info> {
    pub fn delete_profile() -> Result<()> {

        Ok(())
    }
}