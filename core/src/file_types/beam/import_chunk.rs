
pub struct Import {
    pub module_name: u32,
    pub function_name: u32,
    pub arity: u32,
}

pub struct ImportChunk {
    pub name: &'static str,
    pub imports: Vec<Import>,
}

impl ImportChunk {
    pub fn new() -> ImportChunk {
        ImportChunk {
            name: "ExpT",
            imports: Vec::new(),
        }
    }

    pub fn push_import( &mut self, module_name: u32, function_name: u32, arity: u32) {
        let exp = Import {
            module_name,
            function_name,
            arity,
        };
        self.imports.push(exp);
    }
}
