//! Original and modified Anoto sequences.
//!
//! This module contains (cut-down, quasi) De Bruijn sequences used
//! in Anoto products according to Anoto patents. These sequences
//! can be used together with the encoder/decoder class to recreate
//! Anoto patterns.
//!
//! In total there are 5 different sequences required:
//!     - The Main Number Sequence (MNS)
//!     - Four secondary number sequences A1,...,A4
//!
//! Each sequence is a cut-down or quasi De Bruijn sequence meaning
//! that each substring appears _at most_ once.

/// Main number sequence.
///
/// A quasi De Bruijn sequence of order 6 and length 63. In a quasi De Bruijn
/// sequence of order n, each possible substring of length n appears _at most_
/// once.
pub const MNS: [i8; 63] = [
    0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0,
    1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1,
];

/// Secondary number sequence for the a1 coefficient.
///
/// A quasi De Bruijn sequence of order 5 and length 236.
pub const A1: [i8; 236] = [
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 0, 1, 0, 0, 2, 0, 0, 0, 1, 1, 0, 0, 0, 1, 2, 0,
    0, 1, 0, 2, 0, 0, 2, 0, 2, 0, 1, 1, 0, 1, 0, 1, 1, 0, 2, 0, 1, 2, 0, 1, 0, 1, 2, 0, 2, 1, 0, 0,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 2, 1, 0, 1, 0, 2, 1, 1, 0, 0, 1, 2, 1, 0, 1, 1, 2, 0, 0, 0, 2, 1, 0,
    2, 0, 2, 1, 1, 1, 0, 0, 2, 1, 2, 0, 1, 1, 1, 2, 0, 2, 0, 0, 1, 1, 2, 1, 0, 0, 0, 2, 2, 0, 1, 0,
    2, 2, 0, 0, 1, 2, 2, 0, 2, 0, 2, 2, 1, 0, 1, 2, 1, 2, 1, 0, 2, 1, 2, 1, 1, 0, 2, 2, 1, 2, 1, 2,
    0, 2, 2, 0, 2, 2, 2, 0, 1, 1, 2, 2, 1, 1, 0, 1, 2, 2, 2, 2, 1, 2, 0, 0, 2, 2, 1, 1, 2, 1, 2, 2,
    1, 0, 2, 2, 2, 2, 2, 0, 2, 1, 2, 2, 2, 1, 1, 1, 2, 1, 1, 2, 0, 1, 2, 2, 1, 2, 2, 0, 1, 2, 1, 1,
    1, 1, 2, 2, 2, 0, 0, 2, 1, 1, 2, 2,
];

/// Secondary number sequence for the a2 coefficient.
///
/// A quasi De Bruijn sequence of order 5 and length 233.
pub const A2: [i8; 233] = [
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1,
    0, 1, 0, 0, 2, 0, 0, 0, 1, 2, 0, 1, 0, 1, 2, 1, 0, 0, 0, 2, 1, 1, 1, 0, 1, 1, 1, 0, 2, 1, 0, 0,
    1, 2, 1, 2, 1, 0, 1, 0, 2, 0, 1, 1, 0, 2, 0, 0, 1, 0, 2, 1, 2, 0, 0, 0, 2, 2, 0, 0, 1, 1, 2, 0,
    2, 0, 0, 2, 0, 2, 0, 1, 2, 0, 0, 2, 2, 1, 1, 0, 0, 2, 1, 0, 1, 1, 2, 1, 0, 2, 0, 2, 2, 1, 0, 0,
    2, 2, 2, 1, 0, 1, 2, 2, 0, 0, 2, 1, 2, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1, 2, 2, 1, 2, 0, 1, 1, 1, 2,
    1, 1, 2, 0, 1, 2, 1, 1, 1, 2, 2, 0, 2, 2, 0, 1, 1, 2, 2, 2, 2, 1, 2, 1, 2, 2, 0, 1, 2, 2, 2, 0,
    2, 0, 2, 1, 1, 2, 2, 1, 0, 2, 2, 0, 2, 1, 0, 2, 1, 1, 0, 2, 2, 2, 2, 0, 1, 0, 2, 2, 1, 2, 2, 2,
    1, 1, 2, 1, 2, 0, 2, 2, 2,
];

/// Secondary number sequence for the a3 coefficient.
///
/// A quasi De Bruijn sequence of order 5 and length 31.
pub const A3: [i8; 31] = [
    0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1,
];

/// Original A4 sequence (not De Bruijn).
///
/// Note: This sequence has issues with duplicate substrings.
#[allow(dead_code)]
pub const A4: [i8; 241] = [
    0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 2, 0, 0, 2, 0, 1, 0, 0, 0, 1, 1, 2, 0, 0, 0, 1, 2, 0, 0, 2,
    1, 0, 0, 0, 2, 1, 1, 2, 0, 1, 0, 1, 0, 0, 1, 2, 1, 0, 0, 1, 0, 0, 2, 2, 0, 0, 0, 2, 2, 1, 0, 2,
    0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 2, 0, 1, 1, 1, 1, 0, 0, 2, 0, 2, 0, 1, 2, 0, 2, 2,
    0, 1, 0, 2, 1, 0, 1, 2, 1, 1, 0, 1, 1, 1, 2, 2, 0, 0, 1, 0, 1, 2, 2, 2, 0, 0, 2, 2, 2, 0, 1, 2,
    1, 2, 0, 2, 0, 0, 1, 2, 2, 0, 1, 1, 2, 1, 0, 2, 1, 1, 0, 2, 0, 2, 1, 2, 0, 0, 1, 1, 0, 2, 1, 2,
    1, 0, 1, 0, 2, 2, 0, 2, 1, 0, 2, 2, 1, 1, 1, 2, 0, 2, 1, 1, 1, 0, 2, 2, 2, 2, 0, 2, 0, 2, 2, 1,
    2, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 0, 0, 2, 1, 2, 2, 1, 0, 1, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2,
    2, 1, 2, 0, 1, 2, 2, 1, 2, 2, 0, 2, 2, 2, 1, 1, 1,
];

/// Alternative A4 sequence that is properly De Bruijn.
///
/// This is a corrected version that maintains the De Bruijn property.
pub const A4_ALT: [i8; 241] = [
    0, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 1, 0, 2, 2, 2, 0, 0, 2, 2, 1, 2, 0, 2, 2, 1, 1, 0, 2, 2, 1,
    0, 0, 2, 2, 0, 0, 0, 2, 1, 2, 2, 0, 2, 1, 2, 1, 0, 2, 1, 2, 0, 0, 2, 1, 1, 2, 0, 2, 1, 1, 1, 0,
    2, 1, 1, 0, 0, 2, 1, 0, 0, 0, 2, 0, 2, 2, 0, 2, 0, 2, 1, 0, 2, 0, 2, 0, 0, 2, 0, 1, 0, 0, 2, 0,
    0, 0, 0, 1, 2, 2, 2, 0, 1, 2, 2, 1, 0, 1, 2, 2, 0, 0, 1, 2, 1, 2, 0, 1, 2, 1, 1, 0, 1, 2, 1, 0,
    0, 1, 2, 0, 0, 0, 1, 1, 2, 2, 0, 1, 1, 2, 1, 0, 1, 1, 2, 0, 0, 1, 1, 1, 2, 0, 1, 1, 1, 1, 2, 2,
    2, 2, 1, 2, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 1, 2, 2, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 0, 1, 1,
    1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 2, 2, 0, 1, 0, 2, 1, 0, 1, 0, 2, 0, 0, 1, 0, 1, 2, 0, 2, 0, 1, 2,
    0, 1, 0, 1, 1, 0, 2, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1,
];
