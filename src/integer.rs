//! Integer operations for Anoto codec.
//!
//! This module provides number basis decomposition and Chinese Remainder Theorem
//! solving functionality needed for encoding and decoding.

use ndarray::{Array1, Array2};

/// Computes the extended Euclidean algorithm.
///
/// Returns (gcd, r, s) such that gcd = r*a + s*b.
pub fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = extended_euclid(b % a, a);

    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

/// Represents numbers in a basis defined by prime factors.
pub struct NumberBasis {
    pub upper: i64,
    pub lower: i64,
    pub bases: Array1<i64>,
    pub rbases: Array1<i64>,
    pub pfactors: Array1<i64>,
}

impl NumberBasis {
    /// Initialize basis from prime-factors.
    ///
    /// Given prime-factors p1,...,pn the integer interval [0,p1*...*pn)
    /// can be represented uniquely using n coefficients (one for each basis).
    pub fn new(pfactors: &[i64]) -> Self {
        let pfactors_arr = Array1::from_vec(pfactors.to_vec());
        let upper = pfactors.iter().product();
        let lower = 0;

        let mut p = Vec::with_capacity(pfactors.len() + 1);
        p.push(1i64);
        p.extend_from_slice(pfactors);

        let mut cumulative = 1i64;
        let mut bases_vec = Vec::with_capacity(pfactors.len());
        for &factor in pfactors {
            bases_vec.push(cumulative);
            cumulative *= factor;
        }

        let bases = Array1::from_vec(bases_vec.clone());
        let mut rbases_vec = bases_vec;
        rbases_vec.reverse();
        let rbases = Array1::from_vec(rbases_vec);

        NumberBasis {
            upper,
            lower,
            bases,
            rbases,
            pfactors: pfactors_arr,
        }
    }

    /// Returns coefficients for prime bases for each number.
    ///
    /// # Arguments
    /// * `n` - Array of numbers >= 0 and less than product of prime-factors.
    ///
    /// # Returns
    /// Array of coefficients for each number and each basis.
    pub fn project(&self, n: &Array1<i64>) -> Array2<i64> {
        let mut coeffs = Vec::new();
        let mut n_working = n.clone();

        for &b in self.rbases.iter() {
            let q = &n_working / b;
            let r = &n_working % b;
            coeffs.push(q);
            n_working = r;
        }

        coeffs.reverse();
        let n_nums = n.len();
        let n_bases = self.bases.len();
        let mut result = Array2::zeros((n_nums, n_bases));

        for (i, coeff_arr) in coeffs.iter().enumerate() {
            for (j, &val) in coeff_arr.iter().enumerate() {
                result[[j, i]] = val;
            }
        }

        result
    }

    /// Reconstruct integers from coefficients.
    ///
    /// Each number x can be represented as x = b1*c1 + b2*c2 + ... + bn*cn.
    pub fn reconstruct(&self, coeffs: &Array2<i64>) -> Array1<i64> {
        let mut result = Array1::zeros(coeffs.nrows());
        for i in 0..coeffs.nrows() {
            let mut sum = 0i64;
            for j in 0..coeffs.ncols() {
                sum += coeffs[[i, j]] * self.bases[j];
            }
            result[i] = sum;
        }
        result
    }
}

/// Chinese Remainder Theorem solver.
///
/// Solves for simultaneous congruences using the Chinese Remainder Theorem (CRT).
pub struct CRT {
    pub lengths: Array1<i64>,
    pub l: i64,
    pub qs: Array1<i64>,
    pub es: Array1<i64>,
}

impl CRT {
    /// Initialize CRT solver with lengths that must be relatively prime.
    pub fn new(lengths: &[i64]) -> Result<Self, String> {
        let lengths_arr = Array1::from_vec(lengths.to_vec());
        let l: i64 = lengths.iter().product();
        let qs = Self::compute_qs(&lengths_arr, l)?;

        let mut es = Array1::zeros(lengths.len());
        for i in 0..lengths.len() {
            es[i] = qs[i] * (l / lengths[i]);
        }

        Ok(CRT {
            lengths: lengths_arr,
            l,
            qs,
            es,
        })
    }

    /// Returns the smallest positive number solving the remainder congruences.
    ///
    /// # Arguments
    /// * `remainders` - List of remainders, ri, such that ri = x mod li where
    ///   li is the i-th list length.
    pub fn solve(&self, remainders: &[i64]) -> i64 {
        let mut sum = 0i64;
        for (i, &remainder) in remainders.iter().enumerate() {
            sum = (sum + (remainder * self.es[i]) % self.l) % self.l;
        }
        sum
    }

    fn compute_qs(lengths: &Array1<i64>, l: i64) -> Result<Array1<i64>, String> {
        let mut qs = Array1::zeros(lengths.len());
        for (i, &li) in lengths.iter().enumerate() {
            let (gcd, _, s) = extended_euclid(li, l / li);
            if gcd != 1 {
                return Err("List lengths must be relatively prime.".to_string());
            }
            // Take closest positive s
            let s_mod = ((s % li) + li) % li;
            qs[i] = s_mod;
        }
        Ok(qs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_euclid() {
        let (gcd, r, s) = extended_euclid(30, 20);
        assert_eq!(gcd, 10);
        assert_eq!(r * 30 + s * 20, gcd);
    }

    #[test]
    fn test_number_basis() {
        let nb = NumberBasis::new(&[3, 3, 2, 3]);
        assert_eq!(nb.upper, 54);

        let n = Array1::from_vec(vec![0, 1, 53]);
        let coeffs = nb.project(&n);
        assert_eq!(coeffs.nrows(), 3);
        assert_eq!(coeffs.ncols(), 4);

        let reconstructed = nb.reconstruct(&coeffs);
        assert_eq!(reconstructed, n);
    }

    #[test]
    fn test_crt() {
        let crt = CRT::new(&[236, 233, 31, 241]).unwrap();
        let result = crt.solve(&[97, 0, 3, 211]);
        // This should produce a consistent result
        assert!(result >= 0);
    }
}
