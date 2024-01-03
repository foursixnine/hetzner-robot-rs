use serde::{self, Deserialize, Serialize};

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

