#!/usr/bin/env bash
# The per-template gate (ADR 0007/0010): every template directory must be a
# VALID KB fragment — its schema crate's tests pass and its committed
# registry.ron is FRESH (the schema binary's own hash matches the committed
# one; the engine's accept gate re-proves the full tree at init/accept).
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
for manifest in */kyyn-template.ron; do
    t=$(dirname "$manifest")
    echo "== template '$t'"
    (cd "$t/schema" && cargo test --quiet) || { echo "$t: schema tests failed"; fail=1; continue; }
    live=$(cd "$t/schema" && echo "Registry" | cargo run --quiet | grep -m1 -o 'schema_hash: "[0-9a-f]*"')
    committed=$(grep -m1 -o 'schema_hash: "[0-9a-f]*"' "$t/registry.ron")
    if [ "$live" != "$committed" ]; then
        echo "$t: registry.ron is STALE (committed $committed, sources say $live) — regenerate it"
        fail=1
    else
        echo "$t: registry fresh"
    fi
done
exit $fail
