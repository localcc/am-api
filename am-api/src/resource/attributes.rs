//! Common attributes

use serde::{Deserialize, Serialize};

/// Description attribute
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct DescriptionAttribute {
    /// Short length description text
    pub short: Option<String>,
    /// Standard length description text
    pub standard: String,
}

/// Title only attribute
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct TitleOnlyAttribute {
    /// A localized title to display for the view
    pub title: String,
}
