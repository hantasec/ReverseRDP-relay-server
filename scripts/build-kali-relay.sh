#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/rdp-reverse-relay/Cargo.toml"
output_dir="$repo_root/dist"
output_file="$output_dir/rdp-reverse-relay-linux-x86_64"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo가 없습니다. Kali에서 먼저 Rust 도구체인을 설치하십시오." >&2
  exit 1
fi

cargo build --manifest-path "$manifest" --release
mkdir -p "$output_dir"
install -m 0755 "$repo_root/rdp-reverse-relay/target/release/rdp-reverse-relay" "$output_file"

echo "Relay 생성 완료: $output_file"
sha256sum "$output_file"
