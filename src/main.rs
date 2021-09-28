mod args;

use std::fs;

fn main() {
    let args = args::parse_args();
    let files = args.values_of(args::FILE).unwrap().collect::<Vec<&str>>();
    for file in files {
        println!("Arg file: {}", &file);
    }
}