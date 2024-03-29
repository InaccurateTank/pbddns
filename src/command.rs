use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Key {
	pub apikey: String,
	pub secretapikey: String
}

#[derive(Debug, Serialize)]
pub struct Edit {
	pub apikey: String,
	pub secretapikey: String,
	pub name: String,
	#[serde(rename = "type")]
	pub rec_type: String,
	pub content: String,
	pub ttl: String,
	pub prio: Option<String>
}
