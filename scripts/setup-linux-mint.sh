#!/usr/bin/env sh
# Installs or updates Copla's Linux development build in a user-selected folder.
set -eu

repository_url='https://github.com/damoscodehub/copla.git'
branch_name='feat/linux-support'

fail() {
  printf '%s\n' "Error: $1" >&2
  exit 1
}

command -v apt >/dev/null 2>&1 || fail 'This installer supports Linux Mint and other APT-based distributions only.'
[ "$(id -u)" -ne 0 ] || fail 'Run this script as a regular user, not as root.'

printf '%s\n' 'Copla setup for Linux Mint'
printf '%s\n' 'Enter the full path that should become Copla’s application folder.'
printf '%s' 'Application folder: '
IFS= read -r requested_root

[ -n "$requested_root" ] || fail 'No application folder was provided.'
case "$requested_root" in
  /*) ;;
  *) fail 'Enter an absolute path beginning with /, for example /home/alex/Applications/Copla.' ;;
esac

application_root=$(realpath -m "$requested_root")
[ "$application_root" != '/' ] || fail 'The application folder cannot be /.'

if [ -e "$application_root" ] && [ ! -d "$application_root" ]; then
  fail 'The selected path is a file, not a folder.'
fi

if [ -d "$application_root" ] && [ ! -d "$application_root/.git" ] \
  && [ -n "$(find "$application_root" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
  fail 'The selected folder is not empty and is not an existing Copla checkout. Choose an empty folder to prevent overwriting files.'
fi

printf '%s\n' ''
printf '%s\n' 'Installing Linux packages. Your password may be requested.'
sudo apt update
sudo apt install -y git build-essential curl wget file pkg-config \
  libwebkit2gtk-4.1-dev libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev

node_major=''
if command -v node >/dev/null 2>&1; then
  node_major=$(node --version | sed 's/^v//' | cut -d. -f1)
fi

if [ -z "$node_major" ] || [ "$node_major" -lt 24 ]; then
  printf '%s\n' 'Installing Node.js 24.'
  temporary_node_setup=$(mktemp)
  trap 'rm -f "$temporary_node_setup"' EXIT HUP INT TERM
  curl -fsSL https://deb.nodesource.com/setup_24.x -o "$temporary_node_setup"
  sudo -E bash "$temporary_node_setup"
  sudo apt install -y nodejs
  rm -f "$temporary_node_setup"
  trap - EXIT HUP INT TERM
fi

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env"
fi

if ! command -v rustc >/dev/null 2>&1; then
  printf '%s\n' 'Installing Rust.'
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env"
fi

if [ -d "$application_root/.git" ]; then
  printf '%s\n' 'Updating the existing Copla checkout.'
  [ -z "$(git -C "$application_root" status --porcelain)" ] \
    || fail 'The existing checkout contains local changes. Update it manually after saving or discarding those changes.'
  git -C "$application_root" fetch origin "$branch_name"
  git -C "$application_root" switch "$branch_name" 2>/dev/null || git -C "$application_root" switch --track "origin/$branch_name"
  git -C "$application_root" pull --ff-only origin "$branch_name"
else
  printf '%s\n' 'Downloading Copla.'
  mkdir -p "$(dirname "$application_root")"
  git clone --branch "$branch_name" --single-branch "$repository_url" "$application_root"
fi

printf '%s\n' 'Installing Copla dependencies.'
npm --prefix "$application_root" ci

printf '%s\n' 'Building Copla. The first build can take several minutes.'
npm --prefix "$application_root" run desktop:build:linux

printf '%s\n' ''
printf '%s\n' 'Copla is ready.'
printf '%s\n' "Application folder: $application_root"
printf '%s\n' 'Starting Copla now.'
cd "$application_root"
exec sh scripts/launch-copla.sh
