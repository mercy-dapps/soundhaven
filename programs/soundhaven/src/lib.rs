pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9e15JQmSJKiRK3tVkqa2M4pUFAxJqR5F9RMwP3aFmw8W");

#[program]
pub mod soundhaven {
    use super::*;

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        seed: u64,
        name: String,
        profile_img_avatar: String,
        description: String,
        is_artist: bool
    ) -> Result<()> {
        ctx.accounts.create_profile(seed, name, profile_img_avatar, description, is_artist, &ctx.bumps)?;
        Ok(())
    }

    pub fn create_song(
        ctx: Context<CreateSong>,
        song_id: u64,
        song_title: String,
        song_url: String,
        song_thumbnail_url: String
    ) -> Result<()> {
        ctx.accounts.create_song(song_id, song_title, song_url, song_thumbnail_url, &ctx.bumps)?;

        Ok(())
    }

    pub fn create_playlist(
        ctx: Context<CreatePlaylist>,
        playlist_id: u64,
        playlist_title: String, 
        playlist_description: String,
        playlist_thumbnail_url: String,
        playlist_visibility: bool,
    ) -> Result<()> {
        ctx.accounts.create_playlist(playlist_id, playlist_title, playlist_description, playlist_thumbnail_url, playlist_visibility, &ctx.bumps)?;

        Ok(())
    }

    pub fn like(ctx: Context<Like>, song_key: Pubkey) -> Result<()> {
        ctx.accounts.like(song_key)?;

        Ok(())
    }

    // pub fn follow(ctx: Context<Follow>, follow_key: Pubkey) -> Result<()> {
    //     ctx.accounts.follow(follow_key)?;
    //     Ok(())
    // }


}
