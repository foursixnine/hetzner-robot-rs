use std::time::Duration;
use reqwest::blocking::Response;


mod hetzner;
use hetzner::*;

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
                    response_vector = r.json().expect("failed to deserialize from API");
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

fn query(client: HetznerClient) -> Result<Response, reqwest::Error> {
    // we need to get the client result, as it should be treated as a blocking operation
    // execute the client call and return the response to the caller
    client
        .client
        .get(client.api_url_base.join("zones").unwrap())
        .send()
}
