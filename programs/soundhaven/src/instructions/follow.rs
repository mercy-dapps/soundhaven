use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(profile_owner: Pubkey, follow_profile_owner: Pubkey)]
pub struct Follow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", profile_owner.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"profile", follow_profile_owner.as_ref()],
        bump = follow_profile.bump
    )]
    pub follow_profile: Account<'info, Profile>,
     
    pub system_program: Program<'info, System>
}

impl<'info> Follow<'info> {
    pub fn follow(
        &mut self, 
        profile_owner: Pubkey,
        follow_profile_owner: Pubkey
    ) -> Result<()> {
        
        require!(profile_owner == self.profile.profile_owner, SoundHavenError::InvalidProfile);
        require!(follow_profile_owner == self.follow_profile.profile_owner, SoundHavenError::InvalidProfile);

        self.profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: true, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count + 1, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 

        self.follow_profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: true, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count + 1, 
            bump: self.profile.bump 
        }); 

        Ok(())
    }
}