#!/bin/sh
set -eu
cd /workspace
if curl -sf -o /dev/null --max-time 2 http://127.0.0.1:8080/healthz; then
  exit 0
fi
BIN="/workspace/rust-server/target/release/grok-config-server"
if [ ! -x "$BIN" ]; then
  cargo build --manifest-path /workspace/rust-server/Cargo.toml --release >>/tmp/app-startup.log 2>&1
fi
"$BIN" --host 0.0.0.0 --port 8080 --static-dir /workspace/rust-server/static >>/tmp/app-startup.log 2>&1 &
