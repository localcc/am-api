
use am_api::error::Error;
use am_api::resource::catalog::station::{Station, StationGenre};


mod common;

#[tokio::test]
async fn fetch_station() -> Result<(), Error> {
    let client = common::create_client();

    let station = Station::get()
        .one(&client, "ra.1569482000")
        .await?
        .expect("station fetch returned none");

    let attributes = station
        .attributes
        .expect("station fetch returned a station without attributes");

    assert_eq!(attributes.name, "Lo-Fi Station");

    Ok(())
}

#[tokio::test]
async fn fetch_genre() -> Result<(), Error> {
    let client = common::create_client();

    let genre = StationGenre::get()
        .one(&client, "1149486365")
        .await?
        .expect("station genre fetch returned none");

    let attributes = genre
        .attributes
        .expect("station genre fetch returned a genre without attributes");

    assert_eq!(attributes.name, "Rock");

    Ok(())
}
