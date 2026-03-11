# Commands Reference

All commands support the `-n <namespace>` flag to target a specific namespace. Default namespace is `default`, unless overridden via `set-namespace`.

---

## Resource Listing

### `pod` — List Pods

```bash
kctl pod                    # List all pods in the namespace
kctl pod myapp              # Filter pods matching "myapp"
kctl pod myapp -n staging   # Filter in a specific namespace
```

### `svc` — List Services

```bash
kctl svc                    # List all services
kctl svc myapp              # Filter services matching "myapp"
```

### `deploy` — List Deployments

```bash
kctl deploy                 # List all deployments
kctl deploy myapp           # Filter deployments matching "myapp"
```

### `ing` — List Ingresses

```bash
kctl ing                    # List all ingress resources
kctl ing myapp              # Filter ingresses matching "myapp"
```

### `cm` — List ConfigMaps

```bash
kctl cm                     # List all configmaps
kctl cm myapp               # Filter configmaps matching "myapp"
```

### `secret` — List Secrets

```bash
kctl secret                 # List all secrets
kctl secret myapp           # Filter secrets matching "myapp"
```

---

## Debugging & Inspection

### `log` — Stream Pod Logs

Finds the first pod matching the app name and streams its logs (`-f`).

```bash
kctl log myapp              # Stream logs from first "myapp" pod
kctl log myapp -n staging   # Stream from staging namespace
```

> **Requires** an app name — `kctl log` alone will show an error.

### `describe` — Describe a Pod

Finds the first pod matching the app name and runs `kubectl describe` on it.

```bash
kctl describe myapp
kctl describe myapp -n production
```

### `exec` — Shell Into a Pod

Finds the first matching pod and opens an interactive shell. Tries `sh` first (works on alpine/distroless), falls back to `bash`.

```bash
kctl exec myapp
kctl exec myapp -n staging
```

### `events` — Show Cluster Events

Shows events sorted by timestamp. Optionally filter by app name.

```bash
kctl events                 # All events in the namespace
kctl events myapp           # Events related to "myapp"
```

### `top` — Pod Resource Usage

Shows CPU and memory usage for pods. Requires [Metrics Server](https://github.com/kubernetes-sigs/metrics-server) to be installed.

```bash
kctl top                    # All pod metrics
kctl top myapp              # Metrics for "myapp" pods
```

---

## Actions

### `delete` — Delete a Pod

Finds the first matching pod and deletes it (useful for forcing a restart in a ReplicaSet).

```bash
kctl delete myapp
kctl delete myapp -n staging
```

### `restart` — Rollout Restart a Deployment

Triggers a rolling restart of the deployment (equivalent to `kubectl rollout restart`).

```bash
kctl restart myapp
kctl restart myapp -n production
```

### `scale` — Scale a Deployment

Scales the deployment to the specified number of replicas.

```bash
kctl scale myapp 3          # Scale "myapp" to 3 replicas
kctl scale myapp 0          # Scale down to zero
```

### `forward` — Port Forward to a Pod

Finds the first matching pod and forwards a local port to it.

```bash
kctl forward myapp 8080:80
kctl forward myapp 3000:3000 -n staging
```

---

## Configuration

### `set-namespace` — Persist Default Namespace

Sets a default namespace so you don't need to pass `-n` every time.

```bash
kctl set-namespace -n staging    # All future commands default to "staging"
kctl set-namespace               # Reset to "default"
```

The namespace is stored in `~/.kctlconf`. You can always override it with `-n`.

### `ctx` — List or Switch Contexts

Manage kubectl contexts (clusters) without the verbose `kubectl config` syntax.

```bash
kctl ctx                    # List all contexts (highlights current)
kctl ctx production         # Switch to "production" context
```
