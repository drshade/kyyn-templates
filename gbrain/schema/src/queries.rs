//! Schema-authored analytical queries.
//!
//! None ship with this template yet — the mechanism stays wired (the engine
//! calls `queries()`), so add gbrain queries here as the KB grows, e.g.
//! entities grouped by `type`, or takes grouped by `kind`. The starter's
//! `open-todos-summary` query was removed with the `todo` kind.

use kyyn_core::query::QueryDecl;

pub fn queries() -> Vec<QueryDecl> {
    vec![]
}
