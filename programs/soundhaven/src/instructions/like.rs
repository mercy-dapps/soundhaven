use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(song_id: u64, song_owner: Pubkey)]
pub struct Like<'info> {
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
        seeds = [b"song", song_owner.as_ref(), song_id.to_le_bytes().as_ref()],
        bump = song.bump,
    )]
    pub song: Account<'info, Song>,
     
    pub system_program: Program<'info, System>
}

impl<'info> Like<'info> {
    pub fn like(
        &mut self,
        song_owner: Pubkey
    ) -> Result<()> {

        require!(song_owner == self.song.song_owner, SoundHavenError::InvalidSong);

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
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 

        self.song.set_inner(Song { 
            song_id: self.song.song_id, 
            song_owner: self.song.song_owner, 
            song_title: self.song.song_title.clone(), 
            song_url: self.song.song_url.clone(), 
            song_thumbnail_url: self.song.song_thumbnail_url.clone(), 
            song_likes_count: self.song.song_likes_count.checked_add(1).unwrap(), 
            bump: self.song.bump 
        });

        Ok(())
    }
}