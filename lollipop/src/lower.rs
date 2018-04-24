use string_cache::DefaultAtom as Atom;
use lalrpop::grammar::parse_tree::Span;
use lalrpop::grammar::repr as r;
use lalrpop::grammar::parse_tree as pt;
use lalrpop::collections::Map;

use super::ast;
use analysis::{
    Analysis,
    lexer::MatchBlock,
};
use lalrpop::grammar::parse_tree::TerminalLiteral;

pub fn lower(file: ast::File) -> Option<r::Grammar> {
    let mut a = Analysis::new();
    do_lower(&mut a, file)
}

fn do_lower(a: &mut Analysis, file: ast::File) -> Option<r::Grammar> {
    let start_nonterminals = {
        let start = a.start_symbol(file)?;
        let nt = r::NonterminalString(Atom::from(start.as_str()));
        let mut m = Map::new();
        m.insert(nt.clone(), nt);
        m
    };

    let e = Atom::from("E");
    let e_type_repr = r::TypeRepr::Nominal(r::NominalTypeRepr {
        path: pt::Path {
            absolute: false,
            ids: vec![e.clone()],
        },
        types: Vec::new(),
    });
    let type_parameters = vec![pt::TypeParameter::Id(e.clone())];
    let parameters = {
        let ty = r::TypeRepr::Ref {
            lifetime: None,
            mutable: true,
            referent: Box::new(e_type_repr.clone()),
        };
        vec![r::Parameter { name: Atom::from("events"), ty }]
    };
    let where_clauses = {
        let bounds = vec![pt::TypeBound::Trait {
            forall: None,
            path: pt::Path {
                absolute: false,
                ids: vec![Atom::from("__lalrpop_util") /* TODO: do properly */,
                          Atom::from("LrEvents")],
            },
            parameters: vec![],
        }];
        vec![pt::WhereClause::Type {
            forall: None,
            ty: e_type_repr.clone(),
            bounds,
        }]
    };

    a.lexer_dfa(file);

    let g = r::Grammar {
        prefix: String::new(),
        algorithm: r::Algorithm {
            lalr: true,
            codegen: r::LrCodeGeneration::RecursiveAscent,
        },
        uses_error_recovery: false,
        start_nonterminals,
        uses: Vec::new(),
        type_parameters,
        parameters,
        where_clauses,
        intern_token: None,
        action_fn_defns: Vec::new(),
        terminals: r::TerminalSet {
            all: Vec::new(),
            bits: Map::new(),
        },
        symbols: None,
        nonterminals: Map::new(),
        token_span: Span(0, 0),
        conversions: Map::new(),
        types: r::Types::new(
            "",
            None,
            None,
            r::TypeRepr::Nominal(r::NominalTypeRepr {
                path: r::Path {
                    absolute: false,
                    ids: vec![],
                },
                types: vec![],
            }),
        ),
        module_attributes: Vec::new(),
    };
    Some(g)
}


//fn construct(grammar: &mut Grammar, match_block: MatchBlock) -> NormResult<()> {
//
//    let dfa = match dfa::build_dfa(&regexs, &precedences) {
//        Ok(dfa) => dfa,
//        Err(DFAConstructionError::NFAConstructionError { index, error }) => {
//            let feature = match error {
//                NamedCaptures => r#"named captures (`(?P<foo>...)`)"#,
//                NonGreedy => r#""non-greedy" repetitions (`*?` or `+?`)"#,
//                WordBoundary => r#"word boundaries (`\b` or `\B`)"#,
//                LineBoundary => r#"line boundaries (`^` or `$`)"#,
//                TextBoundary => r#"text boundaries (`^` or `$`)"#,
//                ByteRegex => r#"byte-based matches"#,
//            };
//            let literal = &match_entries[index.index()].match_literal;
//            return_err!(
//                spans[literal],
//                "{} are not supported in regular expressions",
//                feature
//            )
//        }
//        Err(DFAConstructionError::Ambiguity { match0, match1 }) => {
//            let literal0 = &match_entries[match0.index()].match_literal;
//            let literal1 = &match_entries[match1.index()].match_literal;
//            // FIXME(#88) -- it'd be nice to give an example here
//            return_err!(
//                spans[literal0],
//                "ambiguity detected between the terminal `{}` and the terminal `{}`",
//                literal0,
//                literal1
//            )
//        }
//    };
//
//    grammar.items.push(GrammarItem::InternToken(InternToken {
//        match_entries: match_entries,
//        dfa: dfa,
//    }));
//
//    // we need to inject a `'input` lifetime and `input: &'input str` parameter as well:
//
//    let input_lifetime = Atom::from(INPUT_LIFETIME);
//    for parameter in &grammar.type_parameters {
//        match *parameter {
//            TypeParameter::Lifetime(ref i) if *i == input_lifetime => {
//                return_err!(
//                    grammar.span,
//                    "since there is no external token enum specified, \
//                     the `'input` lifetime is implicit and cannot be declared"
//                );
//            }
//            _ => {}
//        }
//    }
//
//    let input_parameter = Atom::from(INPUT_PARAMETER);
//    for parameter in &grammar.parameters {
//        if parameter.name == input_parameter {
//            return_err!(
//                grammar.span,
//                "since there is no external token enum specified, \
//                 the `input` parameter is implicit and cannot be declared"
//            );
//        }
//    }
//
//    grammar
//        .type_parameters
//        .insert(0, TypeParameter::Lifetime(input_lifetime.clone()));
//
//    let parameter = Parameter {
//        name: input_parameter,
//        ty: TypeRef::Ref {
//            lifetime: Some(input_lifetime),
//            mutable: false,
//            referent: Box::new(TypeRef::Id(Atom::from("str"))),
//        },
//    };
//    grammar.parameters.push(parameter);
//
//    Ok(())
//}
