use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(song_id: u64, seed: u64)]
pub struct CreateSong<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init,
        payer = user,
        seeds = [b"song", user.key().as_ref(), song_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Song::INIT_SPACE,
    )]
    pub song: Account<'info, Song>,

    pub system_program: Program<'info, System>
}

impl<'info> CreateSong<'info>  {
    pub fn create_song(
        &mut self, 
        song_id: u64,
        song_title: String, 
        song_url: String,
        song_thumbnail_url: String,
        bumps: &CreateSongBumps
    ) -> Result<()> {
        require!(song_title.len() <= 50, SoundHavenError::SongTitleTooLong);
        require!(song_url.len() <= 200, SoundHavenError::SongUrlTooLong);
        require!(song_thumbnail_url.len() <= 200, SoundHavenError::SongThumbnailUrlTooLong);

        require!(self.profile.has_paid == true, SoundHavenError::PayToUploadSong);

        self.profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: self.profile.has_paid, 
            song_count: self.profile.song_count.checked_add(1).unwrap(), 
            playlist_count: self.profile.playlist_count, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 
        
        let song_owner = self.user.key();
        self.song.set_inner(Song { 
            song_id,
            song_owner, 
            song_title, 
            song_url, 
            song_thumbnail_url, 
            song_likes_count: 0, 
            bump: bumps.song
        });
        Ok(())
    }
}