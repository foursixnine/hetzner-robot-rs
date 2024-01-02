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
    let mut stdout = std::io::stdout();
    for zone in zones {
        let zone_name = zone.name;
        let zone_id = zone.id;
        // we need to wrap the arguments to write_all in a byte string
        stdout.write(
            format!("Zone name: {}, Zone ID: {}\n", zone_name, zone_id).as_bytes(),
        )
        .expect(format!("failed to write zone_id {} to stdout", zone_id).as_str());
    }
    stdout.write_all(b"\n").expect("failed to write to stdout");
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_zones_no_records() {
        let response_object = VecZoneRecord { zones: vec![] };
        assert_eq!(process_zones(response_object), ());
    }
}
