# Installation

## Homebrew (macOS & Linux)

The easiest way to install kctl:

```bash
brew tap alaminopu/kctl https://github.com/alaminopu/kctl.git
brew install kctl
```

To update:

```bash
brew upgrade kctl
```

## From Source

### Prerequisites

Install [Rust and Cargo](https://rustup.rs/):

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Build & Install

```bash
git clone https://github.com/alaminopu/kctl.git
cd kctl
cargo build --release
```

The binary will be at `target/release/kctl`. Install it to your PATH:

```bash
chmod a+x target/release/kctl
mv target/release/kctl /usr/local/bin/
```

### Verify

```bash
kctl --version
# kctl 1.0.0
```

## Requirements

- **kubectl** must be installed and configured with a valid kubeconfig
- For `kctl top`: [Metrics Server](https://github.com/kubernetes-sigs/metrics-server) must be deployed in the cluster
