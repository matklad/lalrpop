use string_cache::DefaultAtom as Atom;
use grammar::parse_tree::{Grammar, GrammarItem, NonterminalData, TypeRef, Alternative, ActionKind,
                          Parameter, TypeParameter, WhereClause, TypeBound, Path};

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

    for item in g.items.iter_mut() {
        match *item {
            GrammarItem::Nonterminal(ref mut nt) => {
                lower_nt(nt)
            }
            _ => ()
        }
    }
    g
}

fn lower_nt(nt: &mut NonterminalData) {
    assert!(nt.type_decl.is_none());
    nt.type_decl = Some(TypeRef::Id(Atom::from("usize")));
    for alt in nt.alternatives.iter_mut() {
        lower_alt(alt);
    }
}

fn lower_alt(nt: &mut Alternative) {
    assert!(nt.action.is_none());
    nt.action = Some(ActionKind::User("92".to_string()));
}
