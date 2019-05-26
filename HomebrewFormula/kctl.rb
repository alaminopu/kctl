class Kctl < Formula
  desc "A CLI wrapper for making kubernetes commands much easier"
  homepage "https://github.com/alaminopu/kctl"
  url "https://github.com/alaminopu/kctl/releases/download/0.2.1/kctl-0.2.1-x86_64-darwin.tar.gz"
  sha256 "5e065d08753f50b5ebefb6cbde37f7abf0ac0215ba1959602f8d5343ff8e806a"
  version "0.2.1"
  def install
    bin.install "kctl"
  end
end
