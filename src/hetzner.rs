use serde::{self, Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct VecZoneRecord {
    pub zones: Vec<ZoneRecord>,
}

impl VecZoneRecord {
    pub fn new() -> Self {
        VecZoneRecord { zones: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneRecord {
    pub id: String,
    created: String,
    modified: String,
    legacy_dns_host: String,
    legacy_ns: Vec<String>,
    pub name: String,
    ns: Vec<String>,
    owner: String,
    paused: bool,
    permission: String,
    project: String,
    registrar: String,
    status: String,
    ttl: u64,
    verified: String,
    records_count: u64,
    is_secondary_dns: bool,
    txt_verification: TxtVerification,
}

// we need to create a struct to represent the txt_verification field
#[derive(Debug, Serialize, Deserialize)]
pub struct TxtVerification {
    name: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    page: u64,
    per_page: u64,
    last_page: u64,
    total_entries: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordsResponse {
    meta: Pagination,
    pub zones: Vec<ZoneRecord>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize)]
pub enum RecordTypes {
    A,
    AAAA,
    NS,
    MX,
    CNAME,
    RP,
    TXT,
    SOA,
    HINFO,
    SRV,
    DANE,
    TLSA,
    DS,
    CAA,
}

// Implament Display for RecordTypes so we can print the record type
// and pass it to the API when needed
// https://doc.rust-lang.org/std/fmt/trait.Display.html
// https://kerkour.com/rust-enum-to-string
impl fmt::Display for RecordTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RecordTypes::A => write!(f, "A"),
            RecordTypes::AAAA => write!(f, "AAAA"),
            RecordTypes::NS => write!(f, "NS"),
            RecordTypes::MX => write!(f, "MX"),
            RecordTypes::CNAME => write!(f, "CNAME"),
            RecordTypes::RP => write!(f, "RP"),
            RecordTypes::TXT => write!(f, "TXT"),
            RecordTypes::SOA => write!(f, "SOA"),
            RecordTypes::HINFO => write!(f, "HINFO"),
            RecordTypes::SRV => write!(f, "SRV"),
            RecordTypes::DANE => write!(f, "DANE"),
            RecordTypes::TLSA => write!(f, "TLSA"),
            RecordTypes::DS => write!(f, "DS"),
            RecordTypes::CAA => write!(f, "CAA"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordType {
    pub record_type: RecordTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub zone_id: String,
    pub name: String,
    pub record_type: RecordType,
    pub value: String,
    pub ttl: u64,
    pub created: String,
    pub modified: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordResponse {
    pub record: Vec<Record>,
}
