#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate regex;
extern crate clap;

use std::env::current_dir;
use std::path::PathBuf;

use clap::{App, Arg};
use regex::Regex;

pub mod parse_nix_instantiate;
use parse_nix_instantiate::exec_nix_instantiate;

fn main() {
    let cwd = current_dir().unwrap();
    let matches = App::new("eval-nix")
        .arg(Arg::with_name("PATH")
             .default_value(cwd.to_str().unwrap()))
        .arg(Arg::with_name("ATTRIBUTE"))
        .arg(Arg::with_name("EXPR_ARGS")
             .short("a")
             .long("arg")
             .number_of_values(2)
             .multiple(true))
	.get_matches();

    let path = PathBuf::from(matches.value_of("PATH").unwrap());
    let attribute = matches.value_of("ATTRIBUTE").map(String::from);
    let mut expr_args = vec!();
    // Iterate through pairs of expression arguments, turn them into a
    // vector of tuples.
    match matches.values_of("EXPR_ARGS") {
        None => {},
        Some(_args) => {
            let mut o_key = None;
            for item in _args {
                match o_key {
                    None => {
                        o_key = Some(item);
                    },
                    Some(key) => {
                        o_key = None;
                        expr_args.push((String::from(key),
                                        String::from(item)))
                    },
                }
            }
        },
    }
    let evald = exec_nix_instantiate(&path, attribute, &expr_args);
    println!("{}", serde_json::to_string(&evald).unwrap());
}
