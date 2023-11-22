use am_api::error::Error;
use am_api::resource::catalog::song::Song;
use am_api::resource::library::playlist::LibraryPlaylist;

mod common;

#[tokio::test]
async fn create_playlist() -> Result<(), Error> {
    let client = common::create_client();

    let song = Song::get()
        .one(&client, "1676792026")
        .await?
        .expect("song fetch returned none");

    LibraryPlaylist::create("test_playlist")
        .public(false)
        .description("Test Description")
        .tracks(&[&song.into()])?
        .create(&client)
        .await?;

    Ok(())
}

#[tokio::test]
async fn add_song() -> Result<(), Error> {
    let client = common::create_client();

    let song = Song::get()
        .one(&client, "1676792026")
        .await?
        .expect("song fetch returned none");

    let playlist = LibraryPlaylist::create("test_add_playlist")
        .public(false)
        .description("Test Description")
        .create(&client)
        .await?
        .expect("library playlist create returned none");

    playlist.add_tracks(&client, &[&song.into()]).await?;

    Ok(())
}
