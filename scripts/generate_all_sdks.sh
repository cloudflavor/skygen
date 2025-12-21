#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

pushd crates/skygen

cargo build --release
cargo install --path .

pushd

targets=()

for file in open-api-specs/scaleway/*; do
  name=$(basename "$file")

  # skip tsv files
  [[ $name =~ tsv ]] && continue

  # transform: scaleway.file.v1alpha1.Api.yml -> scaleway_file
  name=$(sed -E 's/^scaleway\.([^.]+)\..*\.ya?ml$/scaleway_\1/' <<<"$name")
  target=$file:scaleway/$name

  targets+=("$target")
done

specs=(
  "open-api-specs/bundled-cloudflare.yaml:cloudflare"
  "open-api-specs/bundled-digitalocean.yaml:digitalocean"
  "open-api-specs/exoscale-api.json:exoscale"
  "open-api-specs/hetzner.json:hetzner"
)

targets+=("${specs[@]}")

for entry in "${targets[@]}"; do
  spec="${entry%%:*}"
  target="${entry##*:}"
  if [[ ! -f "$spec" ]]; then
    echo "[skip] Spec not found: $spec" >&2
    continue
  fi
  echo "[delete] crates/generated/$target"
  rm -rf crates/generated/$target || true
  echo "[generate] $target from $spec"
  skygen generate -s "$spec" -o "crates/generated/$target"
done
