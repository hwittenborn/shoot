#!/usr/bin/env bash
set -ex

if echo "${DRONE_COMMIT_MESSAGE}" | grep -q 'GH SKIP'; then
    echo "Skipping GitHub release creation!"
    exit 0
fi

.drone/scripts/setup-pbmpr.sh
sudo apt-get install gh parse-changelog rustup -y
rustup default stable

rustup_targets=(
    'aarch64-unknown-linux-gnu'
    'i686-pc-windows-gnu'
    'i686-pc-windows-msvc'
    'i686-unknown-linux-gnu'
    'x86_64-apple-darwin'
    'x86_64-pc-windows-gnu'
    'x86_64-pc-windows-msvc'
    'x86_64-unknown-linux-gnu'
)
gh_release_assets=()
for target in "${rustup_targets[@]}"; do
    rustup target add "${target}"
    cargo build --target "${target}" --release
    gh_release_assets+=("${PWD}/target/${target}/release/shoot#shoot-${target}")
done

source makedeb/PKGBUILD

release_notes="$(parse-changelog CHANGELOG.md "${pkgver}")"
echo "${github_api_key}" | gh auth login --with-token
gh release create "v${pkgver}" --title "v${pkgver}" --target "${DRONE_COMMIT_SHA}" -n "${release_notes}" "${gh_release_assets[@]}"

# vim: set sw=4 expandtab:
