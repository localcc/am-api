use am_api::ApiClient;
use std::env;

#[allow(dead_code)]
pub fn create_client() -> ApiClient {
    let developer_token =
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be defined for tests");
    let media_user_token =
        env::var("MEDIA_USER_TOKEN").expect("MEDIA_USER_TOKEN must be defined for tests");
    ApiClient::new(
        &developer_token,
        &media_user_token,
        celes::Country::the_united_states_of_america(),
    )
    .expect("failed to create api client")
}
