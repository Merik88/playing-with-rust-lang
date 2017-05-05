mod lexer;
mod parser;
mod transformer;
mod generator;
mod compiler;

fn main() {
    let code = "Paper 0 Pen 100 Line 0 50 100 50".to_string();
    let svg = compiler::compile(code);
    println!("{}", svg);
}
