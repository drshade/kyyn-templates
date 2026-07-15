//! The schema binary — what the engine spawns. One RON `Request` on stdin,
//! one RON `Response` on stdout; the whole wire lives in
//! `kyyn_core::protocol`, this crate supplies only its two artifacts.

#[allow(dead_code)] // the shim crates call main_impl directly
fn main() {
    main_impl();
}

pub fn main_impl() {
    kyyn_core::protocol::serve_schema(schema_starter::registry::registry, |entries, systems| {
        schema_starter::validate::validate_entries_with(entries, systems)
    });
}
