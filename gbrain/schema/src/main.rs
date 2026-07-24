//! Native authoring helper for refreshing `registry.ron`. Runtime validation
//! uses the committed Wasm component exported from the library crate.

#[allow(dead_code)] // the shim crates call main_impl directly
fn main() {
    main_impl();
}

pub fn main_impl() {
    gbrain_schema::registry::serve();
}
