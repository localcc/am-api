//! Year or date

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::Date;

pub(crate) const FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

/// Year or date
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YearOrDate {
    /// Year
    Year(i16),
    /// Date
    Date(Date),
}

impl Display for YearOrDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YearOrDate::Year(year) => write!(f, "{:4}", year),
            YearOrDate::Date(e) => Display::fmt(e, f),
        }
    }
}

impl Serialize for YearOrDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            YearOrDate::Year(year) => serializer.serialize_str(&year.to_string()),
            YearOrDate::Date(date) => Serialize::serialize(date, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for YearOrDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        Ok(match s.contains('-') {
            false => YearOrDate::Year(i16::from_str(s).map_err(serde::de::Error::custom)?),
            true => YearOrDate::Date(Date::parse(s, &FORMAT).map_err(serde::de::Error::custom)?),
        })
    }
}
