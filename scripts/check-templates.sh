#!/usr/bin/env bash
# The per-template gate (ADR 0007/0010): every template directory must be a
# VALID KB fragment — its schema crate's tests pass, its Wasm guest compiles,
# and its committed registry.ron is FRESH. The engine's accept gate re-proves
# the frozen component bytes and full tree at init/accept.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
for manifest in */kyyn-template.ron; do
    t=$(dirname "$manifest")
    echo "== template '$t'"
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
