use crate::{
    core::{
        result::Result,
        sa::{self, Position, SymbolArt, SymbolArtLayer},
        symbol,
    },
    parser::decode,
};

/// Parses a byte array into a Payload structure
pub fn parse(bytes: impl Into<Box<[u8]>>) -> Result<impl SymbolArt + std::fmt::Debug> {
    let body = get_body(bytes.into())?;
    Payload::parse(&body)
}

/// Extracts and decompresses the body of the SAR file
fn get_body(mut bytes: Box<[u8]>) -> Result<Box<[u8]>> {
    let compression = decode::validate_format(&bytes)?;
    let (_, body) = bytes.split_at_mut(4);

    decode::decrypt(body);
    match compression {
        decode::Compression::None => Ok(Box::from(body)),
        decode::Compression::Compressed => decode::decompress(body),
    }
}

/// Represents the main payload of a SAR file containing header, layers, and name information.
#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    /// The header containing metadata about the SAR file
    header: Header,
    /// Vector of layers that make up the SAR file content
    layers: Vec<Layer>,
    /// Name of the SAR file in UTF-16LE format (up to 13 characters)
    name: Vec<u16>,
}

impl Payload {
    /// Parses a byte slice into a Payload structure
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let header = Header::parse(&bytes[0..std::mem::size_of::<Header>()])?;
        let layers = Layers::parse(&bytes[std::mem::size_of::<Header>()..])?.into();
        let name = Self::parse_name(bytes, &header)?;

        Ok(Self {
            header,
            layers,
            name,
        })
    }

    /// Parses the name field from the byte slice
    fn parse_name(bytes: &[u8], header: &Header) -> Result<Vec<u16>> {
        let size_of_header = std::mem::size_of::<Header>();
        let size_of_layer = std::mem::size_of::<Layer>();
        let start = size_of_header + size_of_layer * header.layers() as usize;

        let name_bytes = bytes[usize::min(start, bytes.len())..]
            .chunks_exact(2)
            .take(13) // Name is at most 13 chars
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();

        Ok(name_bytes)
    }
}

const HEADER_SIZE_TEAM_FLAG: u8 = 0x40;
const HEADER_SIZE_NORMAL: u8 = 0x80;

impl SymbolArt for Payload {
    type Layer = Layer;

    fn author_id(&self) -> u32 {
        self.header.author_id
    }

    fn height(&self) -> u8 {
        match self.header.height {
            HEADER_SIZE_NORMAL => 96,
            HEADER_SIZE_TEAM_FLAG => 32,
            _ => panic!("Invalid height having: {}", self.header.height),
        }
    }

    fn width(&self) -> u8 {
        match self.header.height {
            HEADER_SIZE_NORMAL => 193,
            HEADER_SIZE_TEAM_FLAG => 32,
            _ => panic!("Invalid width having: {}", self.header.height),
        }
    }

    fn layers(&self) -> Vec<Layer> {
        self.layers.clone()
    }
}

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
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Layer {
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

// Bit masks for layer data
const LAYER_IS_HIDDEN: u32 = 0b10000000000000000000000000000000;
const MASK_SYMBOL_ID: u32 = 0b01111111111000000000000000000000;
const MASK_ALPHA: u32 = 0b00000000000111000000000000000000;
const MASK_COLOR_R: u32 = 0b00000000000000000000000000111111;
const MASK_COLOR_G: u32 = 0b00000000000000000000111111000000;
const MASK_COLOR_B: u32 = 0b00000000000000111111000000000000;

impl Layer {
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
        (layer_data & LAYER_IS_HIDDEN) != 0
    }

    /// Extracts the symbol ID from the layer data
    fn extract_symbol_id(layer_data: u32) -> u16 {
        ((layer_data & MASK_SYMBOL_ID) >> 21) as u16
    }

    /// Extracts the alpha value from the layer data
    fn extract_alpha(layer_data: u32) -> u8 {
        ((layer_data & MASK_ALPHA) >> 18) as u8
    }

    /// Extracts the red color component from the layer data
    fn extract_color_r(layer_data: u32) -> u8 {
        (layer_data & MASK_COLOR_R) as u8
    }

    /// Extracts the green color component from the layer data
    fn extract_color_g(layer_data: u32) -> u8 {
        ((layer_data & MASK_COLOR_G) >> 6) as u8
    }

    /// Extracts the blue color component from the layer data
    fn extract_color_b(layer_data: u32) -> u8 {
        ((layer_data & MASK_COLOR_B) >> 12) as u8
    }
}

/// The factor used to convert the alpha value to a 8-bit value.
/// SAR files use a 3-bit alpha value, so we need to scale it up to 8-bit
///
/// We use 37 to avoid floating point arithmetic for better performance
/// 255 / 7 = 36.4285714286
const ALPHA_FACTOR: u8 = 37;

/// The factor used to convert the color value to a 8-bit value.
/// SAR files use a 4-bit color value, so we need to scale it up to 8-bit
///
/// We use 4 to avoid floating point arithmetic for better performance
/// 255 / 63 = 4.0476190476
const COLOR_FACTOR: u8 = 4;

impl SymbolArtLayer for Layer {
    fn top_left(&self) -> Position {
        self.top_left
    }

    fn bottom_left(&self) -> Position {
        self.bottom_left
    }

    fn top_right(&self) -> Position {
        self.top_right
    }

    fn bottom_right(&self) -> Position {
        self.bottom_right
    }

    fn symbol(&self) -> symbol::Symbol {
        symbol::Symbol::new(self.symbol_id.into())
    }

    fn color(&self) -> sa::Color {
        let a = self.alpha.saturating_mul(ALPHA_FACTOR);
        let r = self.color_r.saturating_mul(COLOR_FACTOR);
        let g = self.color_g.saturating_mul(COLOR_FACTOR);
        let b = self.color_b.saturating_mul(COLOR_FACTOR);
        sa::Color::new(a, r, g, b)
    }

    fn is_hidden(&self) -> bool {
        self.is_hidden
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::sa::Position, test::RAW_FILE};

    #[test]
    fn test_get_body() {
        let bytes = Box::from(RAW_FILE);
        let body = get_body(bytes).unwrap();
        assert_eq!(body.len(), 1682);
    }

    #[test]
    fn test_parse() {
        let bytes = Box::from(RAW_FILE);
        let body = get_body(bytes).unwrap();
        let payload = Payload::parse(&body).unwrap();

        let expected_name = &[12394, 12363, 12383, 12373, 12435]; // "なかたさん"
        let expected = Payload {
            header: Header {
                author_id: 881302016,
                layers: 104,
                height: 128,
                width: 193,
                sound_effect: 3,
            },
            layers: vec![
                Layer {
                    top_left: Position { x: 0, y: 0 },
                    bottom_left: Position { x: 0, y: 0 },
                    top_right: Position { x: 0, y: 0 },
                    bottom_right: Position { x: 0, y: 0 },
                    is_hidden: false,
                    symbol_id: 0,
                    alpha: 0,
                    color_r: 0,
                    color_g: 0,
                    color_b: 0,
                };
                104
            ],
            name: expected_name.to_vec(),
        };

        assert_eq!(payload.header, expected.header);
        assert_eq!(payload.layers.len(), expected.layers.len());
        assert_eq!(payload.name, expected.name);
    }
}
