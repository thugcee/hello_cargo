extern crate clap;

use clap::{App, Arg};

pub const FILE: &str = "file";

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    return App::new("Net Conf Flattener")
        .version("0.1.0")
        .author("Seweryn Niemiec <sew@eioki.eu>")
        .arg(Arg::with_name(FILE).multiple(true).default_value("-"))
        .get_matches();
}