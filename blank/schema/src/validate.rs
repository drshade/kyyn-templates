//! KB validation — the invariants `accept` enforces before advancing `main`.
//!
//! Everything the registry DECLARES is checked generically by
//! `kyyn_core::validate::against_registry` — routing, typed parsing
//! (malformed dates fail the parse), id/filename consistency, link policy
//! (a disallowed kind is an Error), link resolution and inline `[[…]]`
//! prose links (dangling ones Warn, declared external systems pass).
//!
//! What lives HERE is only what the registry cannot say: this KB's own
//! cross-record invariants, as typed, testable Rust. Blank ships none —
//! add them beside the kinds that need them.
//!
//! Iterate a kind's records with
//! `kyyn_core::schema::custom_records::<MyKind>(entries)` — it routes
//! paths through the kind's own storage pattern and yields typed records
//! (`path`, `storage_id`, `value`), so a custom check never re-implements
//! routing or RON decoding by hand. It skips records that fail to
//! deserialize DELIBERATELY: the mandatory generic phase owns parse
//! findings — never report them twice. Likewise, don't re-flag what link
//! policy already rejects; judge only links whose kind the field allows.

pub use kyyn_core::violation::{Severity, Violation};

/// Systems whose links are externally owned (never locally resolvable) —
/// the namespaces the KB's `sources.ron` declares, handed in by the engine.
/// Until the manifest exists, callers pass [`DEFAULT_SYSTEMS`].
pub const DEFAULT_SYSTEMS: &[&str] = &["graph", "sharepoint"];

/// Validate every KB file. `entries` are (repo-relative path, RON text)
/// pairs; paths outside `facts/` are ignored.
pub fn validate_entries(entries: &[(String, String)]) -> Vec<Violation> {
    validate_entries_with(entries, DEFAULT_SYSTEMS)
}

/// Validate against an explicit declared-system set (what the engine calls
/// through the protocol; `validate_entries` is the DEFAULT_SYSTEMS shorthand).
pub fn validate_entries_with(entries: &[(String, String)], systems: &[&str]) -> Vec<Violation> {
    crate::registry::validate_entries_with(entries, systems)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, text: &str) -> (String, String) {
        (path.to_string(), text.to_string())
    }

    /// A fully consistent KB: no violations at all. The generic layer
    /// (routing, parsing, link policy, inline prose) and this schema's
    /// declarations agree end to end.
    #[test]
    fn happy_path_is_clean() {
        let entries = vec![
            entry(
                "facts/charter.ron",
                r#"(purpose: "Track training", objectives: ["a list of sessions"], notes: "")"#,
            ),
            entry("facts/self.ron", r#"(name: "Owner", role: "CTO")"#),
        ];
        let v = validate_entries(&entries);
        assert!(v.is_empty(), "expected clean, got: {v:?}");
    }

    /// The generic layer is wired in: shape and resolution findings arrive
    /// through validate_entries (details are pinned in kyyn-core).
    #[test]
    fn generic_checks_flow_through() {
        let entries = vec![
            entry("facts/self.ron", "(name: "),
            entry(
                "facts/charter.ron",
                r#"(purpose: "P", notes: "see [[default/self]] and [[default/todo:ghost]]")"#,
            ),
        ];
        let v = validate_entries(&entries);
        assert!(v.iter().any(|x| x.message.contains("does not parse")));
        assert!(
            v.iter()
                .any(|x| x.severity == Severity::Warning && x.message.contains("inline"))
        );
    }
}
