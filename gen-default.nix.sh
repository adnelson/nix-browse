#!/usr/bin/env bash

if [[ -z "$NAME" ]]; then
  echo "Need to set project name with \$NAME"
  exit 1
fi

cmd="nix-prefetch-git https://github.com/dmjio/miso"

if [[ -n "$REV" ]]; then
  cmd+=" --rev $REV"
fi

INFO=$(sh -c "$cmd")

COMMIT=$(jq -r .rev <<< "$INFO")
SHA256=$(jq -r .sha256 <<< "$INFO")

cat <<EOF
with (import (builtins.fetchTarball {
  url = "https://github.com/dmjio/miso/archive/$COMMIT.tar.gz";
  sha256 = "$SHA256";
}) {});
pkgs.haskell.packages.ghcjs.callCabal2nix "$NAME" ./. {}
EOF
