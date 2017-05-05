use lexer::Token;

#[derive(Debug)]
pub struct Ast {
    t: String,
    pub b: Vec<Expression>,
}

#[derive(Debug)]
pub struct Expression {
    t: String,
    pub n: String,
    pub a: Vec<Argument>,
}

#[derive(Debug)]
pub struct Argument {
    t: String,
    pub v: String,
}

pub fn parser(mut tokens: Vec<Token>) -> Ast {
    let mut ast = Ast { t: String::from("Drawing"), b: Vec::<Expression>::new() };
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