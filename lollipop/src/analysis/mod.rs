mod diagnostics;
pub mod lexer;

use self::diagnostics::{DiagnosticSink, Diagnostic};
use ast;
use ast::AstNode;

pub struct Analysis {
    sink: DiagnosticSink
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {
            sink: DiagnosticSink::new(),
        }
    }

    pub fn analyse_fully(file: ast::File) -> Vec<Diagnostic> {
        let mut a = Analysis::new();
        a.start_symbol(file);
        a.lexer_dfa(file);
        a.diagnostics()
    }

    pub fn diagnostics(self) -> Vec<Diagnostic> {
        self.sink.into_diagnostics()
    }

    pub fn start_symbol<'p>(&mut self, file: ast::File<'p>) -> Option<ast::Ident<'p>> {
        if let Some(r) = file.rules().next() {
            return Some(r.name());
        }
        self.sink.file_error("no start symbol");
        None
    }

    pub fn symbols<'p>(&mut self, file: ast::File<'p>) -> Vec<&'p str> {
        let mut ret = Vec::new();
        for t in file.tokens_def().tokens() {
            ret.push(t.name().as_str());
        }

        for r in file.rules() {
            let r = r.name().as_str();
            if !r.starts_with("_") {
                ret.push(r)
            }
        }

        ret
    }
}


fn check_diagnostics(
    text: &str,
    expected: &str,
) {
    let file = ::parse(text.to_string());
    let file = ast::File::cast(file.root()).unwrap();
    let mut diagnostics = Analysis::analyse_fully(file);
    diagnostics.sort_by_key(|d| (d.range.start(), d.range.end()));
    let mut buff = String::new();
    for d in diagnostics {
        buff.push_str(&format!("{} {}: {}", d.range.start(), d.range.end(), d.message));
    }

    let actual = buff.trim();
    let expected = expected.trim();
    assert_eq!(actual, expected);
}

#[test]
fn lexer_diagnostics() {
    check_diagnostics(r#"
    tokens {
        foo = 'foo'
        re  = r"(abc)*"
        bar_re = r"("
    }

    rule start = foo
    "#, r#"
75 79: invalid regular expression: Error parsing regex near 'r"("' at character offset 2: Unclosed parenthesis.
    "#)
}
