use string_cache::DefaultAtom as Atom;

use lalrpop::{
    collections::{Map, Set},
    grammar::parse_tree::{MatchEntry, TerminalString, TerminalLiteral, Span, InternToken},
    lexer::{
        re,
        dfa::{self, DFA, Precedence, DFAConstructionError},
        nfa::NFAConstructionError::*,
    },
};
use parse_tree::TextRange;

use ast::{self, AstNode};
use super::Analysis;

impl Analysis {
    fn match_block<'p>(&mut self, file: ast::File<'p>) -> MatchBlock<'p> {
        let mut match_block = MatchBlock::default();
        let n_tokens = file.tokens_def().tokens().count();
        for (idx, mc) in file.tokens_def().tokens().enumerate() {
            // FIXME: add `else {}` and proper precedence
            let precedence = n_tokens - idx;
            let lit = match mc.re() {
                ast::TokenRe::Word(l) => TerminalLiteral::Quoted(Atom::from(l.node().text())),
                ast::TokenRe::Regex(l) => TerminalLiteral::Regex(Atom::from(l.node().text())),
            };
            if let Err(msg) = match_block.add_match_entry(
                precedence,
                lit,
                TerminalString::Bare(Atom::from(mc.name().as_str())),
                mc.re(),
            ) {
                self.sink.error(
                    mc.node(),
                    msg,
                )
            }
        };
        match_block
    }

    pub fn lexer_dfa(&mut self, file: ast::File) -> Option<InternToken> {
        let mut proceed = true;
        let match_block = self.match_block(file);
        let MatchBlock {
            mut match_entries,
            spans,
            ..
        } = match_block;
        // Sort match entries by order of increasing precedence.
        match_entries.sort();

        // Build up two vectors, one of parsed regular expressions and
        // one of precedences, that are parallel with `literals`.
        let mut regexs = Vec::with_capacity(match_entries.len());
        let mut precedences = Vec::with_capacity(match_entries.len());
        for match_entry in &match_entries {
            precedences.push(Precedence(match_entry.precedence));
            match match_entry.match_literal {
                TerminalLiteral::Quoted(ref s) => {
                    regexs.push(re::parse_literal(&s));
                }
                TerminalLiteral::Regex(ref s) => {
                    match re::parse_regex(&s) {
                        Ok(regex) => regexs.push(regex),
                        Err(error) => {
                            let node = spans[&match_entry.match_literal].node();
                            proceed = false;
                            self.sink.error(
                                node,
                                format!("invalid regular expression: {}", error),
                            );
                        }
                    }
                }
            }
        }

        if !proceed {
            return None;
        }

        let dfa = match dfa::build_dfa(&regexs, &precedences) {
            Ok(dfa) => dfa,
            Err(DFAConstructionError::NFAConstructionError { index, error }) => {
                let feature = match error {
                    NamedCaptures => r#"named captures (`(?P<foo>...)`)"#,
                    NonGreedy => r#""non-greedy" repetitions (`*?` or `+?`)"#,
                    WordBoundary => r#"word boundaries (`\b` or `\B`)"#,
                    LineBoundary => r#"line boundaries (`^` or `$`)"#,
                    TextBoundary => r#"text boundaries (`^` or `$`)"#,
                    ByteRegex => r#"byte-based matches"#,
                };
                let literal = &match_entries[index.index()].match_literal;
                let node = spans[&literal].node();
                self.sink.error(
                    node,
                    format!("{} are not supported in regular expressions", feature),
                );
                return None;
            }
            Err(DFAConstructionError::Ambiguity { match0, match1 }) => {
                let literal0 = &match_entries[match0.index()].match_literal;
                let literal1 = &match_entries[match1.index()].match_literal;
                let node = spans[&literal0].node();
                self.sink.error(
                    node,
                    format!(
                        "ambiguity detected between the terminal `{}` and the terminal `{}`",
                        literal0,
                        literal1
                    ),
                );
                return None;
            }
        };

        Some(InternToken { dfa, match_entries })
    }
}

/// Data summarizing the `match { }` block, along with any literals we
/// scraped up.
#[derive(Default)]
pub struct MatchBlock<'p> {
    /// This map stores the `match { }` entries. If `match_catch_all`
    /// is true, then we will grow this set with "identity mappings"
    /// for new literals that we find.
    pub match_entries: Vec<MatchEntry>,

    pub spans: Map<TerminalLiteral, ast::TokenRe<'p>>,
}

impl<'p> MatchBlock<'p> {
    fn add_match_entry(
        &mut self,
        match_group_precedence: usize,
        sym: TerminalLiteral,
        user_name: TerminalString,
        re: ast::TokenRe<'p>,
    ) -> Result<(), String> {
        if let Some(_old_span) = self.spans.insert(sym.clone(), re) {
            return Err("duplicate match entries".to_string());
        }

        self.match_entries.push(MatchEntry {
            precedence: match_group_precedence * 2 + sym.base_precedence(),
            match_literal: sym,
            user_name: user_name,
        });
        Ok(())
    }
}
