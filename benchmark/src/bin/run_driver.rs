#[macro_use]
extern crate log;

use benchmark::driver::run_worker;
use benchmark::scenario;
use clap::{App, Arg};

fn main() {
    // initialization
    env_logger::init();
    let matches = App::new("Test Controller")
        .arg(
            Arg::with_name("Address")
                .long("addr")
                .help("The workers to connect")
                .min_values(2),
        )
        .get_matches();
    let addrs: Vec<String> = if let Some(addr_matches) = matches.values_of("Address") {
        addr_matches.map(String::from).collect()
    } else {
        vec![]
    };

    info!("{:?}", addrs);
    // run test
    run_worker(scenario::async_stream_8channel_320rpcs_64kb(), addrs.clone());
    info!("Control Driver Shutdown");
}
