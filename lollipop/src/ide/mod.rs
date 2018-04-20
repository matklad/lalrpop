use parse_tree::{self, ParseTree};
use super::parse;

mod highlight;
use self::highlight::Highlights;

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
}
