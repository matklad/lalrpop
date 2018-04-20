extern crate string_cache;
extern crate lalrpop;
extern crate lalrpop_util;
#[macro_use]
extern crate parse_tree;

use lalrpop_util::LrEvents;
use lalrpop_util::Symbol as GSymbol;
use parse_tree::{BottomUpBuilder, ParseTree, Symbol};

mod lollipop;
mod ast;
mod analysis;
mod lower;
pub mod ide;

mod symbols {
    symbols! {
        register
        FILE 0
        WHITESPACE 1
        TOKENS_DEF 2
        TOKEN_DEF  3
        TOKEN_RE   4
        RULE_DEF   5
        EXPR       6
        SYMBOL     7
        OP         8
        ATOM       9
        TOKEN_KW   10
        RULE_KW    11
        L_CURLY    12
        R_CURLY    13
        L_PAREN    14
        R_PAREN    15
        EQ         16
        PIPE       17
        STAR       18
        QMARK      19
        IDENT      20
        WORD       21
        REGEX      22
    }
}

pub fn parse(text: String) -> ParseTree {
    symbols::register();
    let mut builder = Builder::new(text.len());
    lollipop::fileParser::new().parse(
        &mut builder,
        &text,
    ).unwrap();
    builder.finish(text)
}


fn map_symbol(s: GSymbol) -> Symbol {
    use self::lollipop::symbols as g;
    match s {
        g::file => symbols::FILE,
        g::tokens_def => symbols::TOKENS_DEF,
        g::token_def => symbols::TOKEN_DEF,
        g::token_re => symbols::TOKEN_RE,
        g::rule_def => symbols::RULE_DEF,
        g::expr => symbols::EXPR,
        g::symbol => symbols::SYMBOL,
        g::op => symbols::OP,
        g::atom => symbols::ATOM,
        g::token_kw_t => symbols::TOKEN_KW,
        g::rule_kw_t => symbols::RULE_KW,
        g::l_curly_t => symbols::L_CURLY,
        g::r_curly_t => symbols::R_CURLY,
        g::l_paren_t => symbols::L_PAREN,
        g::r_paren_t => symbols::R_PAREN,
        g::eq_t => symbols::EQ,
        g::pipe_t => symbols::PIPE,
        g::star_t => symbols::STAR,
        g::qmark_t => symbols::QMARK,
        g::ident_t => symbols::IDENT,
        g::word_t => symbols::WORD,
        g::regex_t => symbols::REGEX,
        _ => panic!(),
    }
}

struct Builder {
    inner: BottomUpBuilder,
    stack: Vec<bool>,
    prev: usize,
    total: usize,
    done: bool,
}

impl Builder {
    fn new(total: usize) -> Builder {
        Builder {
            inner: BottomUpBuilder::new(),
            stack: Vec::new(),
            prev: 0,
            total,
            done: false,
        }
    }

    fn finish(self, text: String) -> ParseTree {
        self.inner.finish(text)
    }

    fn shift_ws(&mut self, current: usize) {
        let len = current - self.prev;
        if len != 0 {
            self.stack.push(true);
            self.inner.shift(symbols::WHITESPACE, (len as u32).into());
        }
    }
}

impl LrEvents for Builder {
    fn shift(&mut self, symbol: GSymbol, start: usize, end: usize) {
        if self.done {
            return;
        }
        let symbol = map_symbol(symbol);
        self.shift_ws(start);
        self.stack.push(false);
        self.prev = end;
        let len = end - start;
        self.inner.shift(symbol, (len as u32).into())
    }

    fn reduce(&mut self, symbol: GSymbol, mut n_symbols: usize) {
        if self.done {
            return;
        }

        let symbol = map_symbol(symbol);
        // trailing space
        if symbol == symbols::FILE {
            let total = self.total;
            self.shift_ws(total);
            self.done = true;
        }
        let mut to_reduce = 0;
        while n_symbols > 0 {
            let is_ws = self.stack.pop().unwrap();
            to_reduce += 1;
            if !is_ws {
                n_symbols -= 1;
            }
        }
        // leading space
        if symbol == symbols::FILE {
            while let Some(&is_ws) = self.stack.last() {
                if is_ws {
                    self.stack.pop().unwrap();
                    to_reduce += 1;
                }
            }
        }
        self.inner.reduce(symbol, to_reduce);
        self.stack.push(false);
    }
}

#[test]
fn test_lollipop() {
    let text = r####"
tokens {
  token_kw = 'tokens'
  rule_kw  = 'rule'
  l_curly  = '{'
  r_curly  = '}'
  l_paren  = '('
  r_paren  = ')'
  eq       = '='
  pipe     = '|'
  star     = '*'
  qmark    = '?'
  ident    = r"\w+"
  word     = r"'[^']+'"
  regex    = r##"((r"[^"]+")|(r#"("[^"#]|[^"])+"#)|(r#{2}"("#[^"#]|"[^"#]|[^"])+"#{2}))"##
}

rule file =
  tokens_def rule_def*

rule tokens_def =
  'tokens' '{' token_def* '}'

rule token_def =
  'ident' '=' token_re

rule token_re =
  'word' | 'regex'

rule rule_def =
  'rule' ident '=' expr ('|' expr)*

rule expr =
  symbol*

rule symbol =
  atom op?

rule op =
  '?' | '*'

rule atom =
  'word' | ident | '(' expr ')'
"####;

    let tree = parse(text.to_string());
    let repr = parse_tree::debug_dump(tree.root());
    println!("{}", repr);
}
