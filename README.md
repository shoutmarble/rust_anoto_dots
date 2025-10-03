# rust_anoto_dots

Rust version of [cheind/py-microdots](https://github.com/cheind/py-microdots)

This library provides **rust_anoto_dots**, a Rust implementation for encoding and decoding 2D locations based on the [Anoto](https://www.anoto.com/cases/anoto-digital-pen/) dot pattern approach.

## Features

- **Encoding**: Generate bit-matrices for specific section coordinates
- **Decoding**: Extract position coordinates, section coordinates, and pattern rotations
- **Generalized interface**: Supports tailored coding variants (e.g. 4x4 codes)
- **High Performance**: Native Rust implementation with ndarray for efficient matrix operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_anoto_dots = "0.1.0"
```

## Example

```rust
use rust_anoto_dots::defaults;
use ndarray::s;

// Use the default embodiment with A4 sequence fixed
let codec = defaults::anoto_6x6_a4_fixed();

// Generate a bit-matrix for section (10,2)
let g = codec.encode_bitmatrix((9, 16), (10, 2));

// Decode a partial matrix
let sub = g.slice(s![3..9, 7..13, ..]).to_owned();
let pos = codec.decode_position(&sub).unwrap();
let sec = codec.decode_section(&sub, pos).unwrap();

println!("pos: {:?} sec: {:?}", pos, sec);
// Output: pos: (7, 3) sec: (10, 2)

// Decode rotation from a larger matrix
let r = g.slice(s![3..11, 7..15, ..]).to_owned();
let rot = codec.decode_rotation(&r).unwrap();
println!("rotation: {}", rot);
// Output: rotation: 0
```

## Credits

This is a Rust port of the Python library [py-microdots](https://github.com/cheind/py-microdots) by Christoph Heindl.

The implementation is based on research published in:

```
@InProceedings{cheind2023microdots,
  author="Heindl, Christoph",
  title="py-microdots: Position Encoding in the Euclidean Plane Based on the Anoto Codec",
  booktitle="Intelligent Computing. Computing Conference SAI",
  year="2023",
  publisher="Springer Nature Switzerland",
  pages="219--235",
  isbn="978-3-031-37963-5"
}
```

## License

MIT License - See LICENSE file for details.
