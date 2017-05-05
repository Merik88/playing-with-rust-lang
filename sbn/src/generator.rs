use transformer::{SvgAst, Attr1, Attr2};

pub fn generator(svg_ast: SvgAst) -> String {
    
    let svg_attr = create_attr1_string(svg_ast.attr);
    
    let elements = svg_ast.body.into_iter()
        .map(|node| format!("<{0} {1}></{0}>", node.tag, create_attr2_string(node.attr)))
        .collect::<Vec<String>>()
        .join("\n\t");

    format!("<svg {0}>\n{1}\n</svg>", svg_attr, elements)
}

fn create_attr1_string(attr: Attr1) -> String {
    format!("height={} width={} viewbox={} xmlns={} version={}", 
        attr.height, attr.width, attr.viewbox, attr.xmlns, attr.version)
}

fn create_attr2_string(attr: Attr2) -> String {
    format!(r"x={} y={} width={} height={} fill={}", 
        attr.x, attr.y, attr.width, attr.height, attr.fill)
}