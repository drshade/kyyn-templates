#!/usr/bin/env bash
# Regenerate one template's committed Registry directly through its schema
# protocol. This deliberately needs no machine-local Kyyn KB registration.
set -euo pipefail
cd "$(dirname "$0")/.."

template=${1:-}
if [ -z "$template" ] || [ ! -f "$template/kyyn-template.ron" ]; then
    echo "usage: $0 <template> (for example: blank or gbrain)" >&2
    exit 2
fi

output="$template/.registry.ron.tmp.$$"
trap 'rm -f "$output"' EXIT
printf 'Registry\n' \
    | cargo run --quiet --manifest-path "$template/schema/Cargo.toml" \
    | sed '1s/^Registry(//; $s/)$//' > "$output"
mv "$output" "$template/registry.ron"
trap - EXIT
echo "$template: refreshed registry.ron"
