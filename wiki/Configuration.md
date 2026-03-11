# Configuration

kctl supports persistent configuration to avoid repeating flags.

## Default Namespace

By default, all commands operate in the `default` Kubernetes namespace. You can change this:

```bash
kctl set-namespace -n staging
```

After this, all commands will use `staging` unless you explicitly pass `-n`:

```bash
kctl pod myapp            # Uses "staging"
kctl pod myapp -n prod    # Overrides to "prod"
```

To reset back to the default:

```bash
kctl set-namespace
```

### How It Works

The namespace is stored in `~/.kctlconf` as a simple key-value:

```
KCTL_NAMESPACE=staging
```

**Priority order:**
1. Explicit `-n <namespace>` flag (highest)
2. Value from `~/.kctlconf`
3. `default` (lowest)

## Context Management

Switch between Kubernetes clusters using the `ctx` command:

```bash
kctl ctx                    # List all available contexts
kctl ctx production         # Switch to "production" context
```

This wraps `kubectl config use-context` and `kubectl config get-contexts`.
