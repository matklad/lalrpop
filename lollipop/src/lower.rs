use string_cache::DefaultAtom as Atom;
use lalrpop::{
    collections::Map,
    grammar::{
        repr as r,
        parse_tree::{self as pt, Span, TerminalLiteral, TerminalString},
        pattern::{Pattern, PatternKind},
    },
};
use ::{
    ast::{self, AstNode},
    analysis::{
        Analysis,
        lexer::MatchBlock,
    },
    ide::Diagnostic,
};

pub fn lower(file: ast::File) -> (Option<r::Grammar>, Vec<Diagnostic>) {
    let mut ctx = LowerCtx::new(file);
    ctx.into_grammar()
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

struct LowerCtx<'p> {
    analysis: Analysis,
    file: ast::File<'p>,
    action_fn_defns: Vec<r::ActionFnDefn>,
}

impl<'p> LowerCtx<'p> {
    fn new(file: ast::File<'p>) -> Self {
        LowerCtx {
            analysis: Analysis::new(),
            file,
            action_fn_defns: Vec::new(),
        }
    }

    fn into_grammar(mut self) -> (Option<r::Grammar>, Vec<Diagnostic>) {
        let g = self.lower();
        (g, self.analysis.diagnostics())
    }

    fn lower(&mut self) -> Option<r::Grammar> {
        let start_symbol = self.analysis.start_symbol(self.file)?;
        let start_nonterminals = {
            let nt = start_symbol.nt_string();
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

        let symbols = Some(self.analysis.symbols(self.file).into_iter().map(|s| s.to_owned()).collect());

        let intern_token = self.analysis.lexer_dfa(self.file);
        let internal_token_path = pt::Path {
            absolute: false,
            ids: vec![Atom::from("Token")],
        };
        let input_str = r::TypeRepr::Ref {
            lifetime: Some(Atom::from("'input")),
            mutable: false,
            referent: Box::new(r::TypeRepr::Nominal(r::NominalTypeRepr {
                path: r::Path::str(),
                types: vec![],
            })),
        };
        let conversions: Vec<(TerminalString, Pattern<r::TypeRepr>)> =
            intern_token.as_ref().into_iter()
                .flat_map(|it| it.match_entries.iter())
                .enumerate()
                .map(|(index, match_entry)| {
                    let pattern = Pattern {
                        span: Span(0, 0),
                        kind: PatternKind::TupleStruct(
                            internal_token_path.clone(),
                            vec![
                                Pattern {
                                    span: Span(0, 0),
                                    kind: PatternKind::Usize(index),
                                },
                                Pattern {
                                    span: Span(0, 0),
                                    kind: PatternKind::Choose(input_str.clone()),
                                },
                            ],
                        ),
                    };

                    (match_entry.user_name.clone(), pattern)
                },
                ).collect();

        let mut types = r::Types::new(
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
        );
        let nonterminals = {
            let mut m = Map::new();
            for r in self.file.rules() {
                let name = r.name().nt_string();
                let data = r::NonterminalData {
                    name: name.clone(),
                    visibility: if r.name() == start_symbol {
                        r::Visibility::Pub(None)
                    } else {
                        r::Visibility::Priv
                    },
                    span: span_of(r.node()),
                    annotations: Vec::new(),
                    productions: self.lower_productions(r),
                };
                types.add_type(name.clone(), usize_type());
                m.insert(name, data);
            }
            m
        };

        let mut all_terminals: Vec<_> = conversions
            .iter()
            .map(|c| c.0.clone())
            .collect();
        all_terminals.sort();

        let terminal_bits: Map<_, _> = all_terminals.iter().cloned().zip(0..).collect();

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
            action_fn_defns: ::std::mem::replace(&mut self.action_fn_defns, Vec::new()),
            terminals: r::TerminalSet {
                all: all_terminals,
                bits: terminal_bits,
            },
            symbols,
            nonterminals,
            token_span: Span(0, 0),
            conversions: conversions.into_iter().collect(),
            types,
            module_attributes: Vec::new(),
        };
        Some(g)
    }

    fn lower_productions(&mut self, rule: ast::RuleDef) -> Vec<r::Production> {
        rule.alts().map(|alt| self.lower_production(alt, rule.name())).collect()
    }

    fn lower_production(&mut self, alt: ast::Expr, nt: ast::Ident) -> r::Production {
        r::Production {
            nonterminal: nt.nt_string(),
            symbols: alt.symbols().map(|s| self.lower_symbol(s)).collect(),
            action: self.mk_action(),
            span: span_of(alt.node()),
        }
    }

    fn lower_symbol(&mut self, symbol: ast::Symbol) -> r::Symbol {
        let name = symbol.node().text();
        let l = name.len();
        let name = &name[1..l - 1];
        let s = TerminalString::Bare(Atom::from(name));
        r::Symbol::Terminal(s)
    }

    fn mk_action(&mut self) -> r::ActionFn {
        let kind = r::UserActionFnDefn {
            arg_patterns: Vec::new(),
            arg_types: Vec::new(),
            code: String::from("92"),
        };
        let def = r::ActionFnDefn {
            fallible: false,
            ret_type: usize_type(),
            kind: r::ActionFnDefnKind::User(kind),
        };
        let id = self.action_fn_defns.len();
        self.action_fn_defns.push(def);
        r::ActionFn::new(id)
    }
}

fn usize_type() -> r::TypeRepr {
    r::TypeRepr::Nominal(r::NominalTypeRepr {
        path: pt::Path {
            absolute: false,
            ids: vec![Atom::from("usize")],
        },
        types: Vec::new(),
    })
}
