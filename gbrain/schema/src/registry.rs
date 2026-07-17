//! Generated registry assembly. Record structure lives once, on the Rust
//! types in `model.rs`; this module retains only KB-wide role policy.

use kyyn_core::registry::{Affordance, RoleDecl};

use crate::model::{Annotation, Charter, Concept, Entity, Media, Relationship, Take, Temporal};

kyyn_core::kyyn_schema! {
    kinds: [Charter, Entity, Media, Temporal, Annotation, Concept, Relationship, Take],
    roles: roles(),
    queries: crate::queries::queries(),
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    /// Fingerprint of the reviewed Registry projection with only `schema_hash`
    /// cleared. Intentional schema evolution updates this only after reviewing
    /// the generated Registry diff.
    const REVIEWED_REGISTRY_SHA256: &str =
        "dcaf548185901a9a0e7ea7169f87fecf37c4033995337e9b88976186219b0d9b";

    #[test]
    fn generated_registry_matches_the_reviewed_projection() {
        let mut generated = registry();
        generated.schema_hash.clear();
        let canonical = kyyn_core::ronfmt::to_ron(&generated).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(canonical.as_bytes())),
            REVIEWED_REGISTRY_SHA256
        );
    }

    #[test]
    fn nested_shape_docs_variants_and_roles_flow_from_the_types() {
        let reg = registry();
        let charter = reg
            .kinds
            .iter()
            .find(|kind| kind.name == "charter")
            .unwrap();
        assert!(charter.doc.contains("WHY"));

        let entity = reg.kinds.iter().find(|kind| kind.name == "entity").unwrap();
        let aliases = entity
            .fields
            .iter()
            .find(|field| field.name == "aliases")
            .unwrap();
        assert!(aliases.doc.contains("name variant"));
        let timeline = entity
            .fields
            .iter()
            .find(|field| field.name == "timeline")
            .unwrap();
        let kyyn_core::registry::FieldType::List(inner) = &timeline.ty else {
            panic!("timeline is a list")
        };
        let kyyn_core::registry::FieldType::Struct(fields) = inner.as_ref() else {
            panic!("timeline elements are records")
        };
        assert_eq!(
            fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>(),
            ["time", "entry"]
        );

        let relationship = reg
            .kinds
            .iter()
            .find(|kind| kind.name == "relationship")
            .unwrap();
        let verb = relationship
            .fields
            .iter()
            .find(|field| field.name == "verb")
            .unwrap();
        let kyyn_core::registry::FieldType::Enum(variants) = &verb.ty else {
            panic!("verb is an enum")
        };
        assert_eq!(variants.len(), 14);
        assert_eq!(reg.coherence_errors(), Vec::<String>::new());
    }

    #[test]
    fn registry_round_trips_through_ron() {
        let back: kyyn_core::registry::Registry =
            ron::from_str(&registry_ron()).expect("registry RON parses back");
        assert_eq!(back.kinds.len(), 8);
        assert!(back.queries.is_empty());
        assert_eq!(back.schema_hash, schema_hash());
    }
}
