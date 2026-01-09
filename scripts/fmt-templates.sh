#!/usr/bin/env bash
# Formats the raw Rust templates that get embedded into the binary.
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/../../../../.." && pwd)"

if [ "$#" -eq 0 ]; then
  cat >&2 <<'EOF'
Usage: fmt-templates.sh <template-dir> [<template-dir>...]
Formats all *.rs files under each provided directory using rustfmt.
EOF
  exit 64
fi

declare -a TEMPLATE_DIRS=()
for arg in "$@"; do
  if [[ "$arg" != /* ]]; then
    echo "fmt-templates: '$arg' is not an absolute path. Please pass absolute directories." >&2
    exit 64
  fi
  TEMPLATE_DIRS+=("$arg")
done

if ! command -v rustfmt >/dev/null 2>&1; then
  echo "fmt-templates: rustfmt not found in PATH" >&2
  exit 127
fi

declare -a CONFIG_CANDIDATES=(
  "$ROOT_DIR/rustfmt.toml"
  "$ROOT_DIR/.rustfmt.toml"
)

declare -a RUSTFMT_ARGS=()
for cfg in "${CONFIG_CANDIDATES[@]}"; do
  if [ -f "$cfg" ]; then
    RUSTFMT_ARGS=(--config-path "$cfg")
    break
  fi
done

declare -a FILES=()
for dir in "${TEMPLATE_DIRS[@]}"; do
  if [ ! -d "$dir" ]; then
    continue
  fi

  while IFS= read -r -d '' file; do
    FILES+=("$file")
  done < <(find "$dir" -type f -name '*.rs' -print0 | sort -z)
done

if [ "${#FILES[@]}" -eq 0 ]; then
  echo "fmt-templates: no .rs files found under the provided directories" >&2
  exit 0
fi

echo "fmt-templates: running rustfmt on ${#FILES[@]} file(s)"
rustfmt "${RUSTFMT_ARGS[@]}" "${FILES[@]}"
