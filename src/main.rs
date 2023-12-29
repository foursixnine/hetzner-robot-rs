use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use reqwest::redirect;
use reqwest::Url;
use serde::{self, Deserialize, Serialize};
use std::env;
use std::time::Duration;
use std::vec::Vec;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    // this will set the user agent to the following:
    // hetzner-robot-rs/0.1.0
);

// we need to create a struct to hold the client
struct HetznerClient {
    api_url_base: Url,
    client: reqwest::blocking::Client,
}

impl HetznerClient {
    #[allow(dead_code)]
    fn new(api_url_base: Url) -> Self {
        Self {
            api_url_base,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Default for HetznerClient {
    fn default() -> Self {
        Self {
            api_url_base: Url::parse("https://dns.hetzner.com/api/v1/").unwrap(),
            client: _setup_client(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VecZoneRecord {
    zones: Vec<ZoneRecord>,
}

impl VecZoneRecord {
    fn new() -> Self {
        VecZoneRecord { zones: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ZoneRecord {
    id: String,
    created: String,
    modified: String,
    legacy_dns_host: String,
    legacy_ns: Vec<String>,
    name: String,
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
struct TxtVerification {
    name: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    page: u64,
    per_page: u64,
    last_page: u64,
    total_entries: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecordsResponse {
    meta: Pagination,
    zones: Vec<ZoneRecord>,
}

fn main() {
    let client_instance = HetznerClient::default();

    let res = query(client_instance);
    let zones = match res {
        Ok(r) => {
            // now that we a response with the zones, we need to extract those zones
            // and return the responseObject using the vec of ZoneRecord

            // code above needs to be rewritten to use the impl From trait
            // using the From trait we can convert from serde_json::Value to Vec<ZoneRecord>
            // we can only do this if r.status().is_success()
            let mut response_vector = VecZoneRecord::new();
            match r.status().is_success() {
                true => {
                    response_vector = r.json().unwrap();
                    println!("{:?}", response_vector);
                    response_vector
                }
                false => {
                    // we need to get the error message from the response
                    // and check if we're being rate limited
                    // if we are being rate limited, we need to wait for the rate limit to expire
                    // and then retry the request
                    // if we're not being rate limited, we need to print the error message
                    // and exit the program
                    let rate_limit_remaining = r.headers().get("RateLimit-Remaining").unwrap();
                    let rate_limit_reset = r.headers().get("RateLimit-Reset").unwrap();
                    println!(
                        "rate_limit_remaining: {:?}, rate_limit_reset: {:?}",
                        rate_limit_remaining, rate_limit_reset
                    );
                    // we need to wait for the rate limit to reset and let the user know
                    // when the rate limit time is up
                    if rate_limit_remaining == "0" {
                        handle_rate_limit(rate_limit_reset);
                    }
                    println!("{:?}", r.status());
                    // we need to return an empty VecZoneRecord
                    response_vector
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("We have an error");
        }
    };

    process_response_object(zones);
}

fn handle_rate_limit(rate_limit_reset: &reqwest::header::HeaderValue) {
    let wait_time = rate_limit_reset.to_str().unwrap().parse().unwrap();
    println!(
        "rate limit is 0, waiting {0} seconds for rate limit reset",
        wait_time
    );
    std::thread::sleep(Duration::from_secs(wait_time));
}

fn process_response_object(response_object: VecZoneRecord) {
    let zones = response_object.zones;
    for zone in zones {
        let zone_name = zone.name;
        let zone_id = zone.id;
        println!("zone_id {}, \tzone_name {}", zone_id, zone_name);
    }
}

// Now that we have a zone id we can get the records for that zone
// https://dns.hetzner.com/api/v1/records?zone_id=123456
// the following code generated by copilot is obviously not correct
// however it can serve as a starting point for the correct implementation
// of the get_records_for_zone function
//
// but first we need to refactor the client creation code into its own struct
// along with the needed functions and impl blocks
//fn get_records_for_zone(client ) -> Result<Response, reqwest::Error> {
//    self.client.get(self.api_url_base.join("records").unwrap())
//        .query(&[("zone_id", self.zone_id)])
//        .send()
//}

// this function is used to generate a custom redirect policy for the client
fn generate_redirect_policy() -> redirect::Policy {
    redirect::Policy::custom(|attempt| {
        if attempt.previous().len() > 5 {
            let url = attempt.url().to_string();
            attempt.error(format!("too many redirects to {:?}", url))
        } else if attempt.url().host_str() == Some("example.domain") {
            // prevent redirects to 'example.domain'
            attempt.stop()
        } else {
            attempt.follow()
        }
    })
}

fn query(client: HetznerClient) -> Result<Response, reqwest::Error> {
    // we need to get the client result, as it should be treated as a blocking operation
    // execute the client call and return the response to the caller
    client
        .client
        .get(client.api_url_base.join("zones").unwrap())
        .send()
}

fn _setup_client() -> reqwest::blocking::Client {
    let custom_redirect_policy = generate_redirect_policy();
    let mut headers = HeaderMap::new();
    headers.insert("Auth-API-Token", _get_api_key().parse().unwrap());
    headers.insert("ACCEPT", "application/json".parse().unwrap());
    headers.insert("User-Agent", APP_USER_AGENT.parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers) // set the default headers for all requests
        .redirect(custom_redirect_policy) // set the max redirects for the client
        .timeout(Duration::from_secs(10)) // we can also set the timeout for the client
        .https_only(true)
        .build();
    match client {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            panic!("We have an error");
        }
    }
}

fn _get_api_key() -> String {
    match std::env::var("HETZNER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            panic!("Please set the HETZNER_API_KEY environment variable");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_key() {
        // Set up
        std::env::set_var("HETZNER_API_KEY", "test_api_key");
        // Execute
        let api_key = _get_api_key();

        // Verify
        assert_eq!(api_key, "test_api_key");

        // verify that an error is returned if the HETZNER_API_KEY
        // environment variable is not set
        std::env::remove_var("HETZNER_API_KEY");
        assert!(std::panic::catch_unwind(|| {
            let _ = _get_api_key();
        })
        .is_err());
    }
}
