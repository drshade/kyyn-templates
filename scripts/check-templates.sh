#!/usr/bin/env bash
# The per-template gate (ADR 0007/0010): every template directory must be a
# VALID KB fragment — its schema crate's tests pass, its Wasm guest compiles,
# and its committed registry.ron is FRESH. The engine's accept gate re-proves
# the frozen component bytes and full tree at init/accept.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
expected_connector_rev=
for manifest in */kyyn-template.ron; do
    t=$(dirname "$manifest")
    echo "== template '$t'"
    if [ ! -f "$t/connectors.ron" ] || [ ! -f "$t/sources.ron" ]; then
        echo "$t: directional connector/source manifests are missing"
        fail=1
        continue
    fi
    if rg -n '\b(taps|tap|plugin|plugins)\b|kyyn-plugins|tap:' \
        "$t/connectors.ron" "$t/sources.ron"; then
        echo "$t: retired tap/plugin vocabulary remains in directional manifests"
        fail=1
        continue
    fi
    if rg -n '(^|[^[:alnum:]_])sources[[:space:]]*:' "$t/connectors.ron" \
        || rg -n '(^|[^[:alnum:]_])connectors[[:space:]]*:' "$t/sources.ron"; then
        echo "$t: repository pins and source instances cross manifest ownership"
        fail=1
        continue
    fi
    if ! grep -Fq 'repo: Some("ssh://git@github.com/drshade/kyyn-connectors")' "$t/connectors.ron"; then
        echo "$t: first-party connector repository URL is not canonical"
        fail=1
        continue
    fi
    connector_rev=$(sed -n 's/.*rev: Some("\([0-9a-f]\{40\}\)").*/\1/p' "$t/connectors.ron")
    if [[ ! "$connector_rev" =~ ^[0-9a-f]{40}$ ]]; then
        echo "$t: connector repository is not pinned exactly once to a full commit"
        fail=1
        continue
    fi
    if [ -z "$expected_connector_rev" ]; then
        expected_connector_rev=$connector_rev
    elif [ "$connector_rev" != "$expected_connector_rev" ]; then
        echo "$t: first-party connector revision drifts from the other templates"
        fail=1
        continue
    fi
    (cd "$t/schema" && cargo test --quiet --locked) || { echo "$t: schema tests failed"; fail=1; continue; }
    (cd "$t/schema" && cargo build --quiet --locked --release --target wasm32-unknown-unknown --lib) \
        || { echo "$t: Wasm validator build failed"; fail=1; continue; }
    component="$t/schema/validator.wasm"
    metadata="$t/schema/validator.toml"
    if [ ! -f "$component" ] || [ ! -f "$metadata" ]; then
        echo "$t: committed validator component or metadata is missing"
        fail=1
        continue
    fi
    declared=$(sed -n 's/^sha256 = "\([0-9a-f]\{64\}\)"$/\1/p' "$metadata")
    actual=$(sha256sum "$component" | cut -d' ' -f1)
    if [ "$declared" != "$actual" ]; then
        echo "$t: validator digest is STALE (metadata $declared, artifact $actual)"
        fail=1
        continue
    fi
    live=$(cd "$t/schema" && echo "Registry" | cargo run --quiet --locked | grep -m1 -o 'schema_hash: "[0-9a-f]*"')
    committed=$(grep -m1 -o 'schema_hash: "[0-9a-f]*"' "$t/registry.ron")
    if [ "$live" != "$committed" ]; then
        echo "$t: registry.ron is STALE (committed $committed, sources say $live) — regenerate it"
        fail=1
    else
        echo "$t: registry fresh"
    fi
done
exit $fail
