use serde::de::{Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>
{
	let buf = Option::<String>::deserialize(deserializer)?;
	match buf {
		Some(s) => u64::from_str(&s).map_err(serde::de::Error::custom),
		None => Ok(0)
	}
}

pub(crate) fn string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>
{
	let buf = Option::<String>::deserialize(deserializer)?;
	match buf {
		Some(s) => u32::from_str(&s).map_err(serde::de::Error::custom),
		None => Ok(0)
	}
}
