use string_cache::DefaultAtom as Atom;
use grammar::parse_tree::{Grammar, GrammarItem, NonterminalData, TypeRef, Alternative, ActionKind,
                          Parameter, TypeParameter, WhereClause, TypeBound, Path, NonterminalString,
                          Symbol, SymbolKind, Visibility, MatchItem, Span, ExprSymbol};

pub fn lower(mut g: Grammar) -> Grammar {
    assert!(g.type_parameters.is_empty());
    assert!(g.where_clauses.is_empty());
    assert!(g.parameters.is_empty());
    let e = Atom::from("E");
    g.type_parameters = vec![TypeParameter::Id(e.clone())];
    let bounds = vec![TypeBound::Trait {
        forall: None,
        path: Path {
            absolute: false,
            ids: vec![Atom::from("__lalrpop_util") /* TODO: do properly */,
                      Atom::from("LrEvents")],
        },
        parameters: vec![],
    }];
    g.where_clauses = vec![WhereClause::Type { forall: None, ty: TypeRef::Id(e.clone()), bounds }];
    let ty = TypeRef::Ref {
        lifetime: None,
        mutable: true,
        referent: Box::new(TypeRef::Id(e)),
    };
    g.parameters = vec![Parameter { name: Atom::from("events"), ty }];

    let lm = lower_match(&g);

    for item in g.items.iter_mut() {
        match *item {
            GrammarItem::Nonterminal(ref mut nt) => {
                lower_nt(nt, &lm)
            }
            GrammarItem::MatchToken(ref mt) => {}
            _ => ()
        }
    }
    g.items.extend(lm.nts.into_iter().map(|nt| GrammarItem::Nonterminal(nt)));
    g
}

fn usize_tr() -> TypeRef {
    TypeRef::Id(Atom::from("usize"))
}

fn lower_nt(nt: &mut NonterminalData, lm: &LoweredMatch) {
    assert!(nt.type_decl.is_none());
    nt.type_decl = Some(usize_tr());
    for alt in nt.alternatives.iter_mut() {
        let name = if nt.tree_symbol {
            Some(&nt.name)
        } else {
            None
        };
        lower_alt(alt, name, lm);
    }
}

fn lower_alt(nt: &mut Alternative, name: Option<&NonterminalString>, lm: &LoweredMatch) {
    assert!(nt.action.is_none());
    lower_expr(&mut nt.expr, lm);
    let action = if let Some(name) = name {
        let code = r"
trait S { fn s(&self) -> usize; }
impl S for usize { fn s(&self) -> usize { *self } }
impl S for Vec<usize> { fn s(&self) -> usize { self.iter().cloned().sum() } }
macro_rules! s {
    () => { 0 };
    ($e:expr) => { $e.s() };
    ($e:expr, $($es:expr),*) => { $e.s() + s!($($es),*)}
}
";
        code.to_string() + &format!("events.reduce(symbols::{}, s!(<>)); 0", name)
    } else {
        "0".to_string()
    };
    nt.action = Some(ActionKind::User(action));
}

fn lower_expr(expr: &mut ExprSymbol, lm: &LoweredMatch) {
    for s in expr.symbols.iter_mut() {
        match s.kind {
            SymbolKind::Expr(ref mut expr) => lower_expr(expr, lm),
            SymbolKind::AmbiguousId(ref mut a) => lm.lower_symbol(a),
            SymbolKind::Macro(_) => (),
            SymbolKind::Repeat(_) => (),
            SymbolKind::Choose(_) => (),
            SymbolKind::Name(_, _) => (),

            SymbolKind::Terminal(_) |
            SymbolKind::Nonterminal(_) |
            SymbolKind::Lookahead |
            SymbolKind::Lookbehind |
            SymbolKind::Error => (),
        };
    }
}


#[test]
fn s_macro() {
    trait S { fn s(&self) -> usize; }
    impl S for usize { fn s(&self) -> usize { *self } }
    impl S for Vec<usize> { fn s(&self) -> usize { self.iter().cloned().sum() } }
    macro_rules! s {
        () => { 0 };
        ($e:expr) => { $e.s() };
        ($e:expr, $($es:expr),*) => { $e.s() + s!($($es),*)}
    }

    assert_eq!(s!(), 0);
    assert_eq!(s!(1), 1);
    assert_eq!(s!(1, 2), 3);
    assert_eq!(s!(1, 2, vec![1, 2, 3]), 9);
}

struct LoweredMatch {
    nts: Vec<NonterminalData>,
}

impl LoweredMatch {
    pub fn lower_symbol(&self, a: &mut Atom) {
        for nt in self.nts.iter() {
            let t_nt = format!("{}_t", a);
            if nt.name.to_string() == t_nt {
                *a = Atom::from(t_nt);
                return;
            }
        }
    }
}

fn lower_match(g: &Grammar) -> LoweredMatch {
    LoweredMatch {
        nts: g.items.iter()
            .filter_map(|item| match item {
                GrammarItem::MatchToken(ref mt) => Some(mt),
                _ => None,
            })
            .flat_map(|mt| mt.contents.iter())
            .flat_map(|mt| mt.items.iter())
            .map(|mi| match mi {
                MatchItem::Mapped(_, m, _) => m,
                _ => panic!("unmapped symbol: {:?}", mi),
            })
            .map(|m| {
                let name = Atom::from(format!("{}_t", m));
                NonterminalData {
                    visibility: Visibility::Priv,
                    tree_symbol: true,
                    name: NonterminalString(name.clone()),
                    annotations: Vec::new(),
                    span: Span(0, 1),
                    args: Vec::new(),
                    type_decl: Some(usize_tr()),
                    alternatives: vec![
                        Alternative {
                            span: Span(0, 1),
                            expr: ExprSymbol {
                                symbols: vec![
                                    Symbol {
                                        span: Span(0, 1),
                                        kind: SymbolKind::Lookbehind,
                                    },
                                    Symbol {
                                        span: Span(0, 1),
                                        kind: SymbolKind::AmbiguousId(Atom::from(m.to_string())),
                                    },
                                    Symbol {
                                        span: Span(0, 1),
                                        kind: SymbolKind::Lookahead,
                                    },
                                ],
                            },
                            condition: None,
                            action: Some(
                                ActionKind::User(
                                    format!("let (l, _, r) = (<>); events.shift(symbols::{}, l, r); 1", name)
                                )
                            ),
                        }
                    ],
                }
            })
            .collect()
    }
}
