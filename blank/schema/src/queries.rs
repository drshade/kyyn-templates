//! Schema-authored analytical queries. Field proxies preserve the model's
//! Rust types while the builder emits kyyn-core's serializable query AST
//! (`kyyn_core::query::query`). Blank ships none — the first typed query
//! over YOUR kinds goes here.

pub fn queries() -> Vec<kyyn_core::query::QueryDecl> {
    vec![]
}
