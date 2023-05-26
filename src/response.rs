use serde::{Deserialize};

// Error Response
#[derive(Debug, Deserialize)]
pub struct ErrorRes {
	pub message: String
}

// Ping Response
#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum Ping {
	#[serde(rename = "SUCCESS")]
	Success {
		#[serde(rename = "yourIp")]
		your_ip: String
	},
	#[serde(rename = "ERROR")]
	Error(ErrorRes)
}

// Record Response
#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum Records {
	#[serde(rename = "SUCCESS")]
	Success { records: Vec<Record> },
	#[serde(rename = "ERROR")]
	Error(ErrorRes)
}

#[derive(Debug, Deserialize)]
pub struct Record {
  pub id: String,
	pub name: String,
	#[serde(rename = "type")]
	pub rec_type: String,
	pub content: String,
	pub ttl: Option<String>,
	pub prio: Option<String>,
	pub notes: Option<String>
}

// Edit Response
#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum Edit {
	#[serde(rename = "SUCCESS")]
	Success,
	#[serde(rename = "ERROR")]
	Error(ErrorRes)
}
