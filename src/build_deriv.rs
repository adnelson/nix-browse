use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
#[allow(dead_code)]
pub struct StorePath {
    hash: String,
    name: String,
}

impl StorePath {
    pub fn new(hashpart: String, name: String) -> Self {
        StorePath {hash: hashpart, name: name}
    }
}

/// Build a derivation
pub fn build_deriv(path: StorePath) -> Result<(), String> {
  Err(format!("{:?} not implemented!", path))
}


/// Given a path to a nix file and a possible attribute off of that
/// file, evaluate the attribute.
pub fn eval_nix_attr(filepath: &PathBuf, attr: Option<String>)
                 -> Vec<StorePath> {
    let s = filepath.as_path().as_os_str();
    let mut cmd = Command::new("nix-instantiate");
    cmd.arg("--eval").arg(s);
    match attr {
        None => {},
        Some(attr) => {cmd.arg(attr);}
    }
    let output = cmd.output().expect("failed to start nix-instantiate");
    let lines = String::from_utf8_lossy(&output.stdout);
    println!("Got lines:\n{}", lines);
    vec!()
// filepath.as_os_str());
//                      .args(&["-A", attr.as_slice()]);
}

// /// Represents an invocation of nix-instantiate.
// enum NixInstantiateAttr {
//     filepath: Path,
//     attributes: Vec<String>,
// }

// impl NixInstantiate
