//! The `blank` template's schema crate — the minimal governed KB. A KB's
//! `schema/` crate is the KB's *type system*: the entity shapes (with doc
//! comments carrying curation intent), and the validation function the
//! engine runs through the committed `kyyn:validator@1` component before any
//! proposal is accepted.
//!
//! Deliberately tiny: the owner (`self`) and the KB's `charter` (why it
//! exists). Everything else is yours to declare — evolve this crate by
//! proposal (the evolve-schema recipe) as reality demands kinds.

pub mod model;
pub mod queries;
pub mod registry;
pub mod validate;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod component;
