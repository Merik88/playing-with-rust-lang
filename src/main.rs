mod lexer;
mod parser;

fn main() {
    let string = String::from("Paper 100");

    let tokens = lexer::lexer(string);
    println!("{:?}", tokens);

    let ast = parser::parser(tokens);
    println!("{:?}", ast);
}
