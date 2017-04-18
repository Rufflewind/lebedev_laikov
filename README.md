# `lebedev_laikov`

[Lebedev–Laikov quadrature](https://en.wikipedia.org/wiki/Lebedev_quadrature) for numerical integration in spherical coordinates.

In this scheme, surface integrals over the sphere are approximated as:

```text
∫ f(Ω) dΩ = ∫ f(θ, φ) sin(θ) dθ dφ ≈ 4 π ∑ₖ wₖ f(xₖ, yₖ, zₖ)
```

Note that the weights are normalized such that they sum to one.

## Usage

Building library requires a C compiler (but not Fortran).  It uses C source code (bundled) translated from Fortran, originally hosted on [ccl.net](http://ccl.net/cca/software/SOURCES/FORTRAN/Lebedev-Laikov-Grids/Lebedev-Laikov.F).

## Reference

V. I. Lebedev, and D. N. Laikov, “A quadrature formula for the sphere of the 131st algebraic order of accuracy,” *Doklady Mathematics*, **59** (3), 477-481 (1999).  http://rad.chem.msu.ru/~laikov/ru/DAN_366_741.pdf
