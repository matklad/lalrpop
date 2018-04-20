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

    pub fn error<M: Into<String>>(&mut self, node: Node, message: M) {
        let message = message.into();
        let location = node.range();
        self.diagnostics.push(Diagnostic { location: Some(location), message })
    }

    pub fn file_error<M: Into<String>>(&mut self, message: M) {
        self.diagnostics.push(Diagnostic {
            location: None,
            message: message.into(),
        })
    }
}

pub struct Diagnostic {
    location: Option<TextRange>,
    message: String,
}

impl Diagnostic {
    pub fn location(&self) -> Option<TextRange> {
        self.location
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
