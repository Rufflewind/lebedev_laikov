use libc;

macro_rules! ns { () => ([6, 14, 26, 38, 50, 74, 86, 110, 146, 170, 194, 230, 266, 302, 350, 434, 590, 770, 974, 1202, 1454, 1730, 2030, 2354, 2702, 3074, 3470, 3890, 4334, 4802, 5294, 5810]) }

extern {
    pub fn ld0006_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0014_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0026_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0038_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0050_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0074_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0086_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0110_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0146_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0170_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0194_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0230_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0266_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0302_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0350_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0434_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0590_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0770_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld0974_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld1202_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld1454_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld1730_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld2030_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld2354_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld2702_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld3074_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld3470_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld3890_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld4334_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld4802_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld5294_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
    pub fn ld5810_(x: *mut libc::c_double, y: *mut libc::c_double,
                   z: *mut libc::c_double, w: *mut libc::c_double,
                   n_out: *mut libc::c_int) -> libc::c_int;
}

pub unsafe fn ld(x: *mut libc::c_double, y: *mut libc::c_double,
                 z: *mut libc::c_double, w: *mut libc::c_double,
                 n: usize, n_out: *mut libc::c_int) -> libc::c_int
{
    match n {
        6 => ld0006_(x, y, z, w, n_out),
        14 => ld0014_(x, y, z, w, n_out),
        26 => ld0026_(x, y, z, w, n_out),
        38 => ld0038_(x, y, z, w, n_out),
        50 => ld0050_(x, y, z, w, n_out),
        74 => ld0074_(x, y, z, w, n_out),
        86 => ld0086_(x, y, z, w, n_out),
        110 => ld0110_(x, y, z, w, n_out),
        146 => ld0146_(x, y, z, w, n_out),
        170 => ld0170_(x, y, z, w, n_out),
        194 => ld0194_(x, y, z, w, n_out),
        230 => ld0230_(x, y, z, w, n_out),
        266 => ld0266_(x, y, z, w, n_out),
        302 => ld0302_(x, y, z, w, n_out),
        350 => ld0350_(x, y, z, w, n_out),
        434 => ld0434_(x, y, z, w, n_out),
        590 => ld0590_(x, y, z, w, n_out),
        770 => ld0770_(x, y, z, w, n_out),
        974 => ld0974_(x, y, z, w, n_out),
        1202 => ld1202_(x, y, z, w, n_out),
        1454 => ld1454_(x, y, z, w, n_out),
        1730 => ld1730_(x, y, z, w, n_out),
        2030 => ld2030_(x, y, z, w, n_out),
        2354 => ld2354_(x, y, z, w, n_out),
        2702 => ld2702_(x, y, z, w, n_out),
        3074 => ld3074_(x, y, z, w, n_out),
        3470 => ld3470_(x, y, z, w, n_out),
        3890 => ld3890_(x, y, z, w, n_out),
        4334 => ld4334_(x, y, z, w, n_out),
        4802 => ld4802_(x, y, z, w, n_out),
        5294 => ld5294_(x, y, z, w, n_out),
        5810 => ld5810_(x, y, z, w, n_out),
        _ => panic!("invalid n"),
    }
}
