use std::collections::HashMap;
pub struct Symbol {
    pub index: u16,
}

impl Symbol {
    pub fn new(index: u16) -> Self {
        Symbol { index }
    }
}

pub struct Symtab {
    next_index: u16,
    map: HashMap<String, Symbol>,
}

impl Symtab {
    pub fn new() -> Self {
        Symtab {
            next_index: 0,
            map: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.next_index = 0;
        self.map.clear();
    }

    pub fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.map.get(name)
    }

    pub fn define_var(&mut self, name: String) -> Option<&Symbol> {
        if !self.map.contains_key(&name) {
            let sym = Symbol::new(self.next_index);
            self.map.insert(name.clone(), sym);
            self.next_index += 1;
        }
        self.resolve(&name)
    }
}
