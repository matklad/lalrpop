pub use ide::{Diagnostic, Severity};
use parse_tree::TextRange;
use parse_tree::Node;

pub struct DiagnosticSink {
    diagnostics: Vec<Diagnostic>
}

impl DiagnosticSink {
    pub fn new() -> DiagnosticSink {
        DiagnosticSink {
            diagnostics: vec![]
        }
    }

    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub fn error<M: Into<String>>(&mut self, node: Node, message: M) {
        let message = message.into();
        let location = node.range();
        self.push_err(Some(location), message);
    }

    pub fn file_error<M: Into<String>>(&mut self, message: M) {
        self.push_err(None, message.into())
    }

    fn push_err(&mut self, location: Option<TextRange>, message: String) {
        let err = Diagnostic {
            range: location.unwrap_or(TextRange::from_len(0.into(), 1.into())),
            severity: Severity::Error,
            message,
        };
        self.diagnostics.push(err)
    }
}

