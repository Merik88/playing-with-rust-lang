use lexer::Token;

#[derive(Debug)]
pub struct AST {
    t: String,
    b: Vec<Expression>,
}

#[derive(Debug)]
struct Expression {
    t: String,
    n: String,
    a: Vec<Argument>,
}

#[derive(Debug)]
struct Argument {
    t: String,
    v: String,
}

pub fn parser(mut tokens: Vec<Token>) -> AST {
    let mut ast: AST = AST { t: String::from("Drawing"), b: Vec::<Expression>::new() };
    tokens.reverse();
    while !tokens.is_empty() {
        let current_token = tokens.pop().unwrap();
        if current_token.t == "word" {
            match current_token.v.as_ref() {
                "Paper" => {
                    let mut exp = Expression { t: String::from("CallExpression"), n: String::from("Paper"), a: Vec::<Argument>::new() };

                    let arg = tokens.pop().unwrap();

                    if arg.t == "number" {
                        exp.a.push(Argument {
                            t: String::from("NumberLiteral"),
                            v: arg.v,
                        });
                        ast.b.push(exp);
                    } else {
                        panic!("Paper command must be followed by a number.");
                    }
                },
                "Pen" => {},
                "Line" => {},
                _ => continue,
            }
        }
    }
    ast
}