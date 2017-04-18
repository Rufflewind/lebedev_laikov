#!/usr/bin/env python
import sys

LDNS = [6, 14, 26, 38, 50, 74, 86, 110, 146, 170, 194, 230, 266, 302, 350, 434,
        590, 770, 974, 1202, 1454, 1730, 2030, 2354, 2702, 3074, 3470, 3890,
        4334, 4802, 5294, 5810]

f = open("src/ffi.rs", "w")

f.write("""
use libc;

macro_rules! ns {{ () => ({!r}) }}
"""[1:].format(LDNS))
f.write("""

extern {
"""[1:])
for n in LDNS:
    f.write("""
    pub fn ld{:04}_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
"""[1:].format(n))
f.write("""
}

pub unsafe fn ld(x: *mut libc::c_double, y: *mut libc::c_double,
                 z: *mut libc::c_double, w: *mut libc::c_double,
                 n: usize, n_out: *mut libc::c_int) -> libc::c_int
{
    match n {
"""[1:])
for n in LDNS:
    f.write("""
        {0} => ld{0:04}_(x, y, z, w, n_out),
"""[1:].format(n))
f.write("""
        _ => panic!("invalid n"),
    }
}
"""[1:])

f.close()
