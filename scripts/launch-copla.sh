#!/usr/bin/env sh
set -eu

repository_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
application_path="$repository_root/src-tauri/target/release/copla"

if [ ! -x "$application_path" ]; then
  printf '%s\n' 'Copla has not been built yet.' >&2
  printf '%s\n' 'Run "npm run desktop:build:linux" from the repository root, then launch this script again.' >&2
  exit 1
fi

exec "$application_path"
