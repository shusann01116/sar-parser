use crate::{core::result::Result, core::sa::Position};

/// Represents the header of a SAR file containing metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    /// Author ID in big endian format
    pub(super) author_id: u32,
    /// Number of layers in the SAR file
    pub(super) layers: u8,
    /// Height of the SAR file
    pub(super) height: u8,
    /// Width of the SAR file
    pub(super) width: u8,
    /// Sound effect identifier
    pub(super) sound_effect: u8,
}

impl Header {
    /// Parses a byte slice into a Header structure
    pub(super) fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Header {
            author_id: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            layers: bytes[4],
            height: bytes[5],
            width: bytes[6],
            sound_effect: bytes[7],
        })
    }

    pub(super) fn layers(&self) -> u8 {
        self.layers
    }
}

/// Represents a collection of layers in a SAR file
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    /// Parses a byte slice into a Layers structure
    pub(super) fn parse(bytes: &[u8]) -> Result<Self> {
        let layers = bytes
            .chunks_exact(std::mem::size_of::<Layer>())
            .map(Layer::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { layers })
    }
}

impl From<Layers> for Vec<Layer> {
    fn from(layers: Layers) -> Self {
        layers.layers
    }
}

/// Represents a single layer in a SAR file
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Layer {
    /// Top-left position of the layer
    pub(super) top_left: Position,
    /// Bottom-left position of the layer
    pub(super) bottom_left: Position,
    /// Top-right position of the layer
    pub(super) top_right: Position,
    /// Bottom-right position of the layer
    pub(super) bottom_right: Position,
    /// Whether the layer is hidden
    pub(super) is_hidden: bool,
    /// Symbol ID of the layer
    pub(super) symbol_id: u16,
    /// Alpha/transparency value of the layer
    pub(super) alpha: u8,
    /// Red color component
    pub(super) color_r: u8,
    /// Green color component
    pub(super) color_g: u8,
    /// Blue color component
    pub(super) color_b: u8,
}

impl Layer {
    // Bit masks for layer data
    const LAYER_IS_HIDDEN: u32 = 0b10000000000000000000000000000000;
    const MASK_SYMBOL_ID: u32 = 0b01111111111000000000000000000000;
    const MASK_ALPHA: u32 = 0b00000000000111000000000000000000;
    const MASK_COLOR_R: u32 = 0b00000000000000000000000000111111;
    const MASK_COLOR_G: u32 = 0b00000000000000000000111111000000;
    const MASK_COLOR_B: u32 = 0b00000000000000111111000000000000;

    /// Parses a byte slice into a Layer structure
    fn parse(bytes: &[u8]) -> Result<Self> {
        let top_left = Position::parse(&bytes[0..2])?;
        let bottom_left = Position::parse(&bytes[2..4])?;
        let top_right = Position::parse(&bytes[4..6])?;
        let bottom_right = Position::parse(&bytes[6..8])?;

        let layer_data = u32::from_le_bytes(bytes[8..12].try_into().unwrap());

        Ok(Self {
            top_left,
            bottom_left,
            top_right,
            bottom_right,
            is_hidden: Self::extract_is_hidden(layer_data),
            symbol_id: Self::extract_symbol_id(layer_data),
            alpha: Self::extract_alpha(layer_data),
            color_r: Self::extract_color_r(layer_data),
            color_g: Self::extract_color_g(layer_data),
            color_b: Self::extract_color_b(layer_data),
        })
    }

    /// Extracts the hidden flag from the layer data
    fn extract_is_hidden(layer_data: u32) -> bool {
        (layer_data & Self::LAYER_IS_HIDDEN) != 0
    }

    /// Extracts the symbol ID from the layer data
    fn extract_symbol_id(layer_data: u32) -> u16 {
        ((layer_data & Self::MASK_SYMBOL_ID) >> 21) as u16
    }

    /// Extracts the alpha value from the layer data
    fn extract_alpha(layer_data: u32) -> u8 {
        ((layer_data & Self::MASK_ALPHA) >> 18) as u8
    }

    /// Extracts the red color component from the layer data
    fn extract_color_r(layer_data: u32) -> u8 {
        (layer_data & Self::MASK_COLOR_R) as u8
    }

    /// Extracts the green color component from the layer data
    fn extract_color_g(layer_data: u32) -> u8 {
        ((layer_data & Self::MASK_COLOR_G) >> 6) as u8
    }

    /// Extracts the blue color component from the layer data
    fn extract_color_b(layer_data: u32) -> u8 {
        ((layer_data & Self::MASK_COLOR_B) >> 12) as u8
    }
}

impl Position {
    /// Parses a byte slice into a Position structure
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            x: bytes[0],
            y: bytes[1],
        })
    }
}
