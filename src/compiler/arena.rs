pub struct AstArena {
    nodes: Vec<Box<crate::compiler::ast::Stmt>>,
}

impl AstArena {
    pub fn new() -> Self {
        AstArena { nodes: Vec::new() }
    }

    pub fn alloc(&mut self, node: crate::compiler::ast::Stmt) -> &crate::compiler::ast::Stmt {
        let boxed = Box::new(node);
        let ptr = &*boxed as *const crate::compiler::ast::Stmt;
        self.nodes.push(boxed);
        unsafe { &*ptr }
    }
}
