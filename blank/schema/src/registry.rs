//! Generated registry assembly. Record structure lives once, on the Rust
//! types in `model.rs`; this module retains only KB-wide role policy.

use kyyn_core::registry::{Affordance, RoleDecl};

use crate::model::{Charter, SelfContext};

kyyn_core::kyyn_schema! {
    kinds: [Charter, SelfContext],
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
        "cdf7f153c7715276262020a7af95d45c79ae615ea48ed601815917f22de418d6";

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
    fn docs_and_roles_flow_from_the_types() {
        let reg = registry();
        let charter = reg
            .kinds
            .iter()
            .find(|kind| kind.name == "charter")
            .unwrap();
        assert!(charter.doc.contains("WHY"));
        assert_eq!(reg.coherence_errors(), Vec::<String>::new());
    }

    #[test]
    fn registry_round_trips_through_ron() {
        let back: kyyn_core::registry::Registry =
            ron::from_str(&registry_ron()).expect("registry RON parses back");
        assert_eq!(back.kinds.len(), 2);
        assert!(back.queries.is_empty());
        assert_eq!(back.schema_hash, schema_hash());
    }
}
