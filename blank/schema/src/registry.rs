//! Generated registry assembly. Record structure lives once, on the Rust
//! types in `model.rs`; this module retains only KB-wide role policy.

use kyyn_core::registry::{Affordance, RoleDecl};

use crate::model::{Charter, SelfContext};

kyyn_core::kyyn_schema! {
    namespaces: [
        "default" => {
            doc: "The starter's explicit general-purpose ontology.",
            kinds: [Charter, SelfContext],
            roles: roles(),
            queries: crate::queries::queries(),
        },
    ],
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

    #[test]
    fn docs_and_roles_flow_from_the_types() {
        let reg = registry();
        let charter = reg
            .kinds()
            .find(|kind| kind.name == "default/charter")
            .unwrap();
        assert!(charter.doc.contains("WHY"));
        assert_eq!(reg.coherence_errors(), Vec::<String>::new());
    }

    #[test]
    fn registry_round_trips_through_ron() {
        let back: kyyn_core::registry::Registry =
            ron::from_str(&registry_ron()).expect("registry RON parses back");
        assert_eq!(kyyn_core::ronfmt::to_ron(&back).unwrap(), registry_ron());
        assert_eq!(back.schema_hash, schema_hash());
    }
}
