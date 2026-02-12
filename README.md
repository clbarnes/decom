# decom

Rust crate for **decom**pressing streams with an automatically-selected codec.

Compressing data is out of scope.

```rust
use std::io::Read;
use decom::io::Decompressor;

// anything readable; a file, an HTTP response etc.
let compressed = std::io::Cursor::new([
    0x04, 0x22, 0x4d, 0x18, 0x64, 0x40, 0xa7, 0x0e,
    0x00, 0x00, 0x80, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
    0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
    0x0a, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x1f, 0x28,
    0xd3,
]);

// LZ4 compression is detected from the first few bytes
if let Ok(mut decomp) = Decompressor::try_new(compressed) {
    // Read the uncompressed data
    let mut out = String::default();
    decomp.read_to_string(&mut out).unwrap();

    assert_eq!(out, "Hello, world!\n")
};
```

## Codec support

| codec | feature | notes |
| ----- | ------- | ----- |
| GZip | `gzip` | `flate2` crate with `zlib-rs` backend |
| LZ4 | `lz4` | `lz4_flex` crate |
| ZLib | `zlib` | `flate2` crate with `zlib-rs` backend |
| Zstd | `zstd` | `zstd` crate |

### Wishlist

Codecs' compressed byte streams must be self-identifying (i.e. start with "magic bytes").

- BZip2
- XZip
- Snappy
- LZMA

## Adding new codecs

- If necessary, add a dependency to handle decoding, under a new feature
  - `pub use` the dependency in `lib.rs`
- Include the new codec in `decom::Format` and its `TryFrom<&[u8; 4]>` implementation
- Update `decom::io::Decompressor` to return a valid reader
- Unit test it
  - Use the new codec to compress `data/lorem.txt`, with an appropriate file extension
  - Add this file to the `test_utils` module
  - Add it to the response of `supported_unsupported`
  - Add any individual unit tests you need
- Document the change in `README.md`
