use crate::Format;
use std::io::{Cursor, Read};

/// Decompressor which abstracts over multiple compression formats.
pub struct Decompressor<'a> {
    reader: Box<dyn Read + 'a>,
    format: Format,
}

impl std::fmt::Debug for Decompressor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decompressor")
            .field("format", &self.format)
            .finish()
    }
}

impl<'a> Decompressor<'a> {
    fn new<R: Read + 'a>(reader: R, format: Format) -> Self {
        Decompressor {
            reader: Box::new(reader),
            format,
        }
    }

    /// The format used by the decompressor.
    pub fn format(&self) -> Format {
        self.format
    }

    /// Create a new decompressor from a [Read]er.
    /// The format is automatically detected from the first few bytes of the stream.
    pub fn try_new<R: Read + 'a>(mut reader: R) -> crate::Result<Self> {
        let mut magic = [0u8; 4];
        let nbytes = reader.read(&mut magic)?;
        let prefixed = Cursor::new(magic[..nbytes].to_vec()).chain(reader);
        let format = Format::try_from(&magic)?;
        let out = match format {
            #[cfg(feature = "lz4")]
            Format::Lz4 => Self::new(lz4_flex::frame::FrameDecoder::new(prefixed), format),
            #[cfg(feature = "zstd")]
            Format::Zstd => Self::new(zstd::Decoder::new(prefixed)?, format),
            #[cfg(feature = "zlib")]
            Format::Zlib => Self::new(flate2::read::ZlibDecoder::new(prefixed), format),
            #[cfg(feature = "gzip")]
            Format::Gzip => Self::new(flate2::read::GzDecoder::new(prefixed), format),
            #[allow(unreachable_patterns)]
            _ => {
                return Err(crate::Error::UnsupportedFormat {
                    magic_bytes: magic.to_vec(),
                });
            }
        };
        Ok(out)
    }
}

impl<'a> Read for Decompressor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn check_read<R: Read>(reader: R) {
        let mut d = Decompressor::try_new(reader).unwrap();
        let mut out = Vec::new();
        d.read_to_end(&mut out).unwrap();
        assert_eq!(out, RAW);
    }

    #[cfg(feature = "lz4")]
    #[test]
    fn test_lz4() {
        check_read(LZ4);
    }

    #[cfg(feature = "zstd")]
    #[test]
    fn test_zstd() {
        check_read(ZSTD);
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_zlib() {
        check_read(ZLIB);
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn test_gzip() {
        check_read(GZIP);
    }

    #[test]
    fn test_all() {
        let (_, supported, unsupported) = supported_unsupported();
        for s in supported {
            check_read(s);
        }
        for u in unsupported {
            assert!(Decompressor::try_new(u).is_err());
        }
    }
}
