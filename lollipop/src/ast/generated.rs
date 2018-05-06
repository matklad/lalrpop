use parse_tree::Node;
use super::{AstNode, AstChildren};
use super::super::symbols::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokensDef<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenDef<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleDef<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Expr<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParenExpr<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Regex<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident<'p>(Node<'p>);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenRe<'p> {
    Word(Word<'p>),
    Regex(Regex<'p>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Atom<'p> {
    Word(Word<'p>),
    Ident(Ident<'p>),
    ParenExpr(ParenExpr<'p>),
}

impl<'p> AstNode<'p> for File<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == FILE { Some(File(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for TokensDef<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == TOKENS_DEF { Some(TokensDef(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for TokenDef<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == TOKEN_DEF { Some(TokenDef(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for RuleDef<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == RULE_DEF { Some(RuleDef(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Expr<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == EXPR { Some(Expr(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for ParenExpr<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == PAREN_EXPR { Some(ParenExpr(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Symbol<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == SYMBOL { Some(Symbol(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Op<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == OP { Some(Op(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Word<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == WORD { Some(Word(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Regex<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == REGEX { Some(Regex(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Ident<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == IDENT { Some(Ident(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for TokenRe<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if let Some(n) = Word::cast(node) { return Some(TokenRe::Word(n)); }
        if let Some(n) = Regex::cast(node) { return Some(TokenRe::Regex(n)); }
        None
    }
    fn node(self) -> Node<'p> {
        match self {
            TokenRe::Word(n) => n.node(),
            TokenRe::Regex(n) => n.node(),
        }
    }
}

impl<'p> AstNode<'p> for Atom<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if let Some(n) = Word::cast(node) { return Some(Atom::Word(n)); }
        if let Some(n) = Ident::cast(node) { return Some(Atom::Ident(n)); }
        if let Some(n) = ParenExpr::cast(node) { return Some(Atom::ParenExpr(n)); }
        None
    }
    fn node(self) -> Node<'p> {
        match self {
            Atom::Word(n) => n.node(),
            Atom::Ident(n) => n.node(),
            Atom::ParenExpr(n) => n.node(),
        }
    }
}

impl<'p> File<'p> {
    pub fn tokens_def(&self) -> TokensDef<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn rules(&self) -> AstChildren<'p, RuleDef<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> TokensDef<'p> {
    pub fn tokens(&self) -> AstChildren<'p, TokenDef<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> TokenDef<'p> {
    pub fn name(&self) -> Ident<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn re(&self) -> TokenRe<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
}
impl<'p> RuleDef<'p> {
    pub fn name(&self) -> Ident<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn alts(&self) -> AstChildren<'p, Expr<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> Expr<'p> {
    pub fn symbols(&self) -> AstChildren<'p, Symbol<'p>> {
        AstChildren::new(self.node().children())
    }
}
