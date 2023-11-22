
use am_api::error::Error;
use am_api::primitive::AudioVariant;
use am_api::resource::catalog::song::{Song, SongAttributesExtension};

use am_api::time::year_or_date::YearOrDate;
use time::{Date, Month};

mod common;

#[tokio::test]
async fn fetch_song() -> Result<(), Error> {
    let client = common::create_client();

    let song = Song::get()
        .extend(SongAttributesExtension::ArtistUrl)
        .extend(SongAttributesExtension::AudioVariants)
        .one(&client, "1416240728")
        .await?
        .expect("song fetch returned none");

    let attributes = song
        .attributes
        .expect("song fetch returned a song without attributes");

    assert_eq!(
        attributes.release_date,
        Some(YearOrDate::Date(
            Date::from_calendar_date(2018, Month::October, 12).unwrap()
        ))
    );
    assert_eq!(attributes.name, "So Strange (feat. Cuco)");
    assert_eq!(
        attributes.artist_url,
        Some(String::from(
            "https://music.apple.com/us/artist/polyphia/640294344"
        ))
    );

    assert!(attributes.audio_variants.is_some());
    assert!(attributes
        .audio_variants
        .as_ref()
        .unwrap()
        .contains(&AudioVariant::LossyStereo));
    assert!(attributes
        .audio_variants
        .as_ref()
        .unwrap()
        .contains(&AudioVariant::Lossless));

    Ok(())
}
