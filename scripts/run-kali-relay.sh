#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
relay="$repo_root/dist/rdp-reverse-relay-linux-x86_64"

if ! ip -4 address show | grep -Fq "203.230.56.162/"; then
  echo "이 호스트에서 203.230.56.162 주소를 찾지 못했습니다." >&2
  exit 1
fi

if [[ ! -x "$relay" ]]; then
  echo "Relay 실행 파일이 없습니다. 먼저 scripts/build-kali-relay.sh를 실행하십시오." >&2
  exit 1
fi

export RDP_LISTEN="${RDP_LISTEN:-127.0.0.1:13389}"
export TUNNEL_LISTEN="${TUNNEL_LISTEN:-203.230.56.162:14443}"
exec "$relay"
