//! # [Lebedev–Laikov quadrature][wiki]
//! [wiki]: https://en.wikipedia.org/wiki/Lebedev_quadrature
//!
//! Approximates surface integrals over the sphere as:
//!
//! ```text
//! ∫ f(Ω) dΩ = ∫ f(θ, φ) sin(θ) dθ dφ ≈ 4 π ∑ₖ wₖ f(xₖ, yₖ, zₖ)
//! ```
//!
//! Note that the weights are normalized such that they sum to one.
//!
//! ## Reference
//!
//! V. I. Lebedev, and D. N. Laikov, “A quadrature formula for the sphere of
//! the 131st algebraic order of accuracy,” *Doklady Mathematics*, **59** (3),
//! 477-481 (1999).  http://rad.chem.msu.ru/~laikov/ru/DAN_366_741.pdf
extern crate libc;

#[macro_use]
pub mod ffi;

/// The list of numbers of nodes supported by this quadrature scheme.
pub const NS: [usize; 32] = ns!();

/// Populate the given slices with the Lebedev–Laikov grid points (`x`, `y`,
/// and `z`) and weights `w`.  All four slices must have the same length,
/// equal to the number of nodes requested.  Node that only certain number of
/// nodes are supported (see [`NS`](constant.NS.html)).
pub fn ld(x: &mut [f64], y: &mut [f64], z: &mut [f64], w: &mut [f64]) {
    let mut n = 0;
    assert_eq!(x.len(), y.len());
    assert_eq!(x.len(), z.len());
    assert_eq!(x.len(), w.len());
    unsafe {
        ffi::ld(x.as_mut_ptr(), y.as_mut_ptr(),
                z.as_mut_ptr(), w.as_mut_ptr(),
                x.len(), &mut n);
    }
}

/// Same as [`ld`](fn.ld.html) but returns the tables as vectors.
pub fn ld_vecs(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut x = vec![0.0; n];
    let mut y = vec![0.0; n];
    let mut z = vec![0.0; n];
    let mut w = vec![0.0; n];
    ld(&mut x, &mut y, &mut z, &mut w);
    (x, y, z, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for &n in &NS {
            let (x, y, z, w) = ld_vecs(n);

            // Weights should sum to one.
            assert!((w.iter().sum::<f64>() - 1.0).abs() < 1e-13);
            for ((&x, &y), &z) in x.iter().zip(&y).zip(&z) {
                // The vectors should all be of magnitude one.
                assert!(x.powi(2) + y.powi(2) + z.powi(2) - 1.0 < 1e-15);
            }
        }
    }
}
