use string_cache::DefaultAtom as Atom;
use lalrpop::{
    collections::Map,
    grammar::{
        repr as r,
        parse_tree::{self as pt, Span, TerminalLiteral},
    },
};
use ::{
    ast::{self, AstNode},
    analysis::{
        Analysis,
        lexer::MatchBlock,
    },
    ide::Diagnostic
};

pub fn lower(file: ast::File) -> (Option<r::Grammar>, Vec<Diagnostic>) {
    let mut a = Analysis::new();
    let g = do_lower(&mut a, file);
    (g, a.diagnostics())
}

impl<'p> ast::Ident<'p> {
    fn nt_string(&self) -> r::NonterminalString {
        r::NonterminalString(Atom::from(self.as_str()))
    }
}

fn span_of(n: ::parse_tree::Node) -> Span {
    let r = n.range();
    let s: u32 = r.start().into();
    let e: u32 = r.end().into();
    Span(s as usize, e as usize)
}

fn do_lower(a: &mut Analysis, file: ast::File) -> Option<r::Grammar> {
    let start_nonterminals = {
        let start = a.start_symbol(file)?;
        let nt = start.nt_string();
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

    let nonterminals = {
        let mut m = Map::new();
        for r in file.rules() {
            let name = r.name().nt_string();
            let data = r::NonterminalData {
                name: name.clone(),
                visibility: if Some(r.name()) == a.start_symbol(file) {
                    r::Visibility::Pub(None)
                } else {
                    r::Visibility::Priv
                },
                span: span_of(r.node()),
                annotations: Vec::new(),
                productions: lower_productions(a, r),
            };
            m.insert(name, data);
        }
        m
    };

    let g = r::Grammar {
        prefix: String::new(),
        algorithm: r::Algorithm {
            lalr: false,
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
        nonterminals,
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

fn lower_productions(a: &mut Analysis, rule: ast::RuleDef) -> Vec<r::Production> {
    rule.alts().map(|alt| lower_production(a, alt, rule.name())).collect()
}

fn lower_production(a: &mut Analysis, alt: ast::Expr, nt: ast::Ident) -> r::Production {
    r::Production {
        nonterminal: nt.nt_string(),
        symbols: vec![],
        action: r::ActionFn::new(0),
        span: span_of(alt.node()),
    }
}
