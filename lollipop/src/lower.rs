use string_cache::DefaultAtom as Atom;
use lalrpop::grammar::parse_tree::Span;
use lalrpop::grammar::repr as r;
use lalrpop::grammar::parse_tree as pt;
use lalrpop::collections::Map;
use super::ast;
use super::analysis::Analysis;

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

fn lower_tokenizer(a: &mut Analysis, file: ast::File) -> Option<(pt::TypeParameter, r::InternToken)> {
    None
}

//pub fn validate(mut grammar: Grammar) -> NormResult<Grammar> {
//    let mode = {
//        let mode = if let Some(enum_token) = grammar.enum_token() {
//            assert!(
//                grammar.match_token().is_none(),
//                "validator permitted both an extern/match section"
//            );
//
//            TokenMode::Extern {
//                conversions: enum_token
//                    .conversions
//                    .iter()
//                    .map(|conversion| conversion.from.clone())
//                    .collect(),
//            }
//        } else {
//            TokenMode::Internal {
//                match_block: MatchBlock::new(grammar.match_token())?,
//            }
//        };
//
//        let mut validator = Validator {
//            grammar: &grammar,
//            mode: mode,
//        };
//
//        validator.validate()?;
//
//        validator.mode
//    };
//
//    match mode {
//        TokenMode::Extern { .. } => {
//            // If using an external tokenizer, we're all done at this point.
//        }
//        TokenMode::Internal { match_block } => {
//            // Otherwise, construct the `InternToken` item.
//            construct(&mut grammar, match_block)?;
//        }
//    }
//
//    Ok(grammar)
//}
//
/////////////////////////////////////////////////////////////////////////////
//// Validation phase -- this phase walks the grammar and visits all
//// terminals. If using an external set of tokens, it checks that all
//// terminals have a defined conversion to some pattern. Otherwise,
//// it collects all terminals into the `all_literals` set for later use.
//
//struct Validator<'grammar> {
//    grammar: &'grammar Grammar,
//    mode: TokenMode,
//}
//
//enum TokenMode {
//    /// If there is an `extern { ... }` section that defines
//    /// conversions of the form `TERMINAL => PATTERN`, then this is a
//    /// set of those terminals. These are the only terminals that the
//    /// user should be using.
//    Extern { conversions: Set<TerminalString> },
//
//    /// Otherwise, we are synthesizing the tokenizer. In that case,
//    /// `match_block` summarizes the data from the `match { ... }`
//    /// section, if any. If there was no `match` section, or the
//    /// section contains a wildcard, the user can also use additional
//    /// terminals in the grammar.
//    Internal { match_block: MatchBlock },
//}
//
//impl<'grammar> Validator<'grammar> {
//    fn validate(&mut self) -> NormResult<()> {
//        for item in &self.grammar.items {
//            match *item {
//                GrammarItem::Use(..) => {}
//                GrammarItem::MatchToken(..) => {}
//                GrammarItem::ExternToken(_) => {}
//                GrammarItem::InternToken(_) => {}
//                GrammarItem::Nonterminal(ref data) => for alternative in &data.alternatives {
//                    try!(self.validate_alternative(alternative));
//                },
//            }
//        }
//        Ok(())
//    }
//
//    fn validate_alternative(&mut self, alternative: &Alternative) -> NormResult<()> {
//        assert!(alternative.condition.is_none()); // macro expansion should have removed these
//        try!(self.validate_expr(&alternative.expr));
//        Ok(())
//    }
//
//    fn validate_expr(&mut self, expr: &ExprSymbol) -> NormResult<()> {
//        for symbol in &expr.symbols {
//            try!(self.validate_symbol(symbol));
//        }
//        Ok(())
//    }
//
//    fn validate_symbol(&mut self, symbol: &Symbol) -> NormResult<()> {
//        match symbol.kind {
//            SymbolKind::Expr(ref expr) => {
//                try!(self.validate_expr(expr));
//            }
//            SymbolKind::Terminal(ref term) => {
//                try!(self.validate_terminal(symbol.span, term));
//            }
//            SymbolKind::Nonterminal(_) => {}
//            SymbolKind::Repeat(ref repeat) => {
//                try!(self.validate_symbol(&repeat.symbol));
//            }
//            SymbolKind::Choose(ref sym) | SymbolKind::Name(_, ref sym) => {
//                try!(self.validate_symbol(sym));
//            }
//            SymbolKind::Lookahead | SymbolKind::Lookbehind | SymbolKind::Error => {}
//            SymbolKind::AmbiguousId(ref id) => {
//                panic!("ambiguous id `{}` encountered after name resolution", id)
//            }
//            SymbolKind::Macro(..) => {
//                panic!("macro not removed: {:?}", symbol);
//            }
//        }
//
//        Ok(())
//    }
//
//    fn validate_terminal(&mut self, span: Span, term: &TerminalString) -> NormResult<()> {
//        match self.mode {
//            // If there is an extern token definition, validate that
//            // this terminal has a defined conversion.
//            TokenMode::Extern { ref conversions } => {
//                if !conversions.contains(term) {
//                    return_err!(
//                        span,
//                        "terminal `{}` does not have a pattern defined for it",
//                        term
//                    );
//                }
//            }
//
//            // If there is no extern token definition, then collect
//            // the terminal literals ("class", r"[a-z]+") into a set.
//            TokenMode::Internal {
//                ref mut match_block,
//            } => {
//                match *term {
//                    TerminalString::Bare(_) => assert!(
//                        match_block.match_user_names.contains(term),
//                        "bare terminal without match entry: {}",
//                        term
//                    ),
//
//                    TerminalString::Literal(ref l) => {
//                        match_block.add_literal_from_grammar(l.clone(), span)?
//                    }
//
//                    // Error is a builtin terminal that always exists
//                    TerminalString::Error => (),
//                }
//            }
//        }
//
//        Ok(())
//    }
//}
//
/////////////////////////////////////////////////////////////////////////////
//// Construction phase -- if we are constructing a tokenizer, this
//// phase builds up an internal token DFA.
//
//fn construct(grammar: &mut Grammar, match_block: MatchBlock) -> NormResult<()> {
//    let MatchBlock {
//        mut match_entries,
//        spans,
//        ..
//    } = match_block;
//
//    // Sort match entries by order of increasing precedence.
//    match_entries.sort();
//
//    // Build up two vectors, one of parsed regular expressions and
//    // one of precedences, that are parallel with `literals`.
//    let mut regexs = Vec::with_capacity(match_entries.len());
//    let mut precedences = Vec::with_capacity(match_entries.len());
//    try!({
//        for match_entry in &match_entries {
//            precedences.push(Precedence(match_entry.precedence));
//            match match_entry.match_literal {
//                TerminalLiteral::Quoted(ref s) => {
//                    regexs.push(re::parse_literal(&s));
//                }
//                TerminalLiteral::Regex(ref s) => {
//                    match re::parse_regex(&s) {
//                        Ok(regex) => regexs.push(regex),
//                        Err(error) => {
//                            let literal_span = spans[&match_entry.match_literal];
//                            // FIXME -- take offset into account for
//                            // span; this requires knowing how many #
//                            // the user used, which we do not track
//                            return_err!(literal_span, "invalid regular expression: {}", error);
//                        }
//                    }
//                }
//            }
//        }
//        Ok(())
//    });
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
