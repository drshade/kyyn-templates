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
//! Dates are `chrono::NaiveDate` (serde reads/writes the exact `YYYY-MM-DD`
//! strings on disk); references are the universal [`Link`] type
//! (`[system:]kind:id` strings on disk — see `link.rs`).

use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kyyn_core::link::Link;

/// The KB's charter — WHY this knowledge base exists. One record, written
/// early and kept current: every curation judgement should be defensible
/// against it. When the KB's purpose shifts, propose a charter update in the
/// same breath as the work that shifts it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Charter {
    /// One-line statement of what this KB is for.
    pub purpose: String,
    /// The concrete objectives — what "done" looks like, each independently
    /// checkable. Keep them few and honest.
    #[serde(default)]
    pub objectives: Vec<String>,
    /// Scope, period under review, ground rules, exclusions (Markdown prose).
    #[serde(default)]
    pub notes: String,
}

// ===========================================================================
// gbrain-base-v2 — the knowledge model, mapped onto kyyn kinds.
//
// gbrain-base-v2 (`src/core/schema-pack/base/gbrain-base-v2.yaml`) collapses a
// brain's page types onto five closed *primitives*; each page type inherits
// its primitive's frontmatter fields and default link verbs. We mirror that
// seam directly: one kind per primitive, a `type` enum discriminating the page
// types on it, and the primitive's fields as typed fields.
//
// Every page carries the two-layer body gbrain pages have: `compiled` (above
// the line — the always-current synthesis, rewritten on new evidence) and
// `timeline` (below the line — append-only, never rewritten), the latter a
// list of dated [`TimelineEntry`] records. Typed edges and structured claims
// are their own kinds: `relationship` (the link graph, since a kyyn `Link`
// carries no verb) and `take` (gbrain's fact/take/bet/hunch).
//
// Deferred (gbrain-runtime concerns, not schema shape): the alias query-closure
// graph over legacy type names, the v1→v2 `mapping_rules` migration engine, and
// the Postgres chunk/embedding/search layer.
// ===========================================================================

/// One dated entry in a page's append-only timeline — the below-the-line
/// evidence log gbrain generates from its event ledger, here a nested record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TimelineEntry {
    /// When it happened.
    pub time: NaiveDate,
    /// What happened — a dated, sourced note (Markdown prose).
    pub entry: String,
}

/// The page types on the **entity** primitive — the actors in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum EntityType {
    Person,
    Company,
}

/// A person or organisation — the **entity** primitive (`aliases`, `email`,
/// `location`, `role`). The most enriched page type: an entity page is a
/// briefing, not a contact card. Typed edges to other pages live on
/// [`Relationship`]; claims about it live on [`Take`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Entity {
    /// Canonical slug — the stable identity (`jane-doe`, `acme`); matches the
    /// filename. Disambiguate collisions (`david-liu-meta`).
    pub id: String,
    /// Display name.
    pub title: String,
    /// Which entity page type this is.
    pub r#type: EntityType,
    /// Finer class within the type, when it has one (company →
    /// `company`/`product`/`org`). Free-form, matching gbrain's frontmatter.
    #[serde(default)]
    pub subtype: Option<String>,
    /// Every known name variant — misspellings, maiden names, nicknames, email
    /// addresses, social handles. Dedup reads this; it never forks a new page
    /// for a name already listed here.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Primary email address.
    #[serde(default)]
    pub email: Option<String>,
    /// Where they are — city, region.
    #[serde(default)]
    pub location: Option<String>,
    /// Current role/title (person) or one-line what-they-do (company).
    #[serde(default)]
    pub role: Option<String>,
    /// Compiled truth — the always-current synthesis, rewritten when evidence
    /// changes. Read only this and you know the state of play.
    #[serde(default)]
    pub compiled: String,
    /// Timeline — append-only, dated evidence entries; never rewritten.
    #[serde(default)]
    pub timeline: Vec<TimelineEntry>,
}

/// The page types on the **media** primitive — the artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum MediaType {
    Media,
    Tweet,
    Analysis,
    Source,
    Writing,
}

/// An artifact — an article, video, tweet, essay, source document, or analysis
/// (the **media** primitive: `url`, `source`, `author`, `date`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Media {
    /// Canonical slug; matches the filename.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Which media page type this is.
    pub r#type: MediaType,
    /// Finer class within the type (media → `video`/`article`/`essay`/`book`/
    /// `podcast`/`blog`; tweet → `single`/`bundle`/`stub`).
    #[serde(default)]
    pub subtype: Option<String>,
    /// Canonical URL of the artifact.
    #[serde(default)]
    pub url: Option<String>,
    /// Where it came from — publication, platform, feed.
    #[serde(default)]
    pub source: Option<String>,
    /// Author/creator, as named at the source.
    #[serde(default)]
    pub author: Option<String>,
    /// Publication/creation date.
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// Compiled truth — the always-current synthesis.
    #[serde(default)]
    pub compiled: String,
    /// Timeline — append-only, dated evidence entries; never rewritten.
    #[serde(default)]
    pub timeline: Vec<TimelineEntry>,
}

/// The page types on the **temporal** primitive — dated events and exchanges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TemporalType {
    SocialDigest,
    Deal,
    Email,
    Slack,
    Event,
    Diary,
}

/// A dated event or exchange — a deal, email, Slack thread, calendar event,
/// diary entry, or social digest (the **temporal** primitive: `date`,
/// `attendees`, `duration`, `location`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Temporal {
    /// Canonical slug; matches the filename.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Which temporal page type this is.
    pub r#type: TemporalType,
    /// Finer class within the type (social-digest → `daily`/`monthly`).
    #[serde(default)]
    pub subtype: Option<String>,
    /// When it happened.
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// Entities present or involved — typed links to `entity` pages.
    #[serde(default)]
    pub attendees: Vec<Link>,
    /// How long it ran (free-form, e.g. `45m`).
    #[serde(default)]
    pub duration: Option<String>,
    /// Where it happened.
    #[serde(default)]
    pub location: Option<String>,
    /// Compiled truth — your analysis, not a transcript dump.
    #[serde(default)]
    pub compiled: String,
    /// Timeline — append-only, dated evidence entries; never rewritten.
    #[serde(default)]
    pub timeline: Vec<TimelineEntry>,
}

/// The page types on the **annotation** primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum AnnotationType {
    Atom,
}

/// An atom — a small, provenance-bearing unit of extracted knowledge (the
/// **annotation** primitive: `confidence`, `valid_from`, `source`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Annotation {
    /// Canonical slug; matches the filename.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Which annotation page type this is.
    pub r#type: AnnotationType,
    /// Finer class within the type (atom → `extraction`/`manual`/`lore`,
    /// gbrain's `origin`).
    #[serde(default)]
    pub subtype: Option<String>,
    /// How much to trust this — `high`/`medium`/`low` or a note.
    #[serde(default)]
    pub confidence: Option<String>,
    /// When the claim started being true.
    #[serde(default)]
    pub valid_from: Option<NaiveDate>,
    /// Where the atom came from.
    #[serde(default)]
    pub source: Option<String>,
    /// Compiled truth — the always-current synthesis.
    #[serde(default)]
    pub compiled: String,
    /// Timeline — append-only, dated evidence entries; never rewritten.
    #[serde(default)]
    pub timeline: Vec<TimelineEntry>,
}

/// The page types on the **concept** primitive — ideas and workstreams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ConceptType {
    Concept,
    Project,
    Note,
}

/// A concept, project, or note (the **concept** primitive: `tags`). Concept =
/// a framework you could teach; project = something being actively built;
/// note = the catch-all for anything not otherwise typed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Concept {
    /// Canonical slug; matches the filename.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Which concept page type this is.
    pub r#type: ConceptType,
    /// Finer class within the type, when it has one.
    #[serde(default)]
    pub subtype: Option<String>,
    /// Free-form tags for grouping and retrieval.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Compiled truth — the always-current synthesis.
    #[serde(default)]
    pub compiled: String,
    /// Timeline — append-only, dated evidence entries; never rewritten.
    #[serde(default)]
    pub timeline: Vec<TimelineEntry>,
}

/// The 14 typed link verbs of gbrain-base-v2, forward direction. Inverses
/// (`employs`, `founded_by`, `attended_by`, …) are derived on read, not stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum LinkVerb {
    PartnerOf,
    RelatesTo,
    Mentions,
    Discusses,
    Founded,
    WorksAt,
    InvestedIn,
    SourcedFrom,
    DerivedFrom,
    Supersedes,
    RedirectsTo,
    Attended,
    Authored,
    AttributedTo,
}

/// A typed edge between two pages — gbrain's link graph as first-class records.
/// A kyyn `Link` carries no verb, so the edge itself is a kind: it names the
/// two endpoints and the [`LinkVerb`] between them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Relationship {
    /// Canonical slug; matches the filename (e.g. `jane-doe-works-at-acme`).
    pub id: String,
    /// Human-readable label, e.g. "Jane Doe works_at Acme".
    pub title: String,
    /// Source page of the edge.
    pub from: Link,
    /// Target page of the edge.
    pub to: Link,
    /// The relationship verb.
    pub verb: LinkVerb,
    /// Why this edge exists / supporting context (Markdown prose).
    #[serde(default)]
    pub context: String,
}

/// The kind of a structured claim — gbrain's `takes_kinds`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TakeKind {
    /// An established, verifiable fact.
    Fact,
    /// A judgement or opinion.
    Take,
    /// A prediction about the future.
    Bet,
    /// A low-confidence guess.
    Hunch,
}

/// A structured claim about a page — gbrain's "takes" (fact / take / bet /
/// hunch), with provenance. Contradictions become data, not bugs: two takes on
/// the same subject with different values sit side by side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Take {
    /// Canonical slug; matches the filename.
    pub id: String,
    /// Short label for the claim.
    pub title: String,
    /// The page this claim is about.
    pub subject: Link,
    /// What sort of claim this is.
    pub kind: TakeKind,
    /// The claim itself (Markdown prose).
    #[serde(default)]
    pub claim: String,
    /// Confidence — `high`/`medium`/`low` or a note.
    #[serde(default)]
    pub confidence: Option<String>,
    /// When the claim started being true.
    #[serde(default)]
    pub valid_from: Option<NaiveDate>,
    /// Where the claim came from.
    #[serde(default)]
    pub source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn charter_round_trips_and_objectives_default_empty() {
        let c = Charter {
            purpose: "A gbrain-style operational knowledge brain.".into(),
            objectives: vec!["One canonical page per entity".into()],
            notes: "Make this your own.".into(),
        };
        let ser = kyyn_core::ronfmt::to_ron(&c).unwrap();
        let back: Charter = ron::from_str(&ser).unwrap();
        assert_eq!(c, back);
        let min: Charter = ron::from_str(r#"(purpose: "P")"#).unwrap();
        assert!(min.objectives.is_empty() && min.notes.is_empty());
    }

    #[test]
    fn gbrain_kinds_round_trip_through_ron() {
        // An entity with a typed `type`, aliases, and a dated timeline.
        let e = Entity {
            id: "jane-doe".into(),
            title: "Jane Doe".into(),
            r#type: EntityType::Person,
            subtype: None,
            aliases: vec!["J. Doe".into(), "jane@acme.com".into()],
            email: Some("jane@acme.com".into()),
            location: Some("SF".into()),
            role: Some("CTO, Acme".into()),
            compiled: "CTO of Acme; deep infra background.".into(),
            timeline: vec![TimelineEntry {
                time: d("2026-04-07"),
                entry: "Meeting — pushed back on pricing.".into(),
            }],
        };
        let eser = kyyn_core::ronfmt::to_ron(&e).unwrap();
        assert!(eser.contains("2026-04-07"));
        assert_eq!(e, ron::from_str::<Entity>(&eser).unwrap());

        // The timeline is a list of records, omittable when empty.
        let bare: Entity = ron::from_str(r#"(id: "acme", title: "Acme", type: Company)"#).unwrap();
        assert!(bare.timeline.is_empty());

        // A relationship carries its verb and both endpoints.
        let r = Relationship {
            id: "jane-doe-works-at-acme".into(),
            title: "Jane Doe works_at Acme".into(),
            from: Link::kb("entity", "jane-doe"),
            to: Link::kb("entity", "acme"),
            verb: LinkVerb::WorksAt,
            context: "Since 2024.".into(),
        };
        let rser = kyyn_core::ronfmt::to_ron(&r).unwrap();
        assert!(rser.contains("\"entity:jane-doe\""));
        assert_eq!(r, ron::from_str::<Relationship>(&rser).unwrap());

        // A take defaults everything but the required subject/kind.
        let t: Take = ron::from_str(
            r#"(id: "acme-cto", title: "Jane is CTO", subject: "entity:jane-doe", kind: Fact)"#,
        )
        .unwrap();
        assert_eq!(t.kind, TakeKind::Fact);
        assert_eq!(t.subject, Link::kb("entity", "jane-doe"));
        assert_eq!(t.confidence, None);
    }
}
