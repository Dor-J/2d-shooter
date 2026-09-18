# Intentional differences from historical Soldat

These are not unfinished gameplay items. Marking them `Present` would be a false claim.

## Unlicensed content

- **Historical interface packs** (Cabbage, Classic, Lacey V2, Micro1, Military, Predator, Soldat Style, Storm, Tech, Text) are catalogued as metadata names only. Their art is not shipped (`G33-019`–`G33-028`).
- **Audio clips** are generated tones until licensed files land in provenance. Event routing, deafness, and kit/weapon cues are present.
- **Per-weapon sprites and reload animations** use the generic gostek rig rather than Soldat’s licensed sheets.

## Optional product work, left off

- Account-backed persistence and login (`G22-024`, `G36-002`, `G36-011`).
- Clan/team statistics (`G29-023`). Player match summaries persist in `DATA_DIR/store.json`.
- Public DDoS/CDN/WAF (`G31-020`, `G36-031`) wait on an open-internet launch. The invite target uses Caddy TLS, an origin lock, trusted-proxy IP limits, and `docs/ops/security-review.md`.
- An external firm security review and a Playwright browser matrix stay deferred (`G36-024`). The invite checklist is G36-028.
- Protocol fuzzing as a separate nightly harness (`G31-015`). Map bytes are already mutation-tested in-tree.

## Engine and ops choices

- Pause is a room flag, not a `World` field, so a paused match keeps a stable digest.
- Persistence is a versioned JSON file (`DATA_DIR/store.json`), not a database server (`G36-010`). Match worlds are not snapshotted; drain, then restart.
- Protocol stays JSON. Snapshot deltas, server rewind, interest management, and a binary encoding (`G30-008`, `G30-012`, `G30-013`, `G30-014`) wait on a measured bandwidth budget.
- `check:gaps` still fails on unmapped/stale rows and Present-without-evidence, not on every remaining `Missing` line. The leftover lines above are the reason.
