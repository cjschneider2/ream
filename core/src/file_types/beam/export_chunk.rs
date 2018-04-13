
pub struct Export {
    pub function_name: u32,
    pub arity: u32,
    pub label: u32,
}

pub struct ExportChunk {
    pub name: &'static str,
    pub exports: Vec<Export>,
}

impl ExportChunk {
    pub fn new() -> ExportChunk {
        ExportChunk {
            name: "ExpT",
            exports: Vec::new(),
        }
    }

    pub fn push_export( &mut self, function_name: u32, arity: u32, label: u32 ) {
        let exp = Export {
            function_name,
            arity,
            label,
        };
        self.exports.push(exp);
    }
}
