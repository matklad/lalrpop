extern crate lollipop;
extern crate file;
extern crate failure;

use std::{
    process,
    path::PathBuf,
    env::args,
};

use lollipop::codegen::gen;

fn main() {
    let code = if let Err(e) = do_main() {
        eprintln!("{}", e);
        -1
    } else {
        0
    };
    process::exit(code)
}

fn do_main() -> Result<(), failure::Error> {
    let path = args().nth(1)
        .expect("filename");
    let path = PathBuf::from(path);
    let text = file::get_text(path)
        .expect("reading file");
    let code = gen(text)?;
    println!("code:\n\n{}\n\n", code);
    Ok(())
}

//fn emit_recursive_ascent(
//    session: &Session,
//    grammar: &r::Grammar,
//    report_file: &Path,
//) -> io::Result<Vec<u8>> {
//    let mut rust = RustWrite::new(vec![]);
//
//    // We generate a module structure like this:
//    //
//    // ```
//    // mod <output-file> {
//    //     // For each public symbol:
//    //     pub fn parse_XYZ();
//    //     mod __XYZ { ... }
//    //
//    //     // For each bit of action code:
//    //     <action-code>
//    // }
//    // ```
//    //
//    // Note that the action code goes in the outer module.  This is
//    // intentional because it means that the foo.lalrpop file serves
//    // as a module in the rust hierarchy, so if the action code
//    // includes things like `super::` it will resolve in the natural
//    // way.
//
//    try!(emit_module_attributes(grammar, &mut rust));
//    try!(emit_uses(grammar, &mut rust));
//    if let Some(ref symbols) = grammar.symbols {
//        emit_symbols(symbols, &mut rust)?;
//    }
//
//    if grammar.start_nonterminals.is_empty() {
//        println!("Error: no public symbols declared in grammar");
//        exit(1);
//    }
//
//    for (user_nt, start_nt) in &grammar.start_nonterminals {
//        // We generate these, so there should always be exactly 1
//        // production. Otherwise the LR(1) algorithm doesn't know
//        // where to stop!
//        assert_eq!(grammar.productions_for(start_nt).len(), 1);
//
//        log!(
//            session,
//            Verbose,
//            "Building states for public nonterminal `{}`",
//            user_nt
//        );
//
//        let _lr1_tls = lr1::Lr1Tls::install(grammar.terminals.clone());
//
//        let lr1result = lr1::build_states(&grammar, start_nt.clone());
//        if session.emit_report {
//            let mut output_report_file = try!(fs::File::create(&report_file));
//            try!(lr1::generate_report(&mut output_report_file, &lr1result));
//        }
//
//        let states = match lr1result {
//            Ok(states) => states,
//            Err(error) => {
//                let messages = lr1::report_error(&grammar, &error);
//                let _ = report_messages(messages);
//                exit(1) // FIXME -- propagate up instead of calling `exit`
//            }
//        };
//
//        match grammar.algorithm.codegen {
//            r::LrCodeGeneration::RecursiveAscent => try!(lr1::codegen::ascent::compile(
//                &grammar,
//                user_nt.clone(),
//                start_nt.clone(),
//                &states,
//                "super",
//                &mut rust,
//            )),
//            r::LrCodeGeneration::TableDriven => try!(lr1::codegen::parse_table::compile(
//                &grammar,
//                user_nt.clone(),
//                start_nt.clone(),
//                &states,
//                "super",
//                &mut rust,
//            )),
//
//            r::LrCodeGeneration::TestAll => try!(lr1::codegen::test_all::compile(
//                &grammar,
//                user_nt.clone(),
//                start_nt.clone(),
//                &states,
//                &mut rust,
//            )),
//        }
//
//        rust!(
//            rust,
//            "{}use self::{}parse{}::{}Parser;",
//            grammar.nonterminals[&user_nt].visibility,
//            grammar.prefix,
//            start_nt,
//            user_nt
//        );
//    }
//
//    if let Some(ref intern_token) = grammar.intern_token {
//        try!(intern_token::compile(&grammar, intern_token, &mut rust));
//        rust!(rust, "pub use self::{}intern_token::Token;", grammar.prefix);
//    }
//
//    try!(action::emit_action_code(grammar, &mut rust));
//
//    try!(emit_to_triple_trait(grammar, &mut rust));
//
//    Ok(rust.into_inner())
//}
