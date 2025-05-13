use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub seed: u64,
    pub profile_owner: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub profile_img_avatar: String,
    #[max_len(200)]
    pub description: String,
    pub is_artist: bool,
    pub has_paid: bool,
    pub song_count: u32,
    pub playlist_count: u32,
    pub likes_count: u64,
    pub following_count: u64,
    pub followers_count: u64,
    pub bump: u8
}