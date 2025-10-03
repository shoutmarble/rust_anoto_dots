//! Anoto codec implementation for encoding and decoding dot patterns.

use ndarray::{s, Array1, Array2, Array3};

use crate::exceptions::DecodingError;
use crate::helpers;
use crate::integer::{NumberBasis, CRT};

/// Appends the first order-1 characters to make cyclic positions locatable.
fn make_cyclic(seq: &[i8], order: usize) -> Vec<i8> {
    let mut result = seq.to_vec();
    result.extend_from_slice(&seq[..order - 1]);
    result
}

/// A generalized implementation of the Anoto coding.
///
/// An instance of this struct supports encoding and decoding
/// of Anoto patterns. Given a bit-matrix of shape (M,M,2) the provided
/// methods decode:
/// - a) the position coordinate (x,y)
/// - b) the section coordinates (u,v)
/// - c) the pattern orientation
pub struct AnotoCodec {
    pub mns: Vec<i8>,
    pub mns_length: usize,
    pub mns_cyclic: Vec<i8>,
    pub mns_order: usize,
    pub sns_order: usize,
    pub sns: Vec<Vec<i8>>,
    pub sns_lengths: Vec<usize>,
    pub sns_cyclic: Vec<Vec<i8>>,
    pub num_basis: NumberBasis,
    pub crt: CRT,
    pub delta_range: (i64, i64),
}

impl AnotoCodec {
    /// Initialize the Anoto codec.
    ///
    /// # Arguments
    /// * `mns` - A binary quasi de Bruijn sequence QB(2,n,m) of order n and
    ///   length m that acts as the main number sequence (MNS).
    /// * `mns_order` - The order of the MNS.
    /// * `sns` - A list of secondary number sequences which are also
    ///   quasi de Bruijn sequences of order n-1.
    /// * `pfactors` - The sequence of prime factors to decompose difference values.
    /// * `delta_range` - A range of possible difference values (inclusive).
    pub fn new(
        mns: &[i8],
        mns_order: usize,
        sns: &[&[i8]],
        pfactors: &[i64],
        delta_range: (i64, i64),
    ) -> Result<Self, String> {
        let mns_vec = mns.to_vec();
        let mns_length = mns_vec.len();
        let mns_cyclic = make_cyclic(&mns_vec, mns_order);
        let sns_order = mns_order - 1;

        let mut sns_vecs = Vec::new();
        let mut sns_lengths = Vec::new();
        let mut sns_cyclic_vecs = Vec::new();

        for s in sns {
            let s_vec = s.to_vec();
            sns_lengths.push(s_vec.len());
            sns_cyclic_vecs.push(make_cyclic(&s_vec, sns_order));
            sns_vecs.push(s_vec);
        }

        let num_basis = NumberBasis::new(pfactors);
        let crt = CRT::new(&sns_lengths.iter().map(|&x| x as i64).collect::<Vec<_>>())?;

        Ok(AnotoCodec {
            mns: mns_vec,
            mns_length,
            mns_cyclic,
            mns_order,
            sns_order,
            sns: sns_vecs,
            sns_lengths,
            sns_cyclic: sns_cyclic_vecs,
            num_basis,
            crt,
            delta_range,
        })
    }

    /// Generates a (H,W,2) bitmatrix given section coordinates (u,v).
    ///
    /// # Arguments
    /// * `shape` - (H,W) pattern shape
    /// * `section` - section coordinates to use
    ///
    /// # Returns
    /// bits: (H,W,2) matrix of encoded position coordinates.
    pub fn encode_bitmatrix(&self, shape: (usize, usize), section: (usize, usize)) -> Array3<i8> {
        let (h, w) = shape;

        // Find nearest multiples of MNS length for ease of generation
        let mh = (self.mns_length as f64 * (h as f64 / self.mns_length as f64).ceil()) as usize;
        let mw = (self.mns_length as f64 * (w as f64 / self.mns_length as f64).ceil()) as usize;

        let mut m = Array3::zeros((mh, mw, 2));

        // x-direction
        let mut roll = section.0 % self.mns_length;

        for x in 0..mw {
            roll = self.next_roll(x, roll);
            let s = self.roll_mns(roll);

            for y in 0..mh {
                let tile_idx = y % self.mns_length;
                m[[y, x, 0]] = s[tile_idx];
            }
        }

        // y-direction
        roll = section.1 % self.mns_length;

        for y in 0..mh {
            roll = self.next_roll(y, roll);
            let s = self.roll_mns(roll);

            for x in 0..mw {
                let tile_idx = x % self.mns_length;
                m[[y, x, 1]] = s[tile_idx];
            }
        }

        m.slice(s![..h, ..w, ..]).to_owned()
    }

    /// Decodes the (N,M,2) bitmatrix into a 2D location.
    ///
    /// The location is with respect to the section tile.
    ///
    /// # Arguments
    /// * `bits` - (N,M,2) matrix of bits. N,M need to be greater than or
    ///   equal to order of MNS.
    ///
    /// # Returns
    /// 2D (x,y) location wrt to section coordinate system
    pub fn decode_position(&self, bits: &Array3<i8>) -> Result<(usize, usize), DecodingError> {
        self.assert_bitmatrix_shape(bits, None)?;

        let bits_sliced = bits
            .slice(s![..self.mns_order, ..self.mns_order, ..])
            .to_owned();

        let x_bits = bits_sliced.slice(s![.., .., 0]).t().to_owned();
        let y_bits = bits_sliced.slice(s![.., .., 1]).to_owned();

        let x = self.decode_position_along_direction(&x_bits)?;
        let y = self.decode_position_along_direction(&y_bits)?;

        Ok((x, y))
    }

    /// Computes the section coordinates from an observed bits matrix.
    ///
    /// # Arguments
    /// * `bits` - (M,M,2) matrix of observed bits
    /// * `pos` - position coordinates (x,y)
    ///
    /// # Returns
    /// section coordinates (u,v)
    pub fn decode_section(
        &self,
        bits: &Array3<i8>,
        pos: (usize, usize),
    ) -> Result<(usize, usize), DecodingError> {
        self.assert_bitmatrix_shape(bits, None)?;

        let px_seq = bits.slice(s![..self.mns_order, 0, 0]).to_owned();
        let py_seq = bits.slice(s![0, ..self.mns_order, 1]).to_owned();

        let px_mns = self.find_in_mns_cyclic(&px_seq)?;
        let py_mns = self.find_in_mns_cyclic(&py_seq)?;

        let sx = self.integrate_roll(pos.0, 0);
        let sy = self.integrate_roll(pos.1, 0);

        // Convert to signed arithmetic for proper modular math
        let u = ((px_mns as i64 - pos.1 as i64 - sx as i64).rem_euclid(self.mns_length as i64))
            as usize;
        let v = ((py_mns as i64 - pos.0 as i64 - sy as i64).rem_euclid(self.mns_length as i64))
            as usize;

        Ok((u, v))
    }

    /// Determines the rotation of pattern in 90° steps (ccw).
    ///
    /// Returns 0-3 where 0 is canonical orientation.
    pub fn decode_rotation(&self, bits: &Array3<i8>) -> Result<usize, DecodingError> {
        let m = bits.dim().0.min(bits.dim().1);
        let bits_square = bits.slice(s![..m, ..m, ..]).to_owned();

        for k in 0..4 {
            let rotbits = helpers::rot90(&bits_square, k);
            if self.check_rotation(&rotbits, m) {
                return Ok(((4 - k) % 4) as usize);
            }
        }

        Err(DecodingError::new(
            "Failed to determine pattern orientation.",
        ))
    }

    // Helper methods

    fn roll_mns(&self, roll: usize) -> Vec<i8> {
        let mut result = Vec::with_capacity(self.mns_length);
        for i in 0..self.mns_length {
            result.push(self.mns[(i + roll) % self.mns_length]);
        }
        result
    }

    fn next_roll(&self, pos: usize, prev_roll: usize) -> usize {
        if pos == 0 {
            return prev_roll;
        }
        (prev_roll + self.delta(pos - 1) as usize) % self.mns_length
    }

    fn integrate_roll(&self, pos: usize, first_roll: usize) -> usize {
        let mut r = 0;
        for i in 0..pos {
            r += self.delta(i) as usize;
        }
        (first_roll + r) % self.mns_length
    }

    fn delta(&self, pos: usize) -> i64 {
        let rs: Vec<i64> = self
            .sns_lengths
            .iter()
            .map(|&len| (pos % len) as i64)
            .collect();

        let mut coeffs = Vec::new();
        for (i, &r) in rs.iter().enumerate() {
            coeffs.push(self.sns_cyclic[i][r as usize] as i64);
        }

        let coeffs_arr = Array2::from_shape_vec((1, coeffs.len()), coeffs).unwrap();
        self.num_basis.reconstruct(&coeffs_arr)[0] + self.delta_range.0
    }

    fn decode_position_along_direction(&self, bits: &Array2<i8>) -> Result<usize, DecodingError> {
        let m = bits.nrows();

        let mut locs = Vec::new();
        for i in 0..m {
            let row = bits.slice(s![i, ..]).to_owned();
            let loc = self.find_in_mns_cyclic(&row)?;
            locs.push(loc as i64);
        }

        let locs_arr = Array1::from_vec(locs);

        let mut deltae = Vec::new();
        for i in 0..locs_arr.len() - 1 {
            let diff = locs_arr[i + 1] - locs_arr[i];
            let delta_mod =
                ((diff % self.mns_length as i64) + self.mns_length as i64) % self.mns_length as i64;
            deltae.push(delta_mod);
        }

        // Check delta range
        for &d in &deltae {
            if d < self.delta_range.0 || d > self.delta_range.1 {
                return Err(DecodingError::new(
                    "At least one delta value is not within required range",
                ));
            }
        }

        // Subtract delta_range lower bound
        let deltae: Vec<i64> = deltae.iter().map(|&d| d - self.delta_range.0).collect();
        let deltae_arr = Array1::from_vec(deltae);

        let coeffs = self.num_basis.project(&deltae_arr);

        let mut ps = Vec::new();
        for col in 0..coeffs.ncols() {
            let col_data: Vec<i8> = coeffs.column(col).iter().map(|&x| x as i8).collect();
            let pos = self.find_in_sns_cyclic(col, &col_data)?;
            ps.push(pos as i64);
        }

        let p = self.crt.solve(&ps);
        Ok(p as usize)
    }

    fn find_in_mns_cyclic(&self, seq: &Array1<i8>) -> Result<usize, DecodingError> {
        let needle: Vec<i8> = seq.iter().cloned().collect();

        for i in 0..self.mns_cyclic.len() - needle.len() + 1 {
            if self.mns_cyclic[i..i + needle.len()] == needle[..] {
                return Ok(i);
            }
        }

        Err(DecodingError::new("Failed to find partial sequence in MNS"))
    }

    fn find_in_sns_cyclic(&self, sns_idx: usize, seq: &[i8]) -> Result<usize, DecodingError> {
        let sns_cyclic = &self.sns_cyclic[sns_idx];

        for i in 0..sns_cyclic.len() - seq.len() + 1 {
            if &sns_cyclic[i..i + seq.len()] == seq {
                return Ok(i);
            }
        }

        Err(DecodingError::new(format!(
            "Failed to find partial sequence in SNS[{}]",
            sns_idx
        )))
    }

    fn check_rotation(&self, rotbits: &Array3<i8>, m: usize) -> bool {
        let mut xcol_correct = 0;
        let mut yrow_correct = 0;

        for i in 0..m {
            let xcol = rotbits.slice(s![.., i, 0]).to_owned();
            let yrow = rotbits.slice(s![i, .., 1]).to_owned();

            if self.find_in_mns_cyclic(&xcol).is_ok() {
                xcol_correct += 1;
            }
            if self.find_in_mns_cyclic(&yrow).is_ok() {
                yrow_correct += 1;
            }
        }

        xcol_correct >= m / 2 && yrow_correct >= m / 2
    }

    fn assert_bitmatrix_shape(
        &self,
        bits: &Array3<i8>,
        min_size: Option<usize>,
    ) -> Result<(), DecodingError> {
        let (n, m, c) = bits.dim();
        let min = min_size.unwrap_or(self.mns_order);

        if n < min || m < min || c != 2 {
            return Err(DecodingError::new(format!(
                "Expected at least a matrix of size ({},{},2) matrix, but got ({},{},{})",
                min, min, n, m, c
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anoto_sequences::*;

    fn create_test_codec() -> AnotoCodec {
        AnotoCodec::new(&MNS, 6, &[&A1, &A2, &A3, &A4_ALT], &[3, 3, 2, 3], (5, 58)).unwrap()
    }

    #[test]
    fn test_encode_bitmatrix() {
        let codec = create_test_codec();
        let m = codec.encode_bitmatrix((60, 60), (0, 0));
        assert_eq!(m.dim(), (60, 60, 2));
    }

    #[test]
    fn test_encode_decode_position() {
        let codec = create_test_codec();
        let m = codec.encode_bitmatrix((100, 100), (5, 10));

        for y in 0..90 {
            for x in 0..90 {
                let sub = m.slice(s![y..y + 6, x..x + 6, ..]).to_owned();
                let (decoded_x, decoded_y) = codec.decode_position(&sub).unwrap();
                assert_eq!((decoded_x, decoded_y), (x, y));
            }
        }
    }
}
