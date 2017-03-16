extern crate regex;
extern crate unescape;

use std::slice;
use std::iter::Peekable;
use regex::{Regex, CaptureMatches};

lazy_static! {
pub static ref NIX_INSTANTIATE_OUTPUT_RE: Regex = Regex::new(r#"(?x)
  -?\d+                                 # Number
  | [a-zA-Z][\w_\-']*                   # Identifier
  | "(\\.|[^"])*"                       # String literal
  | <CODE> | <LAMBDA> | <PRIMOP>        # Special indicators
  | \[ | \] | \{ | \} | \( | \) | = | ; # Punctuation
"#).unwrap();
}

/// When parsing the output of nix-instantiate, we'll tokenize into a
/// vector of tokens; this type represents these.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Token {
    Null, Bool(bool), String(String), CODE, LAMBDA, PRIMOP, Eq, Semi,
    Number(i64), LParens, RParens, LBracket, RBracket, LCurly, RCurly,
    Ident(String),
}

fn token_from_str(token_str: &str) -> Token {
    lazy_static! {
        static ref int_re: Regex = Regex::new(r"-?\d+").unwrap();
    }
    use self::Token::*;
    match token_str {
        "null" => Null,
        "true" => Bool(true),
        "false" => Bool(false),
        "<CODE>" => CODE,
        "<LAMBDA>" => LAMBDA,
        "<PRIMOP>" => PRIMOP,
        "(" => LParens, ")" => RParens,
        "[" => LBracket, "]" => RBracket,
        "{" => LCurly, "}" => RCurly,
        "=" => Eq, ";" => Semi,
        s if s.starts_with("\"") =>
            // Trim off the quotes, unescape and panic if it fails
            String(unescape::unescape(&s[1..s.len() - 1]).unwrap()),
        s if int_re.is_match(s) => Number(s.parse().unwrap()),
        _ => Ident(token_str.to_string()),
    }
}

/// Iterator for nix output tokens, wraps regex capture matches.
struct Tokens<'a> {
    iter: Peekable<CaptureMatches<'static, 'a>>,
}

impl<'a> Tokens<'a> {
    fn peek(&mut self) -> Option<Token> {
        self.iter.peek().map(|t| token_from_str(&t[0]))
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.iter.next().map(|t| token_from_str(&t[0]))
    }
}


/// A type representing nix values which can be parsed from the output
/// of nix-instantiate. This might be seen as a subset of all valid
/// nix objects (as might be represented in a nix interpreter), as it
/// does not represent functions and some other objects.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Value {
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
    Number(i64),

    /// Strings.
    String(String),

    /// Lists of nix values.
    List(Vec<Value>),

    // Mappings from
    // Set(HashMap<String, Value>),
}

#[derive(Debug, PartialEq)]
enum ParseError {
    UnexpectedEndOfInput,
    UnexpectedToken(Token),
    UnterminatedList,
}

/// Parse a stream of nix output tokens into a nix value. Consumes one or more
/// values from the stream.
fn parse_tokens(tokens: &mut Tokens)
   -> Result<Value, ParseError> {
    use self::ParseError::*;
    let tok = tokens.next();
    println!("Token: {:?}", tok);
    match tok {
        Some(Token::Null) => Ok(Value::Null),
        Some(Token::Bool(b)) => Ok(Value::Bool(b)),
        Some(Token::Number(n)) => Ok(Value::Number(n)),
        Some(Token::String(s)) => Ok(Value::String(s)),
        Some(Token::CODE) => Ok(Value::Unevaluated),
        Some(Token::LAMBDA) | Some(Token::PRIMOP) => Ok(Value::Function),
        Some(Token::LBracket) =>
            parse_list(tokens),
        Some(t) => Err(UnexpectedToken(t)),
        None => Err(UnexpectedEndOfInput),
    }
}

/// Parse a nix list, e.g. [1 2 3].
fn parse_list(tokens: &mut Tokens)
   -> Result<Value, ParseError> {
    use self::ParseError::*;
    let mut values = vec!();
    loop {
        match tokens.peek() {
            Some(Token::RBracket) => {
                println!("terminating the list");
                // Consume the bracket.
                tokens.next();
                println!("ok");
                // Exit the loop wrapping the vector in a list constructor.
                return Ok(Value::List(values));
            },
            Some(_) => {println!("turds"); match parse_tokens(tokens) {
                Ok(value) => values.push(value),
                err => return err
            }},
            _ => return Err(UnterminatedList),
        }
    }

}

fn parse_nix_instantiate(output: &str) -> Result<Value, ParseError> {
    let mut tokens = Tokens {
        iter: NIX_INSTANTIATE_OUTPUT_RE.captures_iter(output).peekable()
    };
    parse_tokens(&mut tokens)
}

#[test]
fn test_parse_literals() {
    use self::Value::*;
    assert!(parse_nix_instantiate("null") == Ok(Null));
    assert!(parse_nix_instantiate("true") == Ok(Bool(true)));
    assert!(parse_nix_instantiate("false") == Ok(Bool(false)));
    assert!(parse_nix_instantiate("<CODE>") == Ok(Unevaluated));
    assert!(parse_nix_instantiate("<LAMBDA>") == Ok(Function));
    assert!(parse_nix_instantiate("<PRIMOP>") == Ok(Function));
}

#[test]
fn test_parse_strings() {
    use self::Value::*;
    assert!(parse_nix_instantiate("\"quote me\"") ==
            Ok(String("quote me".to_string())));
    assert!(parse_nix_instantiate(r#""I am a\nmore \"complex\" string.""#) ==
            Ok(String("I am a\nmore \"complex\" string.".to_string())));
}

#[test]
fn test_parse_nums() {
    use self::Value::*;
    assert!(parse_nix_instantiate("123") == Ok(Number(123)));
    assert!(parse_nix_instantiate("-123") == Ok(Number(-123)));
}

#[test]
fn test_parse_list() {
    use self::Value::*;
    let res = parse_nix_instantiate("[1 2 3]");
    let expected = Ok(List(vec!(Number(1), Number(2), Number(3))));
    debug_assert!(res == expected, "expected {:?}, but got {:?}",
                  expected, res);

}

#[test]
fn test_parse_nested_list() {
    use self::Value::*;
    let res = parse_nix_instantiate("[1 2 [3 4]]");
    let expected = Ok(List(vec!(Number(1), Number(2),
                                List(vec!(Number(3), Number(4))))));
    debug_assert!(res == expected, "expected {:?}, but got {:?}",
                  expected, res);

}


// fn parse_nix_instantiate(iterator: ) -> Result<String, Value> {
//     let mut result = Err("Nothing parsed".to_string());
//     for tok in NIX_INSTANTIATE_OUTPUT_RE.captures_iter(text) {
//         match token_from_str(tok) {
//             Null , Bool(bool), String(String), CODE, LAMBDA, PRIMOP, Eq, Semi,
//     Number(i64), LParens, RParens, LBracket, RBracket, LCurly, RCurly,
//     Ident(String),

//         }
//     }
// }


// /// Remove the escape sequences from a string. For example, translate
// /// \n into a newline, \" into a quote, etc.
// fn unescape_string(string: &str) -> Result<String, String> {
//     let mut result = String::new();
//     let mut in_escape = false;
//     for c in string.chars() {
//         if in_escape {
//             match c {
//                 '\\' => result.push('\\')
//                 'n' => result.push('\n')
//         unimplemented!()
//     }
//     unimplemented!()
// }

#[test]
fn test_token_from_str() {
    use self::Token::*;
    assert!(token_from_str("null") == Null);
    assert!(token_from_str("true") == Bool(true));
    assert!(token_from_str("false") == Bool(false));
    assert!(token_from_str("<CODE>") == CODE);
    assert!(token_from_str("<LAMBDA>") == LAMBDA);
    assert!(token_from_str("123") == Number(123));
    assert!(token_from_str("-123") == Number(-123));
    assert!(token_from_str("hello") == Ident("hello".to_string()));
    assert!(token_from_str("\"quote me\"") == String("quote me".to_string()));
    assert!(token_from_str(r#""I am a\nmore "complex" string.""#) ==
             String("I am a\nmore \"complex\" string.".to_string()));
}
