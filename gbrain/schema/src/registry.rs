//! Registry emission — this schema's self-description, as RON.
//!
//! Structure (field lists, types, roles, storage patterns, allowed link
//! kinds) is declared here, next to the types it describes. Docs and enum
//! variants are *pulled from the compiled types instead of retyped* (via
//! `kyyn_core::schemadef`, straight from the doc comments and derives on
//! `model.rs`), and a test asserts the field lists match the real structs
//! exactly.

use kyyn_core::registry::{Affordance, FieldType, Kind, Registry, RoleDecl};
use kyyn_core::schemadef::{enum_ty, field, json_of, kind_doc, link, list, opt};

use crate::model::{
    Annotation, Charter, Concept, Entity, Media, Relationship, Take, Temporal, TimelineEntry,
};

/// Hash of this crate's schema-bearing sources (every `src/**/*.rs` plus
/// `Cargo.toml` and `build.rs`, enumerated and digested by the build script) —
/// the engine checks the committed registry against it at accept (freshness).
pub fn schema_hash() -> String {
    env!("SCHEMA_SRC_HASH").to_string()
}

/// The whole registry, ready to serialize with `kyyn_core::ronfmt::to_ron`.
pub fn registry() -> Registry {
    Registry {
        schema_hash: schema_hash(),
        kinds: vec![
            charter(),
            entity(),
            media(),
            temporal(),
            annotation(),
            concept(),
            relationship(),
            take(),
        ],
        roles: roles(),
        queries: crate::queries::queries(),
    }
}

pub fn registry_ron() -> String {
    kyyn_core::ronfmt::to_ron(&registry()).expect("registry serializes")
}

// --- per-kind declarations ------------------------------------------------

fn charter() -> Kind {
    let s = json_of::<Charter>();
    Kind {
        name: "charter".into(),
        doc: kind_doc(&s),
        storage: "facts/charter.ron".into(),
        fields: vec![
            field(&s, "purpose", FieldType::Str, Some("title")),
            field(&s, "objectives", list(FieldType::Markdown), None),
            field(&s, "notes", FieldType::Markdown, None),
        ],
    }
}

// --- gbrain-base-v2 kinds --------------------------------------------------

/// Any page kind — the endpoints a `relationship` edge or a `take` subject may
/// point at (the five primitive kinds).
const PAGE_KINDS: &[&str] = &["entity", "media", "temporal", "annotation", "concept"];

/// The `timeline` field shared by every page kind: an append-only list of
/// dated entries (gbrain's below-the-line evidence log) as a nested record —
/// `FieldType::Struct` inside a `List`.
fn timeline_ty() -> FieldType {
    let te = json_of::<TimelineEntry>();
    list(FieldType::Struct(vec![
        field(&te, "time", FieldType::Date, None),
        field(&te, "entry", FieldType::Markdown, None),
    ]))
}

fn entity() -> Kind {
    let s = json_of::<Entity>();
    Kind {
        name: "entity".into(),
        doc: kind_doc(&s),
        storage: "facts/entities/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "type", enum_ty(&s, "type"), None),
            field(&s, "subtype", opt(FieldType::Str), None),
            field(&s, "aliases", list(FieldType::Str), None),
            field(&s, "email", opt(FieldType::Str), None),
            field(&s, "location", opt(FieldType::Str), None),
            field(&s, "role", opt(FieldType::Str), None),
            field(&s, "compiled", FieldType::Markdown, None),
            field(&s, "timeline", timeline_ty(), None),
        ],
    }
}

fn media() -> Kind {
    let s = json_of::<Media>();
    Kind {
        name: "media".into(),
        doc: kind_doc(&s),
        storage: "facts/media/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "type", enum_ty(&s, "type"), None),
            field(&s, "subtype", opt(FieldType::Str), None),
            field(&s, "url", opt(FieldType::Str), None),
            field(&s, "source", opt(FieldType::Str), None),
            field(&s, "author", opt(FieldType::Str), None),
            field(&s, "date", opt(FieldType::Date), Some("date")),
            field(&s, "compiled", FieldType::Markdown, None),
            field(&s, "timeline", timeline_ty(), None),
        ],
    }
}

fn temporal() -> Kind {
    let s = json_of::<Temporal>();
    Kind {
        name: "temporal".into(),
        doc: kind_doc(&s),
        storage: "facts/temporal/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "type", enum_ty(&s, "type"), None),
            field(&s, "subtype", opt(FieldType::Str), None),
            field(&s, "date", opt(FieldType::Date), Some("date")),
            field(&s, "attendees", list(link(&["entity"])), None),
            field(&s, "duration", opt(FieldType::Str), None),
            field(&s, "location", opt(FieldType::Str), None),
            field(&s, "compiled", FieldType::Markdown, None),
            field(&s, "timeline", timeline_ty(), None),
        ],
    }
}

fn annotation() -> Kind {
    let s = json_of::<Annotation>();
    Kind {
        name: "annotation".into(),
        doc: kind_doc(&s),
        storage: "facts/atoms/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "type", enum_ty(&s, "type"), None),
            field(&s, "subtype", opt(FieldType::Str), None),
            field(&s, "confidence", opt(FieldType::Str), None),
            field(&s, "valid_from", opt(FieldType::Date), Some("date")),
            field(&s, "source", opt(FieldType::Str), None),
            field(&s, "compiled", FieldType::Markdown, None),
            field(&s, "timeline", timeline_ty(), None),
        ],
    }
}

fn concept() -> Kind {
    let s = json_of::<Concept>();
    Kind {
        name: "concept".into(),
        doc: kind_doc(&s),
        storage: "facts/concepts/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "type", enum_ty(&s, "type"), None),
            field(&s, "subtype", opt(FieldType::Str), None),
            field(&s, "tags", list(FieldType::Str), None),
            field(&s, "compiled", FieldType::Markdown, None),
            field(&s, "timeline", timeline_ty(), None),
        ],
    }
}

fn relationship() -> Kind {
    let s = json_of::<Relationship>();
    Kind {
        name: "relationship".into(),
        doc: kind_doc(&s),
        storage: "facts/relationships/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "from", link(PAGE_KINDS), None),
            field(&s, "to", link(PAGE_KINDS), None),
            field(&s, "verb", enum_ty(&s, "verb"), None),
            field(&s, "context", FieldType::Markdown, None),
        ],
    }
}

fn take() -> Kind {
    let s = json_of::<Take>();
    Kind {
        name: "take".into(),
        doc: kind_doc(&s),
        storage: "facts/takes/{id}.ron".into(),
        fields: vec![
            field(&s, "id", FieldType::Str, None),
            field(&s, "title", FieldType::Str, Some("title")),
            field(&s, "subject", link(PAGE_KINDS), None),
            field(&s, "kind", enum_ty(&s, "kind"), None),
            field(&s, "claim", FieldType::Markdown, None),
            field(&s, "confidence", opt(FieldType::Str), None),
            field(&s, "valid_from", opt(FieldType::Date), Some("date")),
            field(&s, "source", opt(FieldType::Str), None),
        ],
    }
}

// --- KB roles ----------------------------------------------------------------

/// This KB's role vocabulary — its own words, mapped onto engine affordances
/// (Registry::roles). Title names a record; date places it on the timeline.
/// (The starter's `status` badge role went out with the `todo` kind — no
/// gbrain kind has a lifecycle enum yet.)
fn roles() -> Vec<RoleDecl> {
    vec![
        RoleDecl {
            name: "title".into(),
            doc: "Names the record — lists, links, briefs.".into(),
            binds: Affordance::Title,
            variants: vec![],
        },
        RoleDecl {
            name: "date".into(),
            doc: "Places the record on the KB's timeline — sorting, recency.".into(),
            binds: Affordance::Timeline,
            variants: vec![],
        },
    ]
}

// --- drift guards ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    /// Every kind's registry field list must match the real struct exactly —
    /// a field added to model.rs without a registry entry (or vice versa)
    /// fails here.
    #[test]
    fn registry_fields_match_the_structs() {
        for (kind, schema) in [
            (charter(), json_of::<Charter>()),
            (entity(), json_of::<Entity>()),
            (media(), json_of::<Media>()),
            (temporal(), json_of::<Temporal>()),
            (annotation(), json_of::<Annotation>()),
            (concept(), json_of::<Concept>()),
            (relationship(), json_of::<Relationship>()),
            (take(), json_of::<Take>()),
        ] {
            let declared: BTreeSet<String> = kind.fields.iter().map(|f| f.name.clone()).collect();
            let actual: BTreeSet<String> = schema["properties"]
                .as_object()
                .expect("properties object")
                .keys()
                .cloned()
                .collect();
            assert_eq!(declared, actual, "field drift on kind '{}'", kind.name);
        }
    }

    /// The timeline field is a list of `{time, entry}` records on every page
    /// kind — the nested-struct shape, not an opaque Markdown blob.
    #[test]
    fn timeline_is_a_list_of_records() {
        let FieldType::List(inner) = timeline_ty() else {
            panic!("timeline is a list")
        };
        let FieldType::Struct(sub) = *inner else {
            panic!("timeline elements are records")
        };
        let names: BTreeSet<String> = sub.iter().map(|f| f.name.clone()).collect();
        assert_eq!(
            names,
            ["entry".to_string(), "time".to_string()]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn docs_and_variants_flow_from_the_types() {
        let reg = registry();

        let charter = reg.kinds.iter().find(|k| k.name == "charter").unwrap();
        assert!(
            charter.doc.contains("WHY"),
            "the charter kind's curation intent must reach the registry"
        );

        // A field's doc comment reaches the registry.
        let entity = reg.kinds.iter().find(|k| k.name == "entity").unwrap();
        let aliases = entity.fields.iter().find(|f| f.name == "aliases").unwrap();
        assert!(
            aliases.doc.contains("name variant"),
            "doc comments must reach the registry"
        );

        // The gbrain link-verb enum reaches the registry with all 14 variants.
        let rel = reg.kinds.iter().find(|k| k.name == "relationship").unwrap();
        let verb = rel.fields.iter().find(|f| f.name == "verb").unwrap();
        let FieldType::Enum(verbs) = &verb.ty else {
            panic!("verb is an enum")
        };
        assert_eq!(verbs.len(), 14, "all 14 gbrain link verbs");
    }

    /// The strict-roles gate the engine enforces at every registry load:
    /// adopted roles declared, badge variants matching exactly.
    #[test]
    fn role_mapping_is_coherent() {
        assert_eq!(registry().coherence_errors(), Vec::<String>::new());
    }

    #[test]
    fn registry_round_trips_through_ron() {
        let ron_text = registry_ron();
        let back: kyyn_core::registry::Registry =
            ron::from_str(&ron_text).expect("registry RON parses back");
        assert_eq!(back.kinds.len(), 8);
        assert_eq!(back.queries.len(), 0);
        assert_eq!(back.schema_hash, schema_hash());
    }
}
