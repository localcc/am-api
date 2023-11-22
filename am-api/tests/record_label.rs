
use am_api::error::Error;
use am_api::resource::catalog::record_label::RecordLabel;


mod common;

#[tokio::test]
async fn fetch_label() -> Result<(), Error> {
    let client = common::create_client();

    let label = RecordLabel::get()
        .one(&client, "1543990853")
        .await?
        .expect("record label fetch returned none");

    let attributes = label
        .attributes
        .expect("record label fetch returned a label without attributes");

    assert_eq!(attributes.name, "Ninja Tune");

    Ok(())
}
