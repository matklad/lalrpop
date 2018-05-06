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
use ide::Diagnostic;

pub fn lower(file: ast::File) -> (Option<r::Grammar>, Vec<Diagnostic>) {
    let mut a = Analysis::new();
    let g = do_lower(&mut a, file);
    (g, a.diagnostics())
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

    let symbols = Some(a.symbols(file).into_iter().map(|s| s.to_owned()).collect());

    let intern_token = a.lexer_dfa(file);

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
        intern_token,
        action_fn_defns: Vec::new(),
        terminals: r::TerminalSet {
            all: Vec::new(),
            bits: Map::new(),
        },
        symbols,
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
