use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Song {
    pub song_owner: Pubkey,
    #[max_len(50)]
    pub song_title: String,
    #[max_len(200)]
    pub song_url: String,
    #[max_len(200)]
    pub song_thumbnail_url: String,
    pub song_likes_count: u64,
    pub bump: u8
}