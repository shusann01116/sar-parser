use std::sync::Arc;

use super::result::Result;
use image::{ImageBuffer, Rgb, SubImage};

use super::result::SARError;

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
    group: SymbolGroup,
    image: Arc<SubImage<ImageBuffer<Rgb<u8>, Vec<u8>>>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolGroup {
    Letters,
    Numbers,
    Punctuations,
    FilledSymbols,
    OutlinedSymbols,
    ClassSymbols,
    LineSymbols,
    CalligraphySymbols,
    GradientSymbols,
    Patterns,
    GameSymbols,
    GamePortraits,
}

impl Symbol {
    pub fn new(id: SymbolId, group: SymbolGroup) -> Result<Self> {
        let image = todo!();
        Ok(Self { id, group, image })
    }

    pub fn id(&self) -> SymbolId {
        self.id
    }

    pub fn group(&self) -> SymbolGroup {
        self.group
    }

    pub fn image(&self) -> &SubImage<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        &self.image
    }
}
