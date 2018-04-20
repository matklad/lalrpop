use symbols;
use parse_tree::{Node, Symbol, ParseTree, TextRange};
use visitor::{VisitorBuilder, Visit};
use visitor::process_subtree_bottom_up;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct HlTag(pub &'static str);

pub const ERROR: HlTag = HlTag("error");
pub const COMMENT: HlTag = HlTag("comment");
pub const STRING: HlTag = HlTag("string");
pub const KEYWORD: HlTag = HlTag("keyword");
pub const FUNCTION: HlTag = HlTag("function");
pub const PARAMETER: HlTag = HlTag("parameter");
pub const BUILTIN: HlTag = HlTag("builtin");
pub const ATTRIBUTE: HlTag = HlTag("attribute");
pub const LITERAL: HlTag = HlTag("literal");


pub type Highlights = Vec<(TextRange, HlTag)>;
pub type HlMap = &'static [(HlTag, &'static [Symbol])];


pub fn highlight(tree: &ParseTree) -> Highlights {
    process_subtree_bottom_up(
        tree.root(),
        visitor(&[
            (STRING, &[symbols::WORD, symbols::REGEX]),
            (KEYWORD, &[symbols::TOKEN_KW, symbols::RULE_KW]),
            (FUNCTION, &[symbols::IDENT]),
        ]),
    )
}

fn visitor<'f>(mapping: HlMap) -> VisitorBuilder<'f, Highlights, HlVisitor> {
    VisitorBuilder::new(Vec::new(), HlVisitor(mapping))
}

struct HlVisitor(HlMap);

impl<'f> Visit<'f> for HlVisitor {
    type Context = Highlights;

    fn visit(&mut self, node: Node<'f>, ctx: &mut Highlights) {
//        if node.ty() == ERROR_TY {
//            let range = if node.range().is_empty() {
//                TextRange::from_len(node.range().start(), tu(1))
//            } else {
//                node.range()
//            };
//            ctx.push((range, ERROR));
//            return;
//        }
        for &(tag, symbols) in self.0.iter() {
            for &sym in symbols.iter() {
                if node.symbol() == sym {
                    hl(node, tag, ctx);
                    return;
                }
            }
        }
    }
}

pub fn hl(node: Node, tag: HlTag, highlights: &mut Highlights) {
    highlights.push((node.range(), tag))
}

