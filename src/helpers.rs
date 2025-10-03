//! Helper functions for bit manipulation and rotation.

use ndarray::{Array2, Array3};

/// Maps from displacement direction d to canonical direction c.
const NUM2DIR: [usize; 4] = [0, 3, 1, 2];

/// Reverse of NUM2DIR.
const DIR2NUM: [usize; 4] = [0, 2, 3, 1];

/// Convert bit matrix to numeric representation.
///
/// Packs the bits using little-endian order.
pub fn bits_to_num(bitmatrix: &Array3<i8>) -> Array2<u8> {
    let (h, w, _) = bitmatrix.dim();
    let mut result = Array2::zeros((h, w));

    for i in 0..h {
        for j in 0..w {
            let x = bitmatrix[[i, j, 0]];
            let y = bitmatrix[[i, j, 1]];
            result[[i, j]] = (x as u8) | ((y as u8) << 1);
        }
    }

    result
}

/// Convert numeric matrix back to bit representation.
pub fn num_to_bits(num_matrix: &Array2<u8>) -> Array3<i8> {
    let (h, w) = num_matrix.dim();
    let mut result = Array3::zeros((h, w, 2));

    for i in 0..h {
        for j in 0..w {
            let val = num_matrix[[i, j]];
            result[[i, j, 0]] = (val & 1) as i8;
            result[[i, j, 1]] = ((val >> 1) & 1) as i8;
        }
    }

    result
}

/// Simulates 90° rotation of the bitmatrix applied k-times.
///
/// When k is positive applies a counterclockwise rotation,
/// else clockwise.
pub fn rot90(bitmatrix: &Array3<i8>, k: i32) -> Array3<i8> {
    let m = bits_to_num(bitmatrix);

    // Normalize k to [0, 3]
    let k_norm = k.rem_euclid(4);

    // 1. Rotate array
    let m_rot = rotate_array(&m, k_norm);

    // 2. Change bits: under rotation, bits will be decoded differently
    let mut m_transformed = m_rot.clone();
    for i in 0..m_rot.nrows() {
        for j in 0..m_rot.ncols() {
            let x = m_rot[[i, j]] as usize;
            let d = (NUM2DIR[x] as i32 - k_norm) as usize % 4;
            m_transformed[[i, j]] = DIR2NUM[d] as u8;
        }
    }

    // 3. Convert back to bits
    num_to_bits(&m_transformed)
}

/// Rotate a 2D array by k * 90 degrees counterclockwise.
fn rotate_array(arr: &Array2<u8>, k: i32) -> Array2<u8> {
    let k_norm = k.rem_euclid(4);

    match k_norm {
        0 => arr.clone(),
        1 => {
            // 90 degrees CCW: transpose then reverse each column
            let (h, w) = arr.dim();
            let mut result = Array2::zeros((w, h));
            for i in 0..h {
                for j in 0..w {
                    result[[w - 1 - j, i]] = arr[[i, j]];
                }
            }
            result
        }
        2 => {
            // 180 degrees: reverse both dimensions
            let (h, w) = arr.dim();
            let mut result = Array2::zeros((h, w));
            for i in 0..h {
                for j in 0..w {
                    result[[h - 1 - i, w - 1 - j]] = arr[[i, j]];
                }
            }
            result
        }
        3 => {
            // 270 degrees CCW (90 CW): transpose then reverse each row
            let (h, w) = arr.dim();
            let mut result = Array2::zeros((w, h));
            for i in 0..h {
                for j in 0..w {
                    result[[j, h - 1 - i]] = arr[[i, j]];
                }
            }
            result
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_num_and_back() {
        let mut bits = Array3::zeros((2, 2, 2));
        bits[[0, 0, 0]] = 0;
        bits[[0, 0, 1]] = 0;
        bits[[0, 1, 0]] = 1;
        bits[[0, 1, 1]] = 0;
        bits[[1, 0, 0]] = 0;
        bits[[1, 0, 1]] = 1;
        bits[[1, 1, 0]] = 1;
        bits[[1, 1, 1]] = 1;

        let nums = bits_to_num(&bits);
        let reconstructed = num_to_bits(&nums);

        assert_eq!(bits, reconstructed);
    }

    #[test]
    fn test_rotate_array() {
        let mut arr = Array2::zeros((2, 2));
        arr[[0, 0]] = 1;
        arr[[0, 1]] = 2;
        arr[[1, 0]] = 3;
        arr[[1, 1]] = 4;
        // Original:
        // 1 2
        // 3 4

        // 90 CCW rotation:
        // 2 4
        // 1 3
        let rot1 = rotate_array(&arr, 1);
        assert_eq!(rot1[[0, 0]], 2);
        assert_eq!(rot1[[0, 1]], 4);
        assert_eq!(rot1[[1, 0]], 1);
        assert_eq!(rot1[[1, 1]], 3);

        // 180 rotation:
        // 4 3
        // 2 1
        let rot2 = rotate_array(&arr, 2);
        assert_eq!(rot2[[0, 0]], 4);
        assert_eq!(rot2[[0, 1]], 3);
        assert_eq!(rot2[[1, 0]], 2);
        assert_eq!(rot2[[1, 1]], 1);

        let rot4 = rotate_array(&arr, 4);
        assert_eq!(rot4, arr);
    }
}
