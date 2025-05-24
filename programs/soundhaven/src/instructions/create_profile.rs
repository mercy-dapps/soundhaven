use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"profile", user.key().as_ref()],
        bump,
        space = 8 + Profile::INIT_SPACE
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>
}

impl<'info> CreateProfile<'info> {
    pub fn create_profile(
        &mut self, 
        seed: u64,
        name: String, 
        profile_img_avatar: String, 
        description: String,
        is_artist: bool,
        bumps: &CreateProfileBumps
    ) -> Result<()> {
        require!(name.len() <= 50, SoundHavenError::NameTooLong);
        require!(description.len() <= 200, SoundHavenError::DescriptionTooLong);
        require!(profile_img_avatar.len() <= 200, SoundHavenError::ProfileImgUrlTooLong);

        self.profile.set_inner(Profile { 
            seed,
            profile_owner: self.profile.key(),
            name, 
            profile_img_avatar, 
            description, 
            is_artist, 
            has_paid: false, 
            song_count: 0, 
            playlist_count: 0, 
            likes_count: 0, 
            following_count: 0, 
            followers_count: 0,
            bump: bumps.profile
        });

        Ok(())
    }
}