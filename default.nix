with (import (builtins.fetchTarball {
  url = "https://github.com/dmjio/miso/archive/7e2902a939f7196d2be450b605e880c40ee2542e.tar.gz";
  sha256 = "18nzp904msz9ljb3k65x2gkabbsmq3lx8cfbjywaamx6bqvqqmpi";
}) {});
pkgs.haskell.packages.ghcjs.callCabal2nix "nix-browse" ./. {}
