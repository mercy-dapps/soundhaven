use anchor_lang::prelude::*;

#[error_code]
pub enum SoundHavenError {
    #[msg("Name too long - max length of 50 characters")]
    NameTooLong,
    #[msg("Profile image url link is too long")]
    ProfileImgUrlTooLong,
    #[msg("Description is too long - max length of 200 characters")]
    DescriptionTooLong,

    #[msg("Song title is too long - max length of 50 characters")]
    SongTitleTooLong,
    #[msg("Song url is too long - max length of 200 characters")]
    SongUrlTooLong,
    #[msg("Song thumbnail url is too long - max length of 200 characters")]
    SongThumbnailUrlTooLong,

    #[msg("Playlist title is too long - max length of 50 characters")]
    PlaylistTitleTooLong,
    #[msg("Playlist description is too long -  max length of 200 characters")]
    PlaylistDescriptionTooLong,
    #[msg("Playlist thumbnail url is too long -  max length of 200 characters")]
    PlaylistThumbnailUrlTooLong,

    #[msg("Invalid profile")]
    InvalidProfile,
    #[msg("Invalid song")]
    InvalidSong,

    #[msg("Pay to upload song")]
    PayToUploadSong,
}
