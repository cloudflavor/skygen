#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}" )/.." && pwd)"
cd "$ROOT_DIR"

specs=(
  "crates/skygen/open-api-specs/bundled-cloudflare.yaml:cloudflare"
  "crates/skygen/open-api-specs/bundled-digitalocean.yaml:digitalocean"
  "crates/skygen/open-api-specs/exoscale-api.json:exoscale"
  "crates/skygen/open-api-specs/hetzner.json:hetzner"
)

for entry in "${specs[@]}"; do
  spec="${entry%%:*}"
  target="${entry##*:}"
  if [[ ! -f "$spec" ]]; then
    echo "[skip] Spec not found: $spec" >&2
    continue
  fi
  echo "[generate] $target from $spec"
  cargo run -p skygen -- generate -s "$spec" -o "crates/generated/$target"
done
