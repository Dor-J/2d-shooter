# Invite-only production-style deployment

`compose.yaml` runs the Rust API and a Caddy-hosted web client. `Dockerfile.web` builds the Rust/Wasm module and the Vue site, then serves the built files with Caddy. Caddy terminates HTTPS and proxies `/ws`, `/health`, and `/ready` to the API. `/metrics` stays on the docker network (`api:3000/metrics`); it is not on `:80`/`:443`.

## Required env for a real domain

A non-localhost deploy must set both:

| Variable | Example |
| --- | --- |
| `SITE_ADDRESS` | `play.example.com` |
| `PUBLIC_ORIGIN` | `https://play.example.com` |

`PUBLIC_ORIGIN` becomes `ALLOWED_ORIGIN`. A browser whose `Origin` does not match is refused. An empty origin check is only for a raw local `cargo run`. `TRUSTED_PROXIES=private` lets the API take the rightmost `X-Forwarded-For` hop from Caddy and ignore spoofed headers from a guest socket.

```sh
export SITE_ADDRESS=play.example.com
export PUBLIC_ORIGIN=https://play.example.com
docker compose -f infrastructure/compose.yaml up --build -d
docker compose -f infrastructure/compose.yaml ps
node scripts/record-image-digests.mjs
```

Open the configured HTTPS domain. `/health` is liveness; `/ready` is readiness and returns 503 while the API is draining. `/DRAIN` and `/UNDRAIN` are admin chat commands (not a shell). Use `docker compose -f infrastructure/compose.yaml logs -f` for service output. Stop the stack with `docker compose -f infrastructure/compose.yaml down`. Bans and config persist in the `arena-data` volume; live matches do not. See `docs/ops/policies.md` and `docs/ops/release.md`.

## Invite IP allowlist

Caddyfile comments show a `remote_ip` allowlist. Uncomment it and list operator VPN / known player ranges. That is the DDoS stand-in; there is no Cloudflare/WAF dependency.

## Metrics (optional)

To expose `/metrics` behind Caddy basic auth, set `METRICS_USER` and `METRICS_PASSWORD_HASH` (`caddy hash-password`) and uncomment the handle in `Caddyfile`. Do not proxy `/metrics` without that.

## Images and rollback

`docker compose -f infrastructure/compose.yaml build`, then `node scripts/record-image-digests.mjs` to write `RepoDigests` into `docs/ops/release.md`. Rollback is the previous digest plus the same `arena-data` volume. See that file for the one-command path.
