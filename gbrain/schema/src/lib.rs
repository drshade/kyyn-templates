//! `schema-starter` — the starter-pack schema crate that `kyyn init` ships as the
//! template for a new KB. A KB's `schema/` crate is the KB's *type system*: the
//! entity shapes (with doc comments carrying curation intent), and the
//! validation function the engine runs — as a compiled subprocess — before any
//! proposal is accepted.
//!
//! Deliberately small and domain-neutral — the owner (`self`), the KB's
//! `charter` (why it exists), and a `todo` list — while still exercising
//! every mechanism a real schema uses: roles, typed links with allowed
//! kinds, a closed enum with toned badges, a typed query, singleton and
//! id-carrying storage, and a cross-record invariant (blocked_by cycles).
//! It doubles as the reference example until themed starter packs exist.

pub mod model;
pub mod queries;
pub mod registry;
pub mod validate;
