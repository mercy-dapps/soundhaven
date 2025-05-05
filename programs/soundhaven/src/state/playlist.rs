use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Playlist {
    pub playlist_owner: Pubkey,
    #[max_len(50)]
    pub playlist_title: String,
    #[max_len(200)]
    pub playlist_description: String,
    #[max_len(200)]
    pub playlist_thumbnail_url: String,
    pub playlist_track_count: u64,
    pub playlist_visibility: bool,
    pub bump: u8
}