//! # rust_anoto_dots
//!
//! This library provides encoding and decoding 2D locations based on the
//! [Anoto](https://www.anoto.com/cases/anoto-digital-pen/) dot pattern approach.
//!
//! This is a Rust port of the Python library [py-microdots](https://github.com/cheind/py-microdots).
//!
//! ## Example
//!
//! ```
//! use rust_anoto_dots::defaults;
//! use ndarray::s;
//!
//! // Use the default embodiment with A4 sequence fixed
//! let codec = defaults::anoto_6x6_a4_fixed();
//!
//! // Generate a bit-matrix for section (10,2)
//! let g = codec.encode_bitmatrix((9, 16), (10, 2));
//!
//! // Decode a partial matrix
//! let sub = g.slice(s![3..9, 7..13, ..]).to_owned();
//! let pos = codec.decode_position(&sub).unwrap();
//! let sec = codec.decode_section(&sub, pos).unwrap();
//!
//! println!("pos: {:?} sec: {:?}", pos, sec);
//! // pos: (7, 3) sec: (10, 2)
//! ```

pub mod anoto_sequences;
pub mod codec;
pub mod defaults;
pub mod exceptions;
pub mod helpers;
pub mod integer;

pub use codec::AnotoCodec;
pub use exceptions::{CodecError, DecodingError};
