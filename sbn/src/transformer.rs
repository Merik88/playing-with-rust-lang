use parser::Ast;

#[derive(Debug)]
pub struct SvgAst {
    tag: String,
    pub attr: Attr1,
    pub body: Vec<Element>,
}

#[derive(Debug)]
pub struct Attr1 {
    pub width: i32,
    pub height: i32,
    pub viewbox: String,
    pub xmlns: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Attr2 {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fill: String,
}

#[derive(Debug)]
pub struct Element {
    pub tag: String,
    pub attr: Attr2,
}

pub fn transformer(mut ast: Ast) -> SvgAst {
    let mut svg_ast = SvgAst {
        tag : String::from("svg"),
        attr: Attr1 {
            width: 100,
            height: 100,
            viewbox: String::from("0 0 100 100"),
            xmlns: String::from("http://www.w3.org/2000/svg"),
            version: String::from("1.1"),
        },
        body: Vec::<Element>::new(),
    };

    let mut _pen_color = 100;

    while !ast.b.is_empty() {
        let node = ast.b.pop().unwrap();

        match node.n.as_ref() {
            "Paper" => {
                let paper_color = 100 - node.a[0].v.parse::<i32>().unwrap();
                let fill = format!("rgb({0}%,{0}%,{0}%)", paper_color);
                let ele = Element {
                    tag: String::from("rect"),
                    attr: Attr2 { x: 0, y: 0, width: 100, height: 100, fill: fill},
                };
                svg_ast.body.push(ele);
            },
            "Pen" => {
                _pen_color = 100 - node.a[0].v.parse::<i32>().unwrap();
            },
            "Line" => {},
            _ => continue,
        }
    }

    svg_ast
}