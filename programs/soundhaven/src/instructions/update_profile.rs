use anchor_lang::prelude::*;

use crate::state::*;
use  crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump,
        
    )]
    pub profile: Account<'info, Profile>,
     
    pub system_program: Program<'info, System>
}

impl<'info> UpdateProfile<'info> {
    pub fn update_profile(
        &mut self, 
        seed: u64,
        name: String, 
        profile_img_avatar: String, 
        description: String,
    ) -> Result<()> {
        require!(name.len() <= 50, SoundHavenError::NameTooLong);
        require!(description.len() <= 200, SoundHavenError::DescriptionTooLong);
        require!(profile_img_avatar.len() <= 200, SoundHavenError::ProfileImgUrlTooLong);

        self.profile.set_inner(Profile { 
            seed,
            profile_owner: self.profile.profile_owner,
            name, 
            profile_img_avatar, 
            description, 
            is_artist: self.profile.is_artist, 
            has_paid: self.profile.has_paid, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count,
            bump: self.profile.bump
        });

        Ok(())
    }
}