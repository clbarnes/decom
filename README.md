# decom

Rust crate for **decom**pressing streams with an automatically-selected codec.

Compressing data is out of scope.

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
- Include the new codec in `decom::Format` and its `TryFrom<&[u8; 4]>` implementation
- Update `decom::io::Decompressor` to return a valid reader
