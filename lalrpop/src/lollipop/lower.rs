use grammar::parse_tree::Span;
use grammar::repr as r;
use collections::Map;
use super::ast;

pub fn lower(file: ast::File) -> r::Grammar {
    r::Grammar {
        prefix: String::new(),
        algorithm: r::Algorithm {
            lalr: true,
            codegen: r::LrCodeGeneration::RecursiveAscent,
        },
        uses_error_recovery: false,
        start_nonterminals: Map::new(),
        uses: Vec::new(),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        where_clauses: Vec::new(),
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
            })
        ),
        module_attributes: Vec::new(),
    }
}
