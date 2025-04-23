use crate::{
    decode::{self, Compression},
    result::Result,
};

pub fn parse(bytes: Box<[u8]>) -> Result<Payload> {
    let body = get_body(bytes)?;
    Payload::parse(&body)
}

fn get_body(mut bytes: Box<[u8]>) -> Result<Box<[u8]>> {
    let compression = decode::validate_format(&bytes)?;
    let (_, body) = bytes.split_at_mut(4);
    let body: Box<[u8]> = match compression {
        Compression::None => Box::from(<&[u8] as Into<Box<[u8]>>>::into(body)),
        Compression::Compressed => decode::decompress(body)?,
    };
    Ok(body)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    header: Header,
    // Exactly the number of layers specified in the header
    layers: Vec<Layer>,
    // UTF-16LE chars. Up to 13 chars; no null byte
    name: Vec<u16>,
}

impl Payload {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let header = Header::parse(&bytes[0..std::mem::size_of::<Header>()])?;
        let layers = Layers::parse(&bytes[std::mem::size_of::<Header>()..])?.into();
        let name = Payload::parse_name(&bytes, &header)?;
        Ok(Self {
            header,
            layers,
            name,
        })
    }

    fn parse_name(bytes: &[u8], header: &Header) -> Result<Vec<u16>> {
        let size_of_header = std::mem::size_of::<Header>();
        let size_of_layer = std::mem::size_of::<Layer>();
        let start = size_of_header + size_of_layer * header.layers as usize;
        let bytes = &bytes[start..];
        let bytes = bytes
            .chunks_exact(2)
            // Name is at most 13 chars
            .take(13)
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();
        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    // big endian
    author_id: u32,
    // number of layers
    layers: u8,
    height: u8,
    width: u8,
    sound_effect: u8,
}

impl Header {
    fn parse(bytes: &[u8]) -> Result<Self> {
        let author_id = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let layers = bytes[4];
        let height = bytes[5];
        let width = bytes[6];
        let sound_effect = bytes[7];
        Ok(Header {
            author_id,
            layers,
            height,
            width,
            sound_effect,
        })
    }
}

pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    fn parse(bytes: &[u8]) -> Result<Self> {
        let layers = bytes
            .chunks_exact(std::mem::size_of::<Layer>())
            .map(Layer::parse)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { layers })
    }
}

impl Into<Vec<Layer>> for Layers {
    fn into(self) -> Vec<Layer> {
        self.layers
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    top_left: Position,
    bottom_left: Position,
    top_right: Position,
    bottom_right: Position,
    color: u16,
    symbol: u16,
    _not_used: u32,
}

impl Layer {
    fn parse(bytes: &[u8]) -> Result<Self> {
        let top_left = Position::parse(&bytes[0..2])?;
        let bottom_left = Position::parse(&bytes[2..4])?;
        let top_right = Position::parse(&bytes[4..6])?;
        let bottom_right = Position::parse(&bytes[6..8])?;
        let color = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let symbol = u16::from_le_bytes(bytes[10..12].try_into().unwrap());
        let _not_used = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        Ok(Self {
            top_left,
            bottom_left,
            top_right,
            bottom_right,
            color,
            symbol,
            _not_used,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn parse(bytes: &[u8]) -> Result<Self> {
        let x = bytes[0];
        let y = bytes[1];
        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::RAW_FILE;

    use super::*;

    #[test]
    fn test_get_body() {
        let bytes = Box::from(RAW_FILE);
        let body = get_body(bytes).unwrap();
        assert_eq!(body.len(), 1682);
    }

    #[test]
    fn test_parse() {
        // Arrange
        let bytes = Box::from(RAW_FILE);

        // Act
        let payload = parse(bytes).unwrap();

        // Assert
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
                    color: 0,
                    symbol: 0,
                    _not_used: 0,
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
