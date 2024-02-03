use super::item::Item;

pub struct ScopeItem<'a> {
    name: &'a str,
    depth: usize,
    item: Item<'a>
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

    fn add(&mut self, item: Item<'a>, name: &'a str) {
        self.items.push(ScopeItem {
            name,
            depth: self.depth,
            item
        })
    }
}
