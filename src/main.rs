#[macro_use] extern crate lazy_static;
extern crate regex;

use std::path::PathBuf;
use regex::Regex;

pub mod build_deriv;
pub mod parse_nix_instantiate;
use parse_nix_instantiate::NIX_INSTANTIATE_OUTPUT_RE;

use build_deriv::eval_nix_attr;

fn main() {
    // let s: String = "farts".to_string();
    // let s1: &str = &s[1..(s.len() - 1)];
    // let s2: String = String::from(s1);
    // println!("Hello, world {}!", s1);
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
    let re = Regex::new(r"(?x)
      (?P<y>\d{4}) # the year
      -
      (?P<m>\d{2}) # the month
      -
      (?P<d>\d{2}) # the day
    ").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");

    let text = r#"[2012 3 14 -123 {x = "hey \"yo\" sup"; zzz = true; y = <CODE>}]"#;
    for cap in NIX_INSTANTIATE_OUTPUT_RE.captures_iter(text) {
        println!("Cap: {}", &cap[0]);
    }

    // let path = PathBuf::from("/home/anelson/test.nix");
    // eval_nix_attr(&path, None);
}
