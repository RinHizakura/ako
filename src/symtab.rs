use std::collections::HashMap;
pub struct Symbol {
    name: String,
}

pub struct Symtab {
    map: HashMap<String, Symbol>,
}

impl Symtab {
    pub fn new() -> Self {
        Symtab {
            map: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.map.clear();
    }

    pub fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.map.get(name)
    }

    pub fn define_var(&mut self, name: String) -> Option<&Symbol> {
        if let Some(sym) = self.resolve(&name) {
            return Some(sym);
        }

        todo!();
        None
    }
}
