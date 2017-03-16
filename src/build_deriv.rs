use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

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


/// A rust embedding of a nix derivation.
/// Derivations are how nix represents yet-to-be-built objects.
#[derive(Debug)]
#[allow(dead_code)]
struct Derivation {
    /// Outputs the derivation is expected to produce and what they're
    /// called. Those outputs might have known hashes (fixed-output
    /// derivations); if so include those.
    outputs: HashMap<String, String /* should be StorePath */>,

    /// System the derivation is to be built on.
    system: String,

    /// Path to the executable to build the derivation.
    builder: String, // should be StorePath,

    /// Arguments to the builder.
    build_args: Vec<String>,

    /// Environment to run the builder in.
    environment: HashMap<String, String>,

    /// Non-derivation inputs the derivation needs in order to build
    /// (paths that were copied from the file system to the store)
    input_files: Vec<String /* should be StorePath */>,

    /// Derivations this derivation needs to have as inputs, and
    /// outputs of those derivations.
    input_derivations: HashMap<String /* should be StorePath */, Vec<String>>,
}

/// A type representing nix values which can be parsed from the output
/// of nix-instantiate. This might be seen as a subset of all valid
/// nix objects (as might be represented in a nix interpreter), as it
/// does not represent functions and some other objects.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ParsableNixValue {
    /// The singleton null value.
    Null,

    /// A function, which we do not inspect further.
    Function,

    /// Code which is not yet evaluated; this is returned by nix to
    /// allow for circularity and avoiding compilation of large data
    /// structures.
    Unevaluated,

    /// A derivation, which can be viewed as a set but we represent
    /// in a special way. Eventually this will be represented with a
    /// derivation object, but for now just the path to the derivation.
    Derivation(String),

    /// Boolean values (true/false).
    Bool(bool),

    /// Integers.
    Int(i64),

    /// Strings.
    String(String),

    /// Lists of nix values.
    List(Vec<ParsableNixValue>),

    /// Mappings from
    Set(HashMap<String, ParsableNixValue>),
}

/// Parse the output of 'nix-instantiate --eval'
pub fn parse_nix_instantiate(output: &str)
       -> Result<ParsableNixValue, String> {
    use self::ParsableNixValue::*;
    match output {
        "null" => Ok(Null),
        "true" => Ok(Bool(true)),
        "false" => Ok(Bool(false)),
        "<CODE>" => Ok(Unevaluated),
        "<LAMBDA>" => Ok(Function),
        s if s.starts_with("[") && s.ends_with("]") =>
            parse_nix_instantiate_list(s.to_string()),
        // s if s.starts_with("{") && s.ends_with("}") =>
        //     parse_nix_instantiate_set(s),
        _ => Err(format!("Unrecognized output: {}", output)),
    }
}

/// When parsing the output of nix-instantiate, we'll tokenize into a
/// vector of tokens; this type represents these.
enum NixOutputToken {
    Null, Bool(bool), String(String), CODE, LAMBDA,
    LParens, RParens, LBracket, RBracket, LCurly, RCurly,
}

use std::str::Chars;

// pub fn parse_nix_instantiate_(char: Chars)
//        -> Result<ParsableNixValue, String> {
//     use self::ParsableNixValue::*;
//     match output {
//         "null" => Ok(Null),
//         "true" => Ok(Bool(true)),
//         "false" => Ok(Bool(false)),
//         "<CODE>" => Ok(Unevaluated),
//         "<LAMBDA>" => Ok(Function),
//         s if s.starts_with("[") && s.ends_with("]") =>
//             parse_nix_instantiate_list(s.to_string()),
//         // s if s.starts_with("{") && s.ends_with("}") =>
//         //     parse_nix_instantiate_set(s),
//         _ => Err(format!("Unrecognized output: {}", output)),
//     }
// }

pub fn parse_nix_instantiate_list(list_string: String)
       -> Result<ParsableNixValue, String> {

    unimplemented!()
}

#[test]
fn parse_simples() {
    use self::ParsableNixValue::*;
    assert!(parse_nix_instantiate("null") == Ok(Null));
    assert!(parse_nix_instantiate("true") == Ok(Bool(true)));
    assert!(parse_nix_instantiate("false") == Ok(Bool(false)));
    assert!(parse_nix_instantiate("<CODE>") == Ok(Unevaluated));
    assert!(parse_nix_instantiate("<LAMBDA>") == Ok(Function));
}

#[test]
fn parse_list() {
    // assert!(parse_nix_instantiate("[ ]") == Ok(ParsableNixValue::List(vec!())));
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
