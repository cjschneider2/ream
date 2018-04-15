
pub struct LocalFn{
    pub function_name: u32,
    pub arity: u32,
    pub label: u32,
}

pub struct LocalFnChunk{
    pub name: &'static str,
    pub local_fn: Vec<LocalFn>,
}

impl LocalFnChunk {
    pub fn new() -> LocalFnChunk {
        LocalFnChunk {
            name: "ExpT",
            local_fn: Vec::new(),
        }
    }

    pub fn push_fn( &mut self, function_name: u32, arity: u32, label: u32 ) {
        let exp = LocalFn {
            function_name,
            arity,
            label,
        };
        self.local_fn.push(exp);
    }
}
