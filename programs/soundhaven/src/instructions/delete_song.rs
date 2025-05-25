use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(song_id: u64)]
pub struct DeleteSong<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"song", user.key().as_ref(), song_id.to_le_bytes().as_ref()],
        bump = song.bump,
    )]
    pub song: Account<'info, Song>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
     
    pub system_program: Program<'info, System>
}

impl<'info> DeleteSong<'info> {
    pub fn delete_song(&mut self) -> Result<()> {

        require!(self.user.key() == self.song.song_owner, SoundHavenError::InvalidProfile);

        self.profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: self.profile.has_paid, 
            song_count: self.profile.song_count.checked_sub(1).unwrap(), 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 

        Ok(())
    }
}