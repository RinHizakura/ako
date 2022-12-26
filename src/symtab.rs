use std::collections::HashMap;
struct Symbol {
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

    pub fn define_var(&mut self) {
        todo!();
    }
}
