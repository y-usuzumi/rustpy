use std::env;
use std::fs::copy;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let from_path = Path::new("src/libpypy-c.so");
    let to_path = Path::new(&out_dir);
    copy(from_path, to_path);
}
