use super::{Ident, AstNode};

impl<'p> Ident<'p> {
    pub fn as_str(&self) -> &'p str {
        self.node().text()
    }
}
