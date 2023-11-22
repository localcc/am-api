
use am_api::error::Error;
use am_api::resource::catalog::music_video::{MusicVideo, MusicVideoAttributesExtension};


mod common;

#[tokio::test]
async fn fetch_video() -> Result<(), Error> {
    let client = common::create_client();

    let music_video = MusicVideo::get()
        .extend(MusicVideoAttributesExtension::ArtistUrl)
        .one(&client, "1093958719")
        .await?
        .expect("music video fetch returned none");

    let attributes = music_video
        .attributes
        .expect("music video fetch returned a music video without attributes");

    assert_eq!(
        attributes.artist_url,
        Some(String::from(
            "https://music.apple.com/us/artist/polyphia/640294344"
        ))
    );
    assert_eq!(attributes.name, "Nightmare");

    Ok(())
}
