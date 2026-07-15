//! Registry emission — this schema's self-description, as RON.
//!
//! Structure (field lists, types, roles, storage patterns, allowed link
//! kinds) is declared here, next to the types it describes. Docs are
//! *pulled from the compiled types instead of retyped* (via
//! `kyyn_core::schemadef`, straight from the doc comments and derives on
//! `model.rs`), and a test asserts the field lists match the real structs
//! exactly.

use kyyn_core::registry::{Affordance, FieldType, Kind, Registry, RoleDecl};
use kyyn_core::schemadef::{field, json_of, kind_doc, list};

use crate::model::{Charter, SelfContext};

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
        kinds: vec![charter(), self_kind()],
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

fn self_kind() -> Kind {
    let s = json_of::<SelfContext>();
    Kind {
        name: "self".into(),
        doc: kind_doc(&s),
        storage: "facts/self.ron".into(),
        fields: vec![
            field(&s, "name", FieldType::Str, Some("title")),
            field(&s, "role", FieldType::Str, None),
            field(&s, "notes", FieldType::Markdown, None),
        ],
    }
}

// --- KB roles ----------------------------------------------------------------

/// This KB's role vocabulary — its own words, mapped onto engine affordances
/// (Registry::roles). Blank ships the two universal ones; declare Badge
/// roles (with toned variants) alongside the kinds that adopt them.
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
            (self_kind(), json_of::<SelfContext>()),
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

    #[test]
    fn docs_flow_from_the_types() {
        let reg = registry();
        let charter = reg.kinds.iter().find(|k| k.name == "charter").unwrap();
        assert!(
            charter.doc.contains("WHY"),
            "the charter kind's curation intent must reach the registry"
        );
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
        assert_eq!(back.kinds.len(), 2);
        assert!(back.queries.is_empty());
        assert_eq!(back.schema_hash, schema_hash());
    }
}
