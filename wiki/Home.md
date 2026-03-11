# Welcome to the kctl Wiki

**kctl** is a CLI wrapper that simplifies everyday Kubernetes commands. Instead of typing long `kubectl` commands, use short, memorable aliases.

## Quick Start

```bash
# Install via Homebrew
brew tap alaminopu/kctl https://github.com/alaminopu/kctl.git
brew install kctl

# Or build from source
cargo build --release
mv target/release/kctl /usr/local/bin/
```

## Pages

- **[Commands Reference](Commands)** — Full list of all commands with examples
- **[Configuration](Configuration)** — Persistent namespace and context management
- **[Installation](Installation)** — Detailed install instructions for all platforms
