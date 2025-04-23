use crate::{
    core::result::Result,
    parser::decode::{self, Compression},
};

use super::inner::{Header, Layer, Layers};

/// Parses a byte array into a Payload structure
pub fn parse(bytes: Box<[u8]>) -> Result<Payload> {
    let body = get_body(bytes)?;
    Payload::parse(&body)
}

/// Extracts and decompresses the body of the SAR file
fn get_body(mut bytes: Box<[u8]>) -> Result<Box<[u8]>> {
    let compression = decode::validate_format(&bytes)?;
    let (_, body) = bytes.split_at_mut(4);

    match compression {
        Compression::None => Ok(Box::from(<&[u8] as Into<Box<[u8]>>>::into(body))),
        Compression::Compressed => decode::decompress(body),
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
        let name = Self::parse_name(&bytes, &header)?;

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

        let name_bytes = bytes[start..]
            .chunks_exact(2)
            .take(13) // Name is at most 13 chars
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();

        Ok(name_bytes)
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
        let payload = Payload::parse(&bytes).unwrap();

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
