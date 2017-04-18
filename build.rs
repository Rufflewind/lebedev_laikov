extern crate gcc;

fn main() {
    gcc::compile_library("liblebedevlaikov.a", &["src/Lebedev-Laikov.c"]);
}
