mod diagnostics;
mod lexer;

use self::diagnostics::DiagnosticSink;
use super::ast;

pub struct Analysis {
    sink: DiagnosticSink
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {
            sink: DiagnosticSink::new(),
        }
    }

    pub fn start_symbol<'p>(&mut self, f: ast::File<'p>) -> Option<ast::Ident<'p>> {
        if let Some(r) = f.rules().next() {
            return Some(r.name());
        }
        self.sink.file_error("no start symbol");
        None
    }
}
