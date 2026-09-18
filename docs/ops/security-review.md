<!-- Acceptance evidence: docs:ops:security-review -->
# Invite-deploy security review

Reviewed 2026-09-18 against an invite-only HTTPS domain. Not an external firm audit.

| Check | Status |
| --- | --- |
| `ALLOWED_ORIGIN` must match the public HTTPS origin | Required in compose for a non-localhost deploy |
| TCP peer is used for bans/limits; `X-Forwarded-For` only from `TRUSTED_PROXIES` | Implemented (`security::client_ip`) |
| Room-create bucket is per guest IP, not `127.0.0.1` | Implemented |
| `/metrics` is not on :80/:443 | Caddy public matcher is `/ws /health /ready` only |
| Admin password lives in `DATA_DIR/store.json` | Volume perms only; no accounts product |
| Resume tokens expire in 20 seconds and stay in memory | `security::resume_ttl` |
| No `unsafe` Rust | `#![forbid(unsafe_code)]` on every crate |
| Drain then restart | `/DRAIN` → `/ready` 503 → persist → roll image |
| SIGTERM flushes bans/config | `persist_for_shutdown`; operator drain survives, accidental stop does not force drain on the next boot |

Advisory scan: CI runs `cargo audit` (empty ignore list in `.cargo/audit.toml`) and `npm audit --omit=dev`. Add a RUSTSEC id to that file only with a reason here.

## Out of this review

Public DDoS/WAF/CDN, protocol fuzz, licensed Soldat assets, and account login. See `docs/parity/intentional-differences.md`.
