#!/usr/bin/env bash
# Run a command under fnm-managed Node 24 LTS.
set -euo pipefail

export PATH="${HOME}/.local/share/fnm:${PATH}"
if ! command -v fnm >/dev/null 2>&1; then
  echo "fnm not found. Install: curl -fsSL https://fnm.vercel.app/install | bash" >&2
  exit 1
fi

eval "$(fnm env --shell bash)"
# Prefer workspace pin (.node-version / .nvmrc = 24)
if [[ -f .node-version ]] || [[ -f .nvmrc ]]; then
  fnm use --install-if-missing
else
  fnm use 24 --install-if-missing
fi

exec "$@"
