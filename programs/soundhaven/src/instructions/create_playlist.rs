use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
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
        seeds = [b"playlist", user.key().as_ref(), &profile.playlist_count.to_le_bytes()],
        bump,
        space = 8 + Playlist::INIT_SPACE,
    )]
    pub playlist: Account<'info, Playlist>,

    pub system_program: Program<'info, System>
}

impl<'info> CreatePlaylist<'info>  {
    pub fn create_playlist(
        &mut self, 
        playlist_title: String, 
        playlist_description: String,
        playlist_thumbnail_url: String,
        playlist_visibility: bool,
        bumps: &CreatePlaylistBumps
    ) -> Result<()> {
        require!(playlist_title.len() > 50, SoundHavenError::PlaylistTitleTooLong);
        require!(playlist_description.len() > 200, SoundHavenError::PlaylistDescriptionTooLong);
        require!(playlist_thumbnail_url.len() > 200, SoundHavenError::PlaylistThumbnailUrlTooLong);

        let mut profile = self.profile.clone();
        let playlist_owner = self.user.key();
       
        self.playlist.set_inner(Playlist { 
            playlist_owner,
            playlist_title, 
            playlist_description, 
            playlist_thumbnail_url, 
            playlist_track_count: 0, 
            playlist_visibility, 
            bump: bumps.playlist
        });

        profile.playlist_count += 1;

        Ok(())
    }
}