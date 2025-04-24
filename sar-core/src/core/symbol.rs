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

impl std::fmt::Display for SymbolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for SymbolId {
    fn from(id: u16) -> Self {
        Self(id as u32)
    }
}

#[derive(Clone, Debug)]
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
