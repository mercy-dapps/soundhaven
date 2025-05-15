use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};

use crate::state::*;
use crate::error::SoundHavenError;

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"state", admin.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Pay<'info> {
    pub fn pay(&mut self) -> Result<()> {

        require!(self.profile.is_artist == true, SoundHavenError::InvalidProfile);

        let cpi_program = self.system_program.to_account_info();

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

        let amount= 1 * LAMPORTS_PER_SOL;

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}