use std::path::PathBuf;

pub mod build_deriv;

use build_deriv::eval_nix_attr;

fn main() {
    println!("Hello, world!");
    let path = PathBuf::from("/home/anelson/test.nix");
    eval_nix_attr(&path, None);
}
