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
        song_key: Pubkey
    ) -> Result<()> {

        let mut profile= self.profile.clone();
        let mut song= self.song.clone();

        require!(song_key == self.song.key(), SoundHavenError::InvalidSong);

        profile.likes_count += 1;
        song.song_likes_count += 1;

        Ok(())
    }
}