
use am_api::error::Error;
use am_api::primitive::TrackType;
use am_api::resource::catalog::playlist::{Playlist, PlaylistAttributesExtension};


mod common;

#[tokio::test]
async fn fetch_playlist() -> Result<(), Error> {
    let client = common::create_client();

    let playlist = Playlist::get()
        .extend(PlaylistAttributesExtension::TrackTypes)
        .one(&client, "pl.d51e513de87947fd900d5f048be5e16c")
        .await?
        .expect("playlist fetch returned none");

    let attributes = playlist
        .attributes
        .expect("playlist fetch returned a playlist without attributes");

    assert_eq!(attributes.track_types, Some(vec![TrackType::Song]));
    assert_eq!(attributes.name, "Breakcore");

    Ok(())
}

#[tokio::test]
async fn fetch_charts() -> Result<(), Error> {
    let client = common::create_client();

    let _ = Playlist::get().chart(&client, "us").await?;

    Ok(())
}
