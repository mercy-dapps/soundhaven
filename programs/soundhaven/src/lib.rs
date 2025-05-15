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

    pub fn like(ctx: Context<Like>, _song_id: u64, song_owner: Pubkey) -> Result<()> {
        ctx.accounts.like(song_owner)?;

        Ok(())
    }
    // test not working yet
    pub fn follow(ctx: Context<Follow>, profile_owner: Pubkey, follow_profile_owner: Pubkey) -> Result<()> {
        ctx.accounts.follow(profile_owner, follow_profile_owner)?;
        Ok(())
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)
    }

    pub fn initialize_token_vault(ctx: Context<InitializeTokenVault>, seed: u64) -> Result<()> {
        ctx.accounts.initialize_token_vault(seed, &ctx.bumps)
    }

    pub fn delete_playlist(ctx: Context<DeletePlaylist>) -> Result<()> {
        ctx.accounts.delete_playlist()
    }

    pub fn delete_song(ctx: Context<DeleteSong>) -> Result<()> {
        ctx.accounts.delete_song()
    }

    pub fn delete_profile(ctx: Context<DeleteProfile>) -> Result<()> {
        ctx.accounts.delete_profile()
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        seed: u64,
        name: String,
        profile_img_avatar: String,
        description: String,
    ) -> Result<()> {
        ctx.accounts.update_profile(seed, name, profile_img_avatar, description)?;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>) -> Result<()> {
        ctx.accounts.pay()
    }

    pub fn claim(ctx: Context<ClaimTokenReward>, amount: u64) -> Result<()> {
        ctx.accounts.claim(amount)
    }

    pub fn withdraw_fund(ctx: Context<WithdrawFund>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

}
