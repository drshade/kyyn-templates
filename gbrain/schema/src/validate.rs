//! KB validation — the invariants `accept` enforces before advancing `main`.
//!
//! Everything the registry DECLARES is checked generically by
//! `kyyn_core::validate::against_registry` — routing, typed parsing
//! (malformed dates fail the parse), id/filename consistency, link policy
//! (a disallowed kind is an Error), link resolution and inline `[[…]]`
//! prose links (dangling ones Warn, declared external systems pass).
//!
//! What lives HERE is only what the registry cannot say: this KB's own
//! cross-record invariants. The starter's `blocked_by` cycle check went out
//! with the `todo` kind; there are none left for the gbrain kinds yet, so
//! validation is currently the generic layer alone. Add typed, testable
//! cross-record judgement here (e.g. a `relationship` pointing a page at
//! itself, or a `supersedes` cycle) when the need arises.

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

    /// A consistent brain: a charter, two entities, and a resolvable typed
    /// edge between them. The generic layer (routing, parsing, link policy,
    /// resolution) and this schema's declarations agree end to end.
    #[test]
    fn happy_path_is_clean() {
        let entries = vec![
            entry(
                "facts/charter.ron",
                r#"(purpose: "A brain", objectives: ["one page per entity"], notes: "")"#,
            ),
            entry(
                "facts/entities/jane-doe.ron",
                r#"(id: "jane-doe", title: "Jane Doe", type: Person, aliases: ["J. Doe"])"#,
            ),
            entry(
                "facts/entities/acme.ron",
                r#"(id: "acme", title: "Acme", type: Company)"#,
            ),
            entry(
                "facts/relationships/jane-works-at-acme.ron",
                r#"(id: "jane-works-at-acme", title: "Jane works_at Acme",
                    from: "entity:jane-doe", to: "entity:acme", verb: WorksAt)"#,
            ),
        ];
        let v = validate_entries(&entries);
        assert!(v.is_empty(), "expected clean, got: {v:?}");
    }

    /// Link policy is wired in: `relationship` endpoints allow only PAGE
    /// kinds, so a link to a real-but-non-page kind (`take:`) is a hard
    /// Error. (A link to a kind that does not exist at all — the old
    /// `todo:` this test once used — is merely "unrecognized", a Warning:
    /// policy can only judge kinds the registry knows.)
    #[test]
    fn a_disallowed_link_kind_is_an_error() {
        let entries = vec![
            entry(
                "facts/entities/acme.ron",
                r#"(id: "acme", title: "Acme", type: Company)"#,
            ),
            entry(
                "facts/relationships/bad.ron",
                r#"(id: "bad", title: "Bad", from: "take:nope", to: "entity:acme", verb: WorksAt)"#,
            ),
        ];
        let v = validate_entries(&entries);
        assert!(
            v.iter()
                .any(|x| x.severity == Severity::Error && x.message.contains("take")),
            "a disallowed link kind should Error: {v:?}"
        );
    }

    /// A malformed date fails typed parsing, surfaced through the generic layer.
    #[test]
    fn a_malformed_date_is_reported() {
        let entries = vec![entry(
            "facts/media/x.ron",
            r#"(id: "x", title: "X", type: Media, date: Some("31/07/2026"))"#,
        )];
        let v = validate_entries(&entries);
        assert!(!v.is_empty(), "malformed date should be reported: {v:?}");
    }
}
