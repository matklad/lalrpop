use std::{
    fmt
};

use parse_tree::{self, ParseTree};
use super::parse;

mod highlight;
use self::highlight::Highlights;

use parse_tree::TextRange;
use parse_tree::search::{find_leaf_at_offset, ancestors, find_covering_node};

use ast::{self, AstNode};
use analysis::Analysis;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Severity { Error, Warning }

#[derive(Serialize, Debug, Clone)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub message: String
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}: {}", self.severity, self.range.start(), self.message)?;
        Ok(())
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }.fmt(f)
    }
}

pub struct File {
    parse_tree: ParseTree
}

impl File {
    pub fn new(text: String) -> File {
        File {
            parse_tree: parse(text)
        }
    }

    pub fn syntax_tree(&self) -> String {
        parse_tree::debug_dump(self.parse_tree.root())
    }

    pub fn highlight(&self) -> Highlights {
        highlight::highlight(&self.parse_tree)
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let file = ast::File::cast(self.parse_tree.root()).unwrap();
        Analysis::analyse_fully(file)
    }

    pub fn extend_selection(&self, range: TextRange) -> Option<TextRange> {
        let root = self.parse_tree.root();
        if range.is_empty() {
            let offset = range.start();
            let mut leaf = find_leaf_at_offset(root, offset).right_biased()?;
            return Some(leaf.range());
        };
        let node = find_covering_node(root, range);

        match ancestors(node).skip_while(|n| n.range() == range).next() {
            None => None,
            Some(parent) => Some(parent.range()),
        }
    }
}
