use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(follow_key: Pubkey)]
pub struct Follow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"profile", follow_key.as_ref()],
        bump = follow_profile.bump
    )]
    pub follow_profile: Account<'info, Profile>,
     
    pub system_program: Program<'info, System>
}

impl<'info> Follow<'info> {
    pub fn follow(
        &mut self, 
        follow_key: Pubkey,
    ) -> Result<()> {

        let mut profile= self.profile.clone();

        let mut follow_profile= self.follow_profile.clone();

        msg!("{:?}", follow_profile.key());
        msg!("{:?}", follow_key);

        require!(follow_key == self.follow_profile.key(), SoundHavenError::InvalidProfile);

        profile.following_count += 1;
        follow_profile.followers_count += 1;


        Ok(())
    }
}