//! Default codec configurations.

use crate::anoto_sequences::*;
use crate::codec::AnotoCodec;

/// Creates the default Anoto 6x6 codec with original A4 sequence.
///
/// Note: This uses the original A4 sequence which has some De Bruijn property issues.
pub fn anoto_6x6() -> AnotoCodec {
    AnotoCodec::new(&MNS, 6, &[&A1, &A2, &A3, &A4], &[3, 3, 2, 3], (5, 58))
        .expect("Failed to create anoto_6x6 codec")
}

/// Creates the default Anoto 6x6 codec with fixed A4 sequence.
///
/// This uses the alternative A4 sequence (A4_ALT) which properly maintains
/// the De Bruijn property.
pub fn anoto_6x6_a4_fixed() -> AnotoCodec {
    AnotoCodec::new(&MNS, 6, &[&A1, &A2, &A3, &A4_ALT], &[3, 3, 2, 3], (5, 58))
        .expect("Failed to create anoto_6x6_a4_fixed codec")
}
