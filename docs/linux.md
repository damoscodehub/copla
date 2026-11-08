# Linux support

Copla supports x86_64 Linux through AppImage, Debian, and RPM packages. The
Linux build baseline is Ubuntu 22.04, selected to preserve broad AppImage
compatibility with the WebKitGTK 4.1 runtime required by Tauri v2.

## Development prerequisites

On Debian or Ubuntu, install the supported Tauri prerequisites before running
the development application or a Linux build:

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Install Node.js 24 or later and Rust stable, then run:

```sh
npm ci
npm run desktop
```

## Guided Linux Mint setup

For a beginner-friendly install or update from the current Linux branch,
download the setup script from a terminal:

```sh
curl -fL https://raw.githubusercontent.com/damoscodehub/copla/feat/linux-support/scripts/setup-linux-mint.sh -o setup-copla-linux-mint.sh
```

Then run it:

```sh
sh setup-copla-linux-mint.sh
```

The script asks for an absolute application-folder path, such as
`/home/alex/Applications/Copla`. That exact folder becomes the Copla checkout;
the script never creates an extra `copla` directory inside it. It installs only
missing prerequisites, clones `feat/linux-support` into an empty selected
folder or fast-forward updates an existing clean checkout, installs locked
dependencies, builds the Linux packages, and starts Copla. It refuses to
overwrite a non-empty folder or discard local source changes.

## Build and launch

Create Linux distribution artifacts with:

```sh
npm run desktop:build:linux
```

The output directory contains an AppImage, `.deb`, and `.rpm` package. Use the
native package for the target distribution where possible. The AppImage is the
portable option; make it executable before launching:

```sh
chmod +x Copla_*.AppImage
./Copla_*.AppImage
```

For a local non-packaged release executable, run `scripts/launch-copla.sh`
after the Linux build. This launcher does not make Copla's preferences or
recovery data portable.

## Verification policy

Linux changes must pass the frontend build and tests, Rust tests, a release
binary build, and bundle creation on Ubuntu 22.04. CI supplies this baseline.
The application is tested as an x86_64 Linux target. ARM64 support requires a
native ARM build runner and is intentionally not claimed until verified.

## Runtime notes

The application is fully local and uses the distribution's WebKitGTK runtime
for native packages. AppImage compatibility depends on the host system's glibc
version; producing releases on Ubuntu 22.04 avoids accidentally raising that
minimum requirement.
