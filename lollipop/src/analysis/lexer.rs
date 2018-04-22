use string_cache::DefaultAtom as Atom;
use lalrpop::grammar::parse_tree::{MatchEntry, TerminalString, TerminalLiteral, Span};
use lalrpop::collections::{Map, Set};
use parse_tree::TextRange;

use ast::{self, AstNode};
use super::Analysis;

impl Analysis {
    pub fn match_block(&mut self, f: ast::File) -> MatchBlock {
        let mut match_block = MatchBlock::default();
        let n_tokens = f.tokens_def().tokens().count();
        for (idx, mc) in f.tokens_def().tokens().enumerate() {
            // FIXME: add `else {}` and proper precedence
            let precedence = n_tokens - idx;
            let lit = match mc.re() {
                ast::TokenRe::Word(l) => TerminalLiteral::Quoted(Atom::from(l.node().text())),
                ast::TokenRe::Regex(l) => TerminalLiteral::Regex(Atom::from(l.node().text())),
            };
            let span = as_span(mc.node().range());
            if let Err(msg) = match_block.add_match_entry(
                precedence,
                lit,
                TerminalString::Bare(Atom::from(mc.name().as_str())),
                span,
            ) {
                self.sink.error(
                    mc.node(),
                    msg,
                )
            }
        };
        match_block
    }
}

fn as_span(r: TextRange) -> Span {
    let s: u32 = r.start().into();
    let e: u32 = r.end().into();
    Span(s as usize, e as usize)
}

/// Data summarizing the `match { }` block, along with any literals we
/// scraped up.
#[derive(Default)]
pub struct MatchBlock {
    /// This map stores the `match { }` entries. If `match_catch_all`
    /// is true, then we will grow this set with "identity mappings"
    /// for new literals that we find.
    match_entries: Vec<MatchEntry>,

    /// The names of all terminals the user can legally type. If
    /// `match_catch_all` is true, then if we encounter additional
    /// terminal literals in the grammar, we will add them to this
    /// set.
    match_user_names: Set<TerminalString>,

    /// For each terminal literal that we have to match, the span
    /// where it appeared in user's source.  This can either be in the
    /// `match { }` section or else in the grammar somewhere (if added
    /// due to a catch-all, or there is no match section).
    spans: Map<TerminalLiteral, Span>,

    /// True if we should permit unrecognized literals to be used.
    catch_all: bool,
}

impl MatchBlock {
    fn add_match_entry(
        &mut self,
        match_group_precedence: usize,
        sym: TerminalLiteral,
        user_name: TerminalString,
        span: Span,
    ) -> Result<(), String> {
        if let Some(_old_span) = self.spans.insert(sym.clone(), span) {
            return Err(format!("multiple match entries for `{}`", sym));
        }

        // NB: It's legal for multiple regex to produce same terminal.
        self.match_user_names.insert(user_name.clone());

        self.match_entries.push(MatchEntry {
            precedence: match_group_precedence * 2 + sym.base_precedence(),
            match_literal: sym,
            user_name: user_name,
        });
        Ok(())
    }

//    fn add_literal_from_grammar(&mut self, sym: TerminalLiteral, span: Span) -> NormResult<()> {
//        // Already saw this literal, maybe in a match entry, maybe in the grammar.
//        if self.match_user_names
//            .contains(&TerminalString::Literal(sym.clone()))
//            {
//                return Ok(());
//            }
//
//        if !self.catch_all {
//            return_err!(
//                span,
//                "terminal `{}` does not have a match mapping defined for it",
//                sym
//            );
//        }
//
//        self.match_user_names
//            .insert(TerminalString::Literal(sym.clone()));
//
//        self.match_entries.push(MatchEntry {
//            precedence: sym.base_precedence(),
//            match_literal: sym.clone(),
//            user_name: TerminalString::Literal(sym.clone()),
//        });
//
//        self.spans.insert(sym, span);
//
//        Ok(())
//    }
}
