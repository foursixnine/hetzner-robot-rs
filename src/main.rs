mod client;
mod hetzner;
use client::*;
use hetzner::*;
use std::io::Write;

fn main() {
    let client_instance = HetznerClient::default();
    let zones = query_zones(client_instance);
    process_zones(zones);
}

fn process_zones(response_object: VecZoneRecord) {
    let zones = response_object.zones;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for zone in zones {
        let zone_name = zone.name;
        let zone_id = zone.id;
        write!(
            &mut handle,
            "zone_id {}, \tzone_name {}",
            zone_id, zone_name
        )
        .expect("failed to write to stdout");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_zones_no_records() {
        let response_object = VecZoneRecord { zones: vec![] };
        // process_zones returns nothing, so we can't assert anything
        // however we can verify that the function doesn't panic
        // and that it prints something to stdout
        
        
    }

    #[test]
    fn test_process_zones_one_record() {
        let response_object = VecZoneRecord {
            zones: vec![ZoneRecord {
                name: String::from("example.com"),
                id: 1,
            }],
        };
        process_zones(response_object);
        // Add assertions here to verify the expected behavior
    }

    #[test]
    fn test_process_zones_hundred_records() {
        let mut zones = vec![];
        for i in 1..=100 {
            zones.push(ZoneRecord {
                name: format!("example{}.com", i),
                id: i,
            });
        }
        let response_object = VecZoneRecord { zones };
        process_zones(response_object);
        // Add assertions here to verify the expected behavior
    }
}
