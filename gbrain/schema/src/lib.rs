//! The `gbrain` template's schema crate: five page primitives, typed
//! relationship edges, provenance-bearing takes, and the charter that governs
//! their curation. A KB's `schema/` crate is its compiled type system and
//! validation boundary; the engine runs its committed `kyyn:validator@1`
//! component before accepting any proposal.

pub mod model;
pub mod queries;
pub mod registry;
pub mod validate;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod component;
