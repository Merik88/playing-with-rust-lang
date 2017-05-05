mod lexer;
mod parser;
mod transformer;
mod generator;

fn main() {
    let string = String::from("Paper 100");

    let tokens = lexer::lexer(string);
    println!("{:?}", tokens);

    let ast = parser::parser(tokens);
    println!("{:?}", ast);

    let svg_ast = transformer::transformer(ast);
    println!("{:?}", svg_ast);

    let svg = generator::generator(svg_ast);
    println!("{:?}", svg);
}
