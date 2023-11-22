# am-api

Apple Music API bindings for rust.

# Examples

Fetching an album by id

```rust
async fn fetch_album() -> Result<(), Error> {
    let developer_token = "DEVELOPER_TOKEN";
    let media_user_token = "MEDIA_USER_TOKEN";
    
    let client = ApiClient::new(
        developer_token, 
        media_user_token, 
        celes::Country::the_united_states_of_america()
    )
    .expect("failed to create api client");
    
    let album = Album::get()
        .one(&client, "1676791755")
        .await?
        .expect("album fetch returned none");

    let attributes = album
        .attributes
        .expect("album fetch returned an album without attributes");

    assert_eq!(attributes.name, "Unrequited Love - EP");

    Ok(())
}
```

More examples can be found in the [examples](https://github.com/localcc/am-api/tree/main/am-api/examples) folder.

# Installation

To add this library to your project use

```
cargo add am-api
```

## Features

* `rustls-tls` pure rust tls implementation **(enabled by default)**
* `native-tls` native platform tls implementation


