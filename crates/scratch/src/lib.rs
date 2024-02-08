use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn path(suffix: &str) -> PathBuf {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let p = Path::new(&out_dir).join(suffix);
    let _ = fs::create_dir(&p);
    p
}
