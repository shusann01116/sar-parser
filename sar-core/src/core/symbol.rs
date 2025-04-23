#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(u32);

impl SymbolId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Clone)]
pub struct Symbol {
    id: SymbolId,
}

impl Symbol {
    pub fn new(id: SymbolId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> SymbolId {
        self.id
    }
}
