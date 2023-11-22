use am_api::error::Error;
use am_api::resource::storefront::Storefront;

mod common;

#[tokio::test]
async fn fetch_us_storefront() -> Result<(), Error> {
    let client = common::create_client();

    let storefront = Storefront::get()
        .one(&client, celes::Country::the_united_states_of_america())
        .await?
        .expect("storefront fetch returned none");

    let attributes = storefront
        .attributes
        .expect("storefront fetch returned a storefront without attributes");

    assert!(attributes
        .supported_language_tags
        .contains(&String::from("en-US")));

    Ok(())
}
