use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
#[instruction(playlist_id: u64)]
pub struct CreatePlaylist<'info> {
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
        seeds = [b"playlist", user.key().as_ref(), playlist_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Playlist::INIT_SPACE,
    )]
    pub playlist: Account<'info, Playlist>,

    pub system_program: Program<'info, System>
}

impl<'info> CreatePlaylist<'info>  {
    pub fn create_playlist(
        &mut self, 
        playlist_id: u64,
        playlist_title: String, 
        playlist_description: String,
        playlist_thumbnail_url: String,
        playlist_visibility: bool,
        bumps: &CreatePlaylistBumps
    ) -> Result<()> {
        require!(playlist_title.len() <= 50, SoundHavenError::PlaylistTitleTooLong);
        require!(playlist_description.len() <= 200, SoundHavenError::PlaylistDescriptionTooLong);
        require!(playlist_thumbnail_url.len() <= 200, SoundHavenError::PlaylistThumbnailUrlTooLong);

        let playlist_owner = self.user.key();
       
        self.playlist.set_inner(Playlist { 
            playlist_id,
            playlist_owner,
            playlist_title, 
            playlist_description, 
            playlist_thumbnail_url, 
            playlist_track_count: 0, 
            playlist_visibility, 
            bump: bumps.playlist
        });

        self.profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: true, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count + 1, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 

        Ok(())
    }
}