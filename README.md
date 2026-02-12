# decom

Rust crate for **decom**pressing streams with an automatically-selected codec.

Compressing data is out of scope; use any of the codecs which `decom` can decode.

## Adding new codecs

- If necessary, add a dependency to handle decoding, under a new feature
- Include the new codec in `decom::Format` and its `TryFrom<&[u8; 4]>` implementation
- Update `decom::io::Decompressor` to return a valid reader
