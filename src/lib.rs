mod error;
pub use error::{Error, Result};
pub mod io;

#[cfg(any(feature = "zlib", feature="gzip"))]
pub use flate2;
#[cfg(feature = "lz4")]
pub use lz4_flex;
#[cfg(feature = "zstd")]
pub use zstd;

/// Supported compression formats.
///
/// Use the [TryFrom] implementation to recognise the format from the first 4 bytes of a compressed stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Format {
    #[cfg(feature = "lz4")]
    Lz4,
    #[cfg(feature = "zstd")]
    Zstd,
    #[cfg(feature = "zlib")]
    Zlib,
    #[cfg(feature = "gzip")]
    Gzip,
}

impl TryFrom<&[u8; 4]> for Format {
    type Error = crate::Error;

    fn try_from(value: &[u8; 4]) -> std::result::Result<Self, Self::Error> {
        match value {
            #[cfg(feature = "lz4")]
            [0x04, 0x22, 0x4D, 0x18] => Ok(Self::Lz4),
            #[cfg(feature = "zstd")]
            [0x28, 0xB5, 0x2F, 0xFD] => Ok(Self::Zstd),
            #[cfg(feature = "zlib")]
            [0x78, 0x01, _, _]
            | [0x78, 0x5E, _, _]
            | [0x78, 0x9C, _, _]
            | [0x78, 0xDA, _, _]
            | [0x78, 0x20, _, _]
            | [0x78, 0x7D, _, _]
            | [0x78, 0xBB, _, _]
            | [0x78, 0xF9, _, _] => Ok(Self::Zlib),
            #[cfg(feature = "gzip")]
            [0x1F, 0x8B, _, _] => Ok(Self::Gzip),
            _ => Err(crate::Error::UnsupportedFormat {
                magic_bytes: value.to_vec(),
            }),
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    pub const RAW: &[u8] = include_bytes!("../data/lorem.txt");
    pub const GZIP: &[u8] = include_bytes!("../data/lorem.txt.gz");
    pub const ZLIB: &[u8] = include_bytes!("../data/lorem.txt.zz");
    pub const LZ4: &[u8] = include_bytes!("../data/lorem.txt.lz4");
    pub const ZSTD: &[u8] = include_bytes!("../data/lorem.txt.zst");

    pub fn supported_unsupported() -> (&'static [u8], Vec<&'static [u8]>, Vec<&'static [u8]>) {
        let supported = vec![
            #[cfg(feature = "lz4")]
            LZ4,
            #[cfg(feature = "zstd")]
            ZSTD,
            #[cfg(feature = "zlib")]
            ZLIB,
            #[cfg(feature = "gzip")]
            GZIP,
        ];
        let unsupported = vec![
            #[cfg(not(feature = "lz4"))]
            LZ4,
            #[cfg(not(feature = "zstd"))]
            ZSTD,
            #[cfg(not(feature = "zlib"))]
            ZLIB,
            #[cfg(not(feature = "gzip"))]
            GZIP,
        ];
        (RAW, supported, unsupported)
    }
}
