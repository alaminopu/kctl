class Kctl < Formula
  desc "A CLI wrapper for making kubernetes commands much easier"
  homepage "https://github.com/alaminopu/kctl"
  url "https://github.com/dnivra26/kctl/releases/download/0.1.4/kctl-0.1.4-x86_64-darwin.tar.gz"
  sha256 "5bd5558d2488023ab3f55a77606de1d75305e7a1cf7c0e2a321026e7ca66e781"

  def install
    bin.install "kctl"
  end
end
