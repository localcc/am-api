use am_api::error::Error;
use am_api::resource::catalog::search::{CatalogSearch, CatalogSearchType};
use futures::{pin_mut, StreamExt};

mod common;

#[tokio::test]
async fn search_catalog() -> Result<(), Error> {
    let client = common::create_client();

    let results = CatalogSearch::search()
        .search(&client, &[CatalogSearchType::Albums], "Unrequited Love")
        .await?;

    let albums = results.albums.iter(&client);
    pin_mut!(albums);

    while let Some(album) = albums.next().await {
        let album = album?;
        let attributes = album
            .attributes
            .expect("search returned an album without attributes");

        if attributes.artist_name == "hkmori" {
            return Ok(());
        }
    }

    panic!("expected album not found");
}
