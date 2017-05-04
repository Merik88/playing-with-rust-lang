mod lexer;

fn main() {
    let string = String::from("Paper 100");
    let tokens = lexer::lexer(string);
    println!("{:?}", tokens);
}
