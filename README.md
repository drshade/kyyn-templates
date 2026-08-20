# kyyn-templates

KB templates for [kyyn](https://github.com/drshade/kyyn) (ADR 0007). Each
top-level directory holding a `kyyn-template.ron` manifest is a template: a
**valid, runnable KB** — schema crate, committed registry, charter, immutable
connector-repository pins, and directional source skeleton — whose identity
values are canonical dummy literals, swapped for real ones at
`kyyn init --template <name>`. Engines pin this repo at a rev;
`kyyn init` instantiates from the pin, and existing KBs adopt a template's
vocabulary later as a pack (an ordinary proposal).

Each template commits `schema/Cargo.lock`, `schema/rust-toolchain.toml`,
`schema/validator.wasm`, and `schema/validator.toml`. Together they pin
dependency resolution, the exact Rust release, the zero-import
`kyyn:validator@1` component, and its deterministic Wasmtime execution
profile. `schema/kyyn.toml` retains the authoring protocol version.

## Templates

- **blank** — the minimal governed KB: an explicit `default` namespace, an
  empty recipe ledger, a charter and the owner's own record. The default for
  `kyyn init`.
- **gbrain** — a gbrain-style operational knowledge brain: five primitive
  page kinds in the `gbrain` namespace (entity / media / temporal /
  annotation / concept), typed `relationship` edges over the gbrain link
  verbs, and `take` claims with provenance. Its charter remains in `default`;
  the active `gbrain-repository` recipe explicitly consumes the declared
  repository source and may update only `gbrain`. Compiled truth above the
  line, an append-only timeline below.

## Maintaining a template

Templates are maintained like any KB: register the template DIRECTORY as a
KB (a subdirectory mount), then propose → validate → accept with the normal
tooling; publishing is `git push` of a tree the accept gate already
validated. Keep each template's `registry.ron` fresh: after a schema change,
run `./scripts/refresh-registry.sh <template>` without requiring a local KB
profile (or use `kyyn --kb <mount> registry --from-sources` for an already
registered mount). Keep every dummy identity declared in its manifest —
nothing else in a tree may be parameterized.

Each template keeps immutable connector repository pins in `connectors.ron`
and configured source instances in `sources.ron`. Source references use only
`connector:<repository>#<connector>`; repository pins never live in the
directional source manifest.

## Evolving a schema

Rust in `<template>/schema/src/model.rs` is the structural source of truth.
Ordinary nested structs and unit enums derive `kyyn_core::KyynType`; stored
record structs derive `kyyn_core::KyynKind` and declare their kind name and
`facts/` storage pattern. Put Kyyn-only meaning beside the field with
`#[kyyn(markdown)]`, `#[kyyn(role = "...")]`, `#[kyyn(refers_to = "...")]`,
or an explicit link policy. Rust types, serde defaults and doc comments flow
into Registry without being restated.

Adding a field normally touches only its Rust declaration. Adding a kind adds
its type plus one entry to the appropriate explicit namespace's
`kinds: [...]` inventory in `registry.rs`. Namespaces organize one ontology;
they are not permissions, imports, or separate validators. For an endpoint-tree
domain invariant, write
`fn custom(entries: &[(String, String)], systems: &[&str]) -> Vec<Violation>`
in `validate.rs` and pass it as `custom_validate` to `kyyn_schema!`; generic
Registry validation still runs first. `systems` is the KB's declared external
link-system set. Iterate a kind with
`kyyn_core::schema::custom_records::<MyKind>(entries)` so custom checks share
Registry's routing and typed decoding without duplicating generic parse
findings. For a rule about a change rather than only its final state, add it to
`validate_transition`: its `kyyn_core::validator::ValidationContext` carries
the previous and candidate snapshots plus the exact engine-verified authority
for each final field transition. Unsupported representations implement the same
`KyynType`/`KyynKind` trait manually—do not inspect macro expansion or create
a second registry declaration.

After an intentional schema edit, use a Kyyn proposal so source, generated
Registry, lockfile, component, and provenance metadata remain one reviewed
unit:

1. stage the source edit, then run
   `kyyn component build --proposal <id> --kb <path>` (or use schema staging,
   which performs the same atomic authoring build);
2. review the `registry.ron`, `schema/validator.toml`, dependency lock, and
   source diff; the component digest binds the binary artifact;
3. run that schema's semantic and round-trip tests;
4. run `./scripts/check-templates.sh`.

There is deliberately no hand-maintained Registry fingerprint or fixed kind
count in a template schema. The generated `registry.ron`, source-bound
component metadata, and accept gate provide the mechanical drift guard; a
parallel hash constant would go stale on every legitimate on-wire schema
evolution and cannot be maintained through the agent authoring surface.

In a real KB, stage the schema sources, generated registry and any required
record migration together. Accept rebuilds source-bound components offline and
requires byte-for-byte agreement before validating the complete new tree
against its own schema.
