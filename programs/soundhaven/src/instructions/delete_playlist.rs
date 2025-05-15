use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(playlist_id: u64)]
pub struct DeletePlaylist<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"playlist", user.key().as_ref(), playlist_id.to_le_bytes().as_ref()],
        bump = playlist.bump,
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

        self.profile.set_inner(Profile { 
            seed: self.profile.seed, 
            profile_owner: self.profile.profile_owner, 
            name: self.profile.name.clone(), 
            profile_img_avatar: self.profile.profile_img_avatar.clone(), 
            description: self.profile.description.clone(), 
            is_artist: self.profile.is_artist, 
            has_paid: self.profile.has_paid, 
            song_count: self.profile.song_count, 
            playlist_count: self.profile.playlist_count - 1, 
            likes_count: self.profile.likes_count, 
            following_count: self.profile.following_count, 
            followers_count: self.profile.followers_count, 
            bump: self.profile.bump 
        }); 
        Ok(())
    }
}