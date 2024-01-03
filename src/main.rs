mod client;
mod hetzner;
use client::*;
use hetzner::*;
use std::io::Write;

fn main() {

    if cfg!(debug_assertions) {
        // TODO: add a command line argument parser
        // main should parse cli arguments using getopts
        // or its rust equivalent

        use std::env;
        // TODO: replace dbg with debug! from log crate
        let arguments: Vec<String> = env::args().collect();
        dbg!("args: {:?}", arguments.clone());

        // Prints each argument on a separate line
        arguments.iter().for_each(|argument| {
            println!("{}", argument);
        });
    }

    let client_instance = HetznerClient::default();
    // this could be changed in the future when multiple commands are supported
    // it could be abuilder pattern like:
    // let client_instance = HetznerClient::default().with_command("zones");
    // another option would be to have a separate struct for each command
    // and then have a function that takes a command struct and returns the
    // response object
    // let client_instance = HetznerClient::default().with_command(ZonesCommand);
    // let client_instance = HetznerClient::default().with_command(RecordsCommand);
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
