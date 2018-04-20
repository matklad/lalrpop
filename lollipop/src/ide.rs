use parse_tree::{self, ParseTree};
use super::parse;

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
}
