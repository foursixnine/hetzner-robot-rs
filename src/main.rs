mod client;
mod hetzner;
use clap::{Parser, Subcommand};
use client::*;
use hetzner::*;
use std::env;
use std::fmt::{self, Debug, Formatter};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Zones {
        #[arg(short, long)]
        query: bool,
    },
}

impl Debug for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Test { list } => write!(f, "Test {{ list: {} }}", list),
            Commands::Zones { query } => write!(f, "Zones {{ query: {} }}", query),
        }
    }
}

impl Clone for Commands {
    fn clone(&self) -> Self {
        match self {
            Commands::Test { list } => Commands::Test { list: *list },
            Commands::Zones { query } => Commands::Zones { query: *query },
        }
    }
}

fn main() {
    let arguments = Cli::parse();
    if cfg!(debug_assertions) {
        // TODO: add a command line argument parser
        // main should parse cli arguments using getopts
        // or its rust equivalent
        // The popular crate for this is clap

        // TODO: replace dbg with debug! from log crate
        dbg!("args: {:?}", &arguments);
    }

    let client_instance = client::HetznerClient {
        ..Default::default()
    };
    // this could be changed in the future when multiple commands are supported
    // it could be abuilder pattern like:
    // let client_instance = HetznerClient::default().with_command("zones");
    // another option would be to have a separate struct for each command
    // and then have a function that takes a command struct and returns the
    // response object
    // let client_instance = HetznerClient::default().with_command(ZonesCommand);
    // let client_instance = HetznerClient::default().with_command(RecordsCommand);

    match arguments.command {
        Some(Commands::Zones { query }) => {
            if query {
                let zones = query_zones(client_instance);
                display_zones(zones);
            } else {
                let _ = std::io::stdout().write_all(b"No query to be performed");
            }
        }
        _ => {
            dbg!("args: {:?}", arguments);
        }
    }
}

fn display_zones(response_object: VecZoneRecord) {
    let zones = response_object.zones;
    let mut stdout = std::io::stdout();
    for zone in zones {
        let zone_name = zone.name;
        let zone_id = zone.id;
        // we need to wrap the arguments to write_all in a byte string
        stdout
            .write_all(format!("Zone name: {}, Zone ID: {}\n", zone_name, zone_id).as_bytes())
            .unwrap_or_else(|_| panic!("failed to write zone_id {} to stdout", zone_id));
    }
    stdout.write_all(b"\n").expect("failed to write to stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_zones_no_records() {
        let response_object = VecZoneRecord { zones: vec![] };
        assert_eq!(display_zones(response_object), ());
    }
}
