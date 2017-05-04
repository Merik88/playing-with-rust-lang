#[derive(Debug)]
pub struct Token {
    t: String,
    v: String,
}

pub fn lexer(code: String) -> Vec<Token> {
    code.split_whitespace()
        .filter(not_empty_string)
        .map(string_to_token)
        .collect()
}

fn string_to_token(s: &str) -> Token {
    let n = s.parse::<i32>();
    match n {
        Ok(_) => Token { t: String::from("number"), v: String::from(s) },
        Err(_) => Token { t: String::from("word"), v: String::from(s) },
    }
}

fn not_empty_string(s: &&str) -> bool {
    s.len() > 0
}