
#[macro_use]

extern crate clap;
use std::env;
use std::path::PathBuf;
use clap::{Arg, App, SubCommand};
use std::net::Ipv4Addr;

fn parse_args() -> Args {
    let progname = "curlover";
    let matches = App::new(progname)
        .version("0.1")
        .about("Tiny server for recieveing files over curl")
        .author("Eden Z.")
        .arg(Arg::with_name("address")
             .short("a")
             .long("address")
             .value_name("ADDRESS")
             .help("Address to bind to")
             .takes_value(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .value_name("PORT")
             .help("Port to run on")
             .takes_value(true))
        .arg(Arg::with_name("root")
             .short("d")
             .long("DIRECTORY")
             .value_name("directory")
             .help("Base directory to recieve files")
             .takes_value(true))
        .get_matches();

    let address = value_t!(matches.value_of("address"), Ipv4Addr).unwrap_or(Ipv4Addr::new(127, 0, 0, 1));

    let port = value_t!(matches.value_of("port"), u16).unwrap_or(8080);

    let root = match matches.value_of("root") {
        Some(v) => PathBuf::from(v),
        None => env::current_dir().unwrap(),
    };

    assert_eq!(root.is_dir(), true);

    return Args {port: port, address: address, root: root}


}

struct Args {
    port: u16,
    address: Ipv4Addr,
    root: PathBuf,
}
