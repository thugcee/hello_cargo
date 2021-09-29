extern crate pretty_env_logger;
extern crate log;

mod args;

use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() {
    pretty_env_logger::init();
    let args = args::parse_args();
    let files = args.values_of(args::FILE).unwrap().collect::<Vec<&str>>();
    for filename in files {
        let reader: Box<dyn io::BufRead> = match filename {
            "-" => {
                Box::new(BufReader::new(io::stdin()))
            },
            _ => match File::open(&filename) {
                    Ok(handler) => Box::new(BufReader::new(handler)),
                    Err(_) => {log::error!("no such file: {}", filename); continue},
                },
        };
        let lines = reader.lines()
            .map(|l| l.expect("INVALID LINE"))
            .collect::<Vec<String>>();
        process(filename, lines);
    }
}

fn process(filename: &str, lines: Vec<String>) {
    log::debug!("{}: {}", filename, lines.len());
    if lines[2].contains("system {") || lines[2].contains("groups {") {
        log::info!("detected Junos config");
    }
}