extern crate gcc;

fn main() {
    gcc::compile_library("libcputime.a", &["c/getCPUTime.c"]);
}