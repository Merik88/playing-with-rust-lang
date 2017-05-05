use lexer::lexer;
use parser::parser;
use transformer::transformer;
use generator::generator;

pub fn compile(code: String) -> String {
    generator(transformer(parser(lexer(code))))
}