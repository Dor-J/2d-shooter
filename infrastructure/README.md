# Local production-style deployment

`compose.yaml` runs the Rust API and a Caddy-hosted web client. `Dockerfile.web` builds the Rust/Wasm module and the Vue site, then serves the built files with Caddy. Caddy terminates HTTPS and proxies `/ws`, `/health`, and `/metrics` to the API.

Set `SITE_ADDRESS` to the public domain and `PUBLIC_ORIGIN` to the exact HTTPS origin used by browsers. For a local Compose test, the defaults are `localhost` and `http://localhost`.

```sh
docker compose -f infrastructure/compose.yaml up --build -d
docker compose -f infrastructure/compose.yaml ps
```

Open `http://localhost` for the local test, or the configured HTTPS domain in production. Check `http://localhost/health` for a local liveness response. Use `docker compose -f infrastructure/compose.yaml logs -f` for service output. Stop the stack with `docker compose -f infrastructure/compose.yaml down`.

The deployment is a single-region starting point, not a validated production release. A restart ends active matches. Before public deployment, configure a domain, firewall, monitoring, image digest pinning, rollback procedure, and load testing. The `/metrics` endpoint is currently proxied publicly; restrict it at the edge before exposing a public domain.
