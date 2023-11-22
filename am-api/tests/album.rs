use am_api::error::Error;
use am_api::resource::catalog::album::{
    Album, AlbumAttributesExtension, AlbumRelationshipType, AlbumViewType,
};

mod common;

#[tokio::test]
async fn fetch_album() -> Result<(), Error> {
    let client = common::create_client();

    let album = Album::get()
        .one(&client, "1676791755")
        .await?
        .expect("album fetch returned none");

    let attributes = album
        .attributes
        .expect("album fetch returned an album without attributes");

    assert_eq!(attributes.name, "Unrequited Love - EP");

    Ok(())
}

#[tokio::test]
async fn fetch_album_extended() -> Result<(), Error> {
    let client = common::create_client();

    let album = Album::get()
        .extend(AlbumAttributesExtension::ArtistUrl)
        .one(&client, "1676791755")
        .await?
        .expect("album fetch returned none");

    let attributes = album
        .attributes
        .expect("album fetch returned an album without attributes");

    assert_eq!(attributes.name, "Unrequited Love - EP");
    assert_eq!(
        attributes.artist_url,
        Some(String::from(
            "https://music.apple.com/us/artist/hkmori/1672126480"
        ))
    );

    Ok(())
}

#[tokio::test]
async fn fetch_album_relationship() -> Result<(), Error> {
    let client = common::create_client();

    let album = Album::get()
        .extend(AlbumAttributesExtension::ArtistUrl)
        .include(AlbumRelationshipType::Tracks)
        .one(&client, "1676791755")
        .await?
        .expect("album fetch returned none");

    let attributes = album
        .attributes
        .expect("album fetch returned an album without attributes");

    assert_eq!(attributes.name, "Unrequited Love - EP");
    assert_eq!(
        attributes.artist_url,
        Some(String::from(
            "https://music.apple.com/us/artist/hkmori/1672126480"
        ))
    );

    let tracks_relationships = album
        .relationships
        .tracks
        .expect("album fetch didn't return any track relationships");

    assert_eq!(tracks_relationships.data.len(), 6);

    Ok(())
}

#[tokio::test]
async fn fetch_album_view() -> Result<(), Error> {
    let client = common::create_client();

    let album = Album::get()
        .extend(AlbumAttributesExtension::ArtistUrl)
        .view(AlbumViewType::AppearsOn)
        .one(&client, "1651577210")
        .await?
        .expect("album fetch returned none");

    let attributes = album
        .attributes
        .expect("album fetch returned an album without attributes");

    assert_eq!(attributes.name, "Remember That You Will Die");
    assert_eq!(
        attributes.artist_url,
        Some(String::from(
            "https://music.apple.com/us/artist/polyphia/640294344"
        ))
    );

    let appears_on = album.views.appears_on;
    assert!(appears_on.is_some());

    Ok(())
}
