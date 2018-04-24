extern crate file;
extern crate heck;

use heck::ShoutySnakeCase;

fn main() {
    gen_ast();
}

fn gen_ast() {
    let mut buff = String::new();
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            buff.push_str(&format!($($tt)*));
            buff.push_str("\n");
        }};
    }

    ln!("use parse_tree::Node;");
    ln!("use super::{{AstNode, AstChildren}};");
    ln!("use super::super::symbols::*;");
    ln!();
    let wrappers = &[
        "File",
        "TokensDef",
        "TokenDef",
        "RuleDef",
        "Expr",
        "ParenExpr",
        "Symbol",
        "Op",
        "Word",
        "Regex",
        "Ident"
    ];
    let enums: &[(&str, &[&str])] = &[
        ("TokenRe", &["Word", "Regex"]),
        ("Atom", &["Word", "Ident", "ParenExpr",]),
    ];

    for &symbol in wrappers.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub struct {}<'p>(Node<'p>);", symbol);
        ln!();
    }
    ln!();

    for &(ref symbol, ref variants) in enums.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub enum {}<'p> {{", symbol);
        for &v in variants.iter() {
            ln!("    {}({}<'p>),", v, v);
        }
        ln!("}}");
        ln!();
    }


    for &symbol in wrappers.iter() {
        ln!("impl<'p> AstNode<'p> for {}<'p> {{", symbol);
        ln!("    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {{");
        ln!(
            "        if node.symbol() == {} {{ Some({}(node)) }} else {{ None }}",
            symbol.to_shouty_snake_case(),
            symbol
        );
        ln!("    }}");
        ln!("    fn node(self) -> Node<'p> {{ self.0 }}");

        ln!("}}");
        ln!();
    }

    for &(ref symbol, ref variants) in enums.iter() {
        ln!("impl<'p> AstNode<'p> for {}<'p> {{", symbol);
        ln!("    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {{");
        for &v in variants.iter() {
            ln!(
                "        if let Some(n) = {}::cast(node) {{ return Some({}::{}(n)); }}",
                v,
                symbol,
                v,
            );
        }
        ln!("        None");
        ln!("    }}");
        ln!("    fn node(self) -> Node<'p> {{");
        ln!("        match self {{");
        for &v in variants.iter() {
            ln!("            {}::{}(n) => n.node(),", symbol, v);
        }
        ln!("        }}");
        ln!("    }}");
        ln!("}}");
        ln!();
    }

    let methods: &[(&str, &[(&str, &str)])] = &[
        ("File", &[("tokens_def", "TokensDef"), ("rules", "RuleDef")]),
        ("TokensDef", &[("tokens", "TokenDef")]),
        ("TokenDef", &[("name", "Ident"), ("re", "TokenRe")]),
        ("RuleDef", &[("name", "Ident"), ("alts", "Expr")]),
    ];

    for &(ref s, ref ms) in methods.iter() {
        ln!("impl<'p> {}<'p> {{", s);
        for &(ref acc, ref s) in ms.iter() {
            let (ret, body) = if acc.ends_with("s") {
                (format!("AstChildren<'p, {}<'p>>", s), "AstChildren::new(self.node().children())")
            } else {
                (format!("{}<'p>", s), "AstChildren::new(self.node().children()).next().unwrap()")
            };
            ln!("    pub fn {}(&self) -> {} {{", acc, ret);
            ln!("        {}", body);
            ln!("    }}");
        }
        ln!("}}");
    }

    file::put_text("../generated.rs", &buff)
        .unwrap();
}
