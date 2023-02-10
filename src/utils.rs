use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub fn parse_date<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%z").map_err(de::Error::custom)
}

// serialize naive date time
pub fn serialize_date<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%dT%H:%M:%S%z").to_string();
    serializer.serialize_str(&s)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Measure {
    #[serde(deserialize_with = "parse_date", serialize_with = "serialize_date")]
    pub instante: DateTime<FixedOffset>,
    pub carga: f32,
}

