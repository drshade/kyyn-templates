# kyyn-templates

KB templates for [kyyn](https://github.com/drshade/kyyn) (ADR 0007). Each
top-level directory holding a `kyyn-template.ron` manifest is a template: a
**valid, runnable KB** — schema crate, committed registry, charter, sources
skeleton — whose identity values are canonical dummy literals, swapped for
real ones at `kyyn init --template <name>`. Engines pin this repo at a rev;
`kyyn init` instantiates from the pin, and existing KBs adopt a template's
vocabulary later as a pack (an ordinary proposal).

## Templates

- **blank** — the minimal governed KB: a charter and the owner's own record.
  The default for `kyyn init`.
- **gbrain** — a gbrain-style operational knowledge brain: five primitive
  page kinds (entity / media / temporal / annotation / concept), typed
  `relationship` edges over the gbrain link verbs, and `take` claims with
  provenance. Compiled truth above the line, an append-only timeline below.

## Maintaining a template

Templates are maintained like any KB: register the template DIRECTORY as a
KB (a subdirectory mount), then propose → validate → accept with the normal
tooling; publishing is `git push` of a tree the accept gate already
validated. Keep each template's `registry.ron` fresh (regenerate with
`kyyn --kb <mount> registry --from-sources > registry.ron` after schema
changes) and every dummy identity declared in its manifest — nothing else in
a tree may be parameterized.

## Evolving a schema

Rust in `<template>/schema/src/model.rs` is the structural source of truth.
Ordinary nested structs and unit enums derive `kyyn_core::KyynType`; stored
record structs derive `kyyn_core::KyynKind` and declare their kind name and
`facts/` storage pattern. Put Kyyn-only meaning beside the field with
`#[kyyn(markdown)]`, `#[kyyn(role = "...")]`, `#[kyyn(refers_to = "...")]`,
or an explicit link policy. Rust types, serde defaults and doc comments flow
into Registry without being restated.

Adding a field normally touches only its Rust declaration. Adding a kind adds
its type plus one entry to the explicit `kinds: [...]` inventory in
`registry.rs`. For a domain invariant, write a typed `custom` function in
`validate.rs` and pass it as `custom_validate` to `kyyn_schema!`; generic
Registry validation still runs first. Unsupported representations implement
the same `KyynType`/`KyynKind` trait manually—do not inspect macro expansion
or create a second registry declaration.

After a schema edit, run `./scripts/check-templates.sh` from this repository
and regenerate the template's committed `registry.ron` as described above.
In a real KB, stage the schema sources, generated registry and any required
record migration together on a proposal so accept validates the complete new
tree against its own schema.
