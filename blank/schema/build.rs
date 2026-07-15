//! Hash every schema-bearing source at compile time. The digest lands in the
//! binary via `SCHEMA_SRC_HASH`, so it describes exactly the sources this
//! binary was compiled from — the registry-freshness property accept checks.
//! Enumerated, not listed: a new source file joins the hash automatically.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

#[allow(dead_code)] // the shim crates call main_impl directly
fn main() {
    main_impl();
}

pub fn main_impl() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let mut files = rs_files_under(&root.join("src"));
    files.push(root.join("Cargo.toml"));
    files.push(root.join("build.rs"));

    // Digest keyed on the /-normalized relative path and sorted by it, so the
    // hash is byte-identical regardless of read_dir order or OS separator.
    let mut keyed: Vec<(String, PathBuf)> = files
        .into_iter()
        .map(|f| {
            let rel = f.strip_prefix(&root).expect("under manifest dir");
            let key = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            (key, f)
        })
        .collect();
    keyed.sort();

    let mut h = Sha256::new();
    for (key, f) in &keyed {
        h.update(key.as_bytes());
        h.update([0]);
        h.update(std::fs::read(f).unwrap_or_else(|e| panic!("reading {}: {e}", f.display())));
        h.update([0]);
        println!("cargo:rerun-if-changed={}", f.display());
    }
    // Directory mtime changes on add/remove, catching new/deleted files.
    println!("cargo:rerun-if-changed={}", root.join("src").display());
    println!("cargo:rustc-env=SCHEMA_SRC_HASH={:x}", h.finalize());
}

fn rs_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(rs_files_under(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}
