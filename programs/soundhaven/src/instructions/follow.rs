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
        seeds = [b"profile", follow_profile_owner.key().as_ref()],
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
            has_paid: self.profile.has_paid, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count.checked_add(1).unwrap(), 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 

        self.follow_profile.set_inner(Profile { 
            seed: self.follow_profile.seed, 
            profile_owner: self.follow_profile.profile_owner, 
            name: self.follow_profile.name.clone(), 
            profile_img_avatar: self.follow_profile.profile_img_avatar.clone(), 
            description: self.follow_profile.description.clone(), 
            is_artist: self.follow_profile.is_artist, 
            has_paid: self.follow_profile.has_paid, 
            song_count: self.follow_profile.song_count, 
            playlist_count: self.follow_profile.playlist_count, 
            likes_count: self.follow_profile.likes_count, 
            following_count: self.follow_profile.following_count, 
            followers_count: self.follow_profile.followers_count.checked_add(1).unwrap(), 
            bump: self.follow_profile.bump 
        }); 

        Ok(())
    }
}