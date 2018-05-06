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
};
use ::{
    parse,
    lower::lower,
    ast::{self, AstNode},
};

type Result<T> = ::std::result::Result<T, ::failure::Error>;

pub fn gen(input: String) -> Result<String> {
    let sess = Session::new();
    let file_text = FileText::new(
        PathBuf::from(".lolipop"),
        input.clone(),
    );
    let _tls = Tls::install(Rc::new(sess), Rc::new(file_text)); // :((((
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
    emit_recursive_ascent(&grammar)
}

fn emit_recursive_ascent(
    grammar: &r::Grammar,
) -> Result<String> {
    let mut rust = RustWrite::new(vec![]);

    // We generate a module structure like this:
    //
    // ```
    // mod <output-file> {
    //     // For each public symbol:
    //     pub fn parse_XYZ();
    //     mod __XYZ { ... }
    //
    //     // For each bit of action code:
    //     <action-code>
    // }
    // ```
    //
    // Note that the action code goes in the outer module.  This is
    // intentional because it means that the foo.lalrpop file serves
    // as a module in the rust hierarchy, so if the action code
    // includes things like `super::` it will resolve in the natural
    // way.

    build::emit_module_attributes(grammar, &mut rust)?;
    build::emit_uses(grammar, &mut rust)?;
    if let Some(ref symbols) = grammar.symbols {
        build::emit_symbols(symbols, &mut rust)?;
    }
    if grammar.start_nonterminals.is_empty() {
        bail!("Error: no public symbols declared in grammar");
    }
    for (user_nt, start_nt) in &grammar.start_nonterminals {
        // We generate these, so there should always be exactly 1
        // production. Otherwise the LR(1) algorithm doesn't know
        // where to stop!
        assert_eq!(grammar.productions_for(start_nt).len(), 1);

        let _lr1_tls = lr1::Lr1Tls::install(grammar.terminals.clone());

        let lr1result = lr1::build_states(&grammar, start_nt.clone());

        let states = match lr1result {
            Ok(states) => states,
            Err(error) => {
                bail!("{:?}", error)
            }
        };

//        lr1::codegen::ascent::compile(
//            &grammar,
//            user_nt.clone(),
//            start_nt.clone(),
//            &states,
//            "super",
//            &mut rust,
//        )?;
//
//        rust!(
//            rust,
//            "{}use self::{}parse{}::{}Parser;",
//            grammar.nonterminals[&user_nt].visibility,
//            grammar.prefix,
//            start_nt,
//            user_nt
//        );
    }
//
//    if let Some(ref intern_token) = grammar.intern_token {
//        try!(intern_token::compile(&grammar, intern_token, &mut rust));
//        rust!(rust, "pub use self::{}intern_token::Token;", grammar.prefix);
//    }
//
//    try!(action::emit_action_code(grammar, &mut rust));
//
//    try!(emit_to_triple_trait(grammar, &mut rust));
    let bytes = rust.into_inner();
    let text = String::from_utf8(bytes).unwrap();
    Ok(text)
}

