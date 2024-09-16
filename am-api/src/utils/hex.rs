//! Serde hex color deserialization

use serde::{Deserialize, Deserializer, Serializer};

pub(crate) mod option {
    use super::*;
    use serde::Serialize;

    pub(crate) fn serialize<S>(color: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        color.map(|e| format!("{:06x}", e)).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;

        s.map(|e| u32::from_str_radix(&e, 16))
            .transpose()
            .map_err(serde::de::Error::custom)
    }
}
