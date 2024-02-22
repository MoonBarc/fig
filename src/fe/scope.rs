use super::{ast::{AstNode, Statement, AstNodeKind, Reference}, symbols::{SymbolTable, Symbol}, Sp};

pub struct ScopeItem<'a> {
    name: &'a str,
    depth: usize,
    item: usize
}

pub struct Scope<'a> {
    /// A stack of ScopeItems.
    /// start = least nested item in scope
    /// end = most nested item in scope
    pub items: Vec<ScopeItem<'a>>,
    pub depth: usize
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            items: vec![],
            depth: 0
        }
    }

    fn start(&mut self) {
        self.depth += 1;
    }

    fn end(&mut self) {
        self.depth -= 1;
        // pop off all the items that were in that scope level
        // TODO: [OPT] this could be a lot faster!!!
        self.items.retain(|i| i.depth <= self.depth);
    }

    pub fn add(&mut self, item: usize, name: &'a str) {
        self.items.push(ScopeItem {
            name,
            depth: self.depth,
            item
        })
    }

    pub fn get(&self, name: &'a str) -> Option<usize> {
        for item in self.items.iter().rev() {
            if item.name == name {
                return Some(item.item)
            }
        }
        None
    }

    pub fn resolve(&mut self, syms: &mut SymbolTable<'a>, ast: &mut AstNode<'a>) {
        match &mut *ast.kind {
            AstNodeKind::Reference(ref mut r) => {
                let ra = r.clone().unwrap_str();
                let thing = self.get(ra)
                    .expect(&format!("unresolved identifier `{}`", ra));
                *r = Reference::Resolved(thing);
            },
            AstNodeKind::BinOp { a, b, .. } => { self.resolve(syms, a); self.resolve(syms, b); }
            AstNodeKind::UnOp { target, .. } => { self.resolve(syms, target); }
            AstNodeKind::Block { stmts } => { self.resolve_block(syms, stmts); }
            AstNodeKind::If { condition, body, else_body } => {
                self.resolve(syms, condition);
                self.resolve(syms, body);
                if let Some(eb) = else_body {
                    self.resolve(syms, eb);
                }
            }
            _ => { /* irrelevant! */ }
        }
    }
    
    pub fn resolve_block(&mut self, syms: &mut SymbolTable<'a>, stmts: &mut Vec<Statement<'a>>) {
        for s in stmts {
            match s {
                Statement::Declare { value, ref mut id, .. } => {
                    self.resolve(syms, value);
                    // add new thing to the thang
                    // HACK: THIS IS BUILTIN ABUSE!!!
                    let sym = syms.add(Sp::builtin(Symbol::Variable { ty: None }));

                    self.add(sym, id.clone().unwrap_str());
                    *id = Reference::Resolved(sym);
                },
                Statement::Expression(e) => self.resolve(syms, e),
                Statement::Return(e) => self.resolve(syms, e),
                Statement::Out(e) => self.resolve(syms, e),
                Statement::Error | Statement::Import { .. } => todo!(),
            }
        }
    }
}
