
use am_api::error::Error;
use am_api::resource::catalog::artist::Artist;


mod common;

#[tokio::test]
async fn fetch_artist() -> Result<(), Error> {
    let client = common::create_client();

    let artist = Artist::get()
        .one(&client, "1672126480")
        .await?
        .expect("artist fetch returned none");

    let attributes = artist
        .attributes
        .expect("artist fetch returned an artist without attributes");

    assert_eq!(attributes.name, "hkmori");
    Ok(())
}
