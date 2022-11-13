#!/usr/bin/env bash
set -ex

sudo apt-get install curl jq git -y
curl -Ls "https://shlink.${hw_url}/ci-utils" | sudo bash -

mkdir ~/.ssh
echo -e "Host ${mpr_url}\n  Hostname  ${mpr_url}\n  IdentityFile  ${HOME}/.ssh/ssh_key" | tee -a "${HOME}/.ssh/config"
echo "${ssh_key}" | tee "/${HOME}/.ssh/ssh_key"

MPR_SSH_KEY="$(curl "https://${mpr_url}/api/meta" | jq -r '.ssh_key_fingerprints.ECDSA')"

SSH_HOST="${mpr_url}" \
	SSH_EXPECTED_FINGERPRINT="${MPR_SSH_KEY}" \
	SET_PERMS=true \
	get-ssh-key

cd makedeb/
source PKGBUILD

git clone "ssh://mpr@${mpr_url}/${pkgname}"
cp PKGBUILD "${pkgname}/"
cd "${pkgname}/"

git config --global user.name 'Kavplex Bot'
git config --global user.email 'kavplex@hunterwittenborn.com'

makedeb --print-srcinfo | tee .SRCINFO
git add .
git commit -m "Bump version to '${pkgver}-${pkgrel}'"
git push

# Clean up, as crates.io publishing doesn't like a Git directory with uncommitted changes.
cd ../
rm "${pkgname}/" -rf
