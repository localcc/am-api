
use am_api::error::Error;
use am_api::resource::catalog::curator::{AppleCurator, Curator, CuratorKind};


mod common;

#[tokio::test]
async fn fetch_curator() -> Result<(), Error> {
    let client = common::create_client();

    let curator = Curator::get()
        .one(&client, "1299820965")
        .await?
        .expect("curator fetch returned none");

    let attributes = curator
        .attributes
        .expect("curator fetch returned a curator without attributes");

    assert_eq!(attributes.name, "ATMA Classique");

    Ok(())
}

#[tokio::test]
async fn fetch_apple_curator() -> Result<(), Error> {
    let client = common::create_client();

    let curator = AppleCurator::get()
        .one(&client, "993270836")
        .await?
        .expect("apple curator fetch returned none");

    let attributes = curator
        .attributes
        .expect("apple curator fetch returned a curator without attributes");

    assert_eq!(attributes.kind, CuratorKind::Show);
    assert_eq!(attributes.name, "Abstract Radio");

    Ok(())
}
