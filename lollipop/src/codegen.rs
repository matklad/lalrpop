use std::{
    rc::Rc,
    path::PathBuf,
};
use lalrpop::{
    build,
    rust::RustWrite,
    grammar::repr as r,
    lr1,
    tls::Tls,
    session::Session,
    file_text::FileText,
    lexer::intern_token,
};
use ::{
    parse,
    lower::lower,
    ast::{self, AstNode},
};

type Result<T> = ::std::result::Result<T, ::failure::Error>;

pub fn gen(input: String) -> Result<String> {
    let sess = Session::new();
    let sess = Rc::new(sess);
    let file_text = FileText::new(
        PathBuf::from(".lolipop"),
        input.clone(),
    );
    let _tls = Tls::install(sess.clone(), Rc::new(file_text)); // :((((
    let file = parse(input);
    let file = ast::File::cast(file.root()).unwrap();
    let (grammar, diags) = lower(file);
    for d in diags {
        eprintln!("{}", d);
    }
    let grammar = match grammar {
        Some(g) => g,
        None => {
            bail!("error: failed to compile grammar")
        }
    };
    println!("lowered!");
    let bytes = build::emit_recursive_ascent(&sess, &grammar, &PathBuf::from("/tmp/report"))?;
    Ok(String::from_utf8(bytes).unwrap())
}
