# Release digests

Recorded 2026-09-18. Keep the previous table when you deploy so rollback has a digest to start.

## Current

_No compose images yet. After `docker compose -f infrastructure/compose.yaml build`, re-run this script._

| Service | Image | Digest |
| --- | --- | --- |
| api | (unrecorded) | |
| web | (unrecorded) | |

## Rollback

1. `/DRAIN` on the live server (or stop accepting joins).
2. Wait until `GET /ready` is 503.
3. `docker compose -f infrastructure/compose.yaml down`
4. Start the **previous** image digests with the same `arena-data` volume.
5. `/UNDRAIN` if the restored process came up drained.

Matches do not survive. Bans and config do.

