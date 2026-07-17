//! The knowledge-base schema — the single source of truth for the shape of the
//! curated KB.
//!
//! Each type derives:
//!   * `Serialize`/`Deserialize` — round-trips to/from RON on disk.
//!   * `JsonSchema` — derives the JSON Schema that becomes an MCP tool's input
//!     contract. Doc comments on fields flow through as schema `description`s,
//!     so the same comment documents the type here AND guides the agent there.
//!
//! `#[serde(default)]` means a defaulted field may be omitted from the stored
//! RON and from an agent's tool call alike.
//!
//! When you add kinds: dates are `chrono::NaiveDate` (serde reads/writes the
//! exact `YYYY-MM-DD` strings on disk); references are the universal
//! `kyyn_core::link::Link` type (`[system:]kind:id` strings on disk).

use serde::{Deserialize, Serialize};

/// The owner's own context (`kb.self`) — who this knowledge base serves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, kyyn_core::KyynKind)]
#[kyyn(kind = "self", storage = "facts/self.ron")]
pub struct SelfContext {
    #[kyyn(role = "title")]
    pub name: String,
    pub role: String,
    /// Distilled, always-current freeform context (Markdown prose) — a
    /// snapshot, not a log.
    #[serde(default)]
    #[kyyn(markdown)]
    pub notes: String,
}

/// The KB's charter — WHY this knowledge base exists. One record, written
/// early and kept current: every curation judgement should be defensible
/// against it. When the KB's purpose shifts, propose a charter update in the
/// same breath as the work that shifts it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, kyyn_core::KyynKind)]
#[kyyn(kind = "charter", storage = "facts/charter.ron")]
pub struct Charter {
    /// One-line statement of what this KB is for.
    #[kyyn(role = "title")]
    pub purpose: String,
    /// The concrete objectives — what "done" looks like, each independently
    /// checkable. Keep them few and honest.
    #[serde(default)]
    #[kyyn(markdown)]
    pub objectives: Vec<String>,
    /// Scope, period under review, ground rules, exclusions (Markdown prose).
    #[serde(default)]
    #[kyyn(markdown)]
    pub notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charter_round_trips_and_objectives_default_empty() {
        let c = Charter {
            purpose: "Track FY training sessions.".into(),
            objectives: vec!["A list of sessions with attendees".into()],
            notes: "Period: 2025-05-01 to 2026-04-30.".into(),
        };
        let ser = kyyn_core::ronfmt::to_ron(&c).unwrap();
        let back: Charter = ron::from_str(&ser).unwrap();
        assert_eq!(c, back);
        let min: Charter = ron::from_str(r#"(purpose: "P")"#).unwrap();
        assert!(min.objectives.is_empty() && min.notes.is_empty());
    }

    #[test]
    fn self_defaults_let_notes_be_omitted() {
        let s: SelfContext = ron::from_str(r#"(name: "Owner", role: "Owner")"#).unwrap();
        assert_eq!(s.notes, "");
    }
}
