
use am_api::error::Error;
use am_api::resource::genre::Genre;


mod common;

#[tokio::test]
async fn fetch_genre() -> Result<(), Error> {
    let client = common::create_client();

    let genre = Genre::get()
        .one(&client, "21")
        .await?
        .expect("genre fetch returned none");

    let attributes = genre
        .attributes
        .expect("genre fetch returned a genre without attributes");

    assert_eq!(attributes.name, "Rock");
    assert_eq!(attributes.parent_name, Some(String::from("Music")));

    Ok(())
}
