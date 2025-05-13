use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DeletePlaylist<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user
    )]
    pub playlist: Account<'info, Playlist>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
    
    pub system_program: Program<'info, System>
}

impl<'info> DeletePlaylist<'info> {
    pub fn delete_playlist(&mut self) -> Result<()> {

        let mut profile = self.profile.clone();
        profile.playlist_count -= 1;

        Ok(())
    }
}