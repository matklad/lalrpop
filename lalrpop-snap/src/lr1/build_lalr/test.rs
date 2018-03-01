use intern::intern;
use grammar::repr::*;
use lr1::tls::Lr1Tls;
use test_util::normalized_grammar;
use tls::Tls;
use super::build_lalr_states;
use super::super::interpret::interpret;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

macro_rules! tokens {
    ($($x:expr),*) => {
        vec![$(TerminalString::quoted(intern($x))),*]
    }
}

#[test]
fn figure9_23() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(
        r#"
        grammar;
        extern { enum Tok { "-" => .., "N" => .., "(" => .., ")" => .. } }
        S: () = E       => ();
        E: () = {
            E "-" T     => (),
            T           => ()
        };
        T: () = {
            "N"         => (),
            "(" E ")"   => ()
        };
   "#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    let states = build_lalr_states(&grammar, nt("S")).unwrap();
    println!("{:#?}", states);

    let tree = interpret(&states, tokens!["N", "-", "(", "N", "-", "N", ")"]).unwrap();
    assert_eq!(
        &format!("{:?}", tree)[..],
        r#"[S: [E: [E: [T: "N"]], "-", [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]]]"#
    );
}
