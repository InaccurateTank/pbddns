//! API command objects.

use std::{borrow::Cow, net::{IpAddr, Ipv4Addr, Ipv6Addr}};
use serde::Serialize;
use crate::{ApiCommand, DnsTypes};

/// Standard payload for the creation or editing of a single [DnsRecord][crate::responses::DnsRecord].
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct CreateOrEditRecord<'a> {
	/// The subdomain for the record being created, not including the domain itself. Leave blank to create a record on the root domain. Use * to create a wildcard record.
	#[serde(rename = "name")]
	pub subdomain: Option<&'a str>,
	/// The type of record being created. See [DnsTypes] for all valid types.
	#[serde(rename = "type")]
	pub record_type: DnsTypes,
	/// The answer content for the record. Please see the DNS management popup from the domain management console for proper formatting of each record type.
	pub content: Cow<'a, str>,
	/// Optional. The time to live in seconds for the record. The minimum and the default is 600 seconds.
	pub ttl: Option<u64>,
	/// Optional. The priority of the record for those that support it.
	pub prio: Option<u32>
}
impl<'a> CreateOrEditRecord<'a> {
	/// Creates a new instance of [CreateOrEditRecord] for use as a payload.
	pub fn new(
		subdomain: Option<&'a str>,
		record_type: DnsTypes,
		content: impl Into<Cow<'a, str>>
	) -> Self {
		Self {
			subdomain,
			record_type,
			content: content.into(),
			ttl: None,
			prio: None
		}
	}

	/// Adds a priority to the record.
	pub fn with_priority(self, prio: Option<u32>) -> Self {
		Self { prio, ..self }
	}

	/// Adds a ttl to the record. The mimimum and default is 600.
	pub fn with_ttl(self, ttl: u64) -> Self {
		Self { ttl: Some(ttl), ..self}
	}

	/// Shorthand for using [CreateOrEditRecord::new()] with the type [DnsTypes::A].
	///
	/// Verifies that the contents are an actual [Ipv4Addr].
	///
	/// # Example
	/// ```
	/// use std::net::Ipv4Addr;
	/// use pb_api::{commands::CreateOrEditRecord, DnsTypes};
	///
	/// let ip = Ipv4Addr::new(127, 0, 0, 1);
	///
	/// let a = CreateOrEditRecord::new(None, DnsTypes::A, "127.0.0.1");
	/// let b = CreateOrEditRecord::new_a(None, ip);
	/// assert_eq!(a, b);
	/// ```
	pub fn new_a(
		subdomain: Option<&'a str>,
		ip: Ipv4Addr
	) -> Self {
		Self::new(subdomain, DnsTypes::A, Cow::Owned(ip.to_string()))
	}

	/// Shorthand for using [CreateOrEditRecord::new()] with the type [DnsTypes::AAAA].
	///
	/// Verifies that the contents are an actual [Ipv6Addr].
	///
	/// # Example
	/// ```
	/// use std::net::Ipv6Addr;
	/// use pb_api::{commands::CreateOrEditRecord, DnsTypes};
	///
	/// let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
	///
	/// let a = CreateOrEditRecord::new(None, DnsTypes::AAAA, "::1");
	/// let b = CreateOrEditRecord::new_aaaa(None, ip);
	/// assert_eq!(a, b);
	/// ```
	pub fn new_aaaa(
		subdomain: Option<&'a str>,
		ip: Ipv6Addr
	) -> Self {
		Self::new(subdomain, DnsTypes::AAAA, Cow::Owned(ip.to_string()))
	}


	/// Shorthand for using [CreateOrEditRecord::new()] with either the type [DnsTypes::A] or [DnsTypes::AAAA].
	///
	/// Takes a generic ip and matches it with the correct builder.
	///
	/// # Example
	/// ```
	/// use std::net::{IpAddr, Ipv6Addr};
	/// use pb_api::{commands::CreateOrEditRecord, DnsTypes};
	///
	/// let v6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
	/// let ip = IpAddr::from(v6);
	///
	/// let a = CreateOrEditRecord::new_aaaa(None, v6);
	/// let b = CreateOrEditRecord::new_a_or_aaaa(None, ip);
	/// assert_eq!(a, b);
	/// ```
	pub fn new_a_or_aaaa(
		subdomain: Option<&'a str>,
		ip: IpAddr
	) -> Self {
		match ip {
			IpAddr::V4(ip) => Self::new_a(subdomain, ip),
			IpAddr::V6(ip) => Self::new_aaaa(subdomain, ip)
		}
	}
}
impl<'a> ApiCommand for CreateOrEditRecord<'a> {}
