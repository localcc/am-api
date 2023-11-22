use am_api::error::Error;
use am_api::resource::catalog::song::Song;
use am_api::resource::library::LibraryAddResourceBuilder;

mod common;

#[tokio::test]
pub async fn add_song() -> Result<(), Error> {
    let client = common::create_client();

    let song = Song::get()
        .one(&client, "1416240728")
        .await?
        .expect("song fetch returned none");

    LibraryAddResourceBuilder::new()
        .add_resource(&song.into())?
        .send(&client)
        .await?;

    Ok(())
}
