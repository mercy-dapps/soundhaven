use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DeleteSong<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user
    )]
    pub song: Account<'info, Song>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>
}

impl<'info> DeleteSong<'info> {
    pub fn delete_song(&mut self) -> Result<()> {

        let mut profile = self.profile.clone();
        profile.song_count -= 1;

        Ok(())
    }
}