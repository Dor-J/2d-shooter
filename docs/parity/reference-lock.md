# Soldat parity reference lock

Parity work uses immutable upstream revisions so expected mechanics and licensed content cannot drift between test runs.

| Reference | Revision | License | Use |
| --- | --- | --- | --- |
| [OpenSoldat source](https://github.com/opensoldat/opensoldat) `develop` | `c7596cdca32416cb66339b339105eb1e07e7fbbf` | MIT | Behavioral reference and numeric fixture derivation |
| [OpenSoldat base content](https://github.com/opensoldat/base) `master` | `5b6e5bef23f5c0d58fb1d4d887b9b94ebcf799b4` | CC BY 4.0 | Content reference subject to per-asset provenance review |
| [Soldat Community Wiki](https://wiki.soldat.pl/) | Retrieved 2026-09-15 | Community documentation | Descriptive context listed in `docs/gaps/gap-list.md` |

Every parity fixture must record the upstream repository, this exact commit, source path, and relevant symbol or data section. Derived expected values may be recorded; upstream implementation code must not be copied without preserving its MIT notice. Base assets may not enter the repository until `docs/provenance.md` records their path, author when known, license, modifications, attribution location, and redistribution approval.

Current source-derived implementations:

| Area | Pinned source paths |
| --- | --- |
| PMS binary layout and CRC | `shared/MapFile.pas` (`TMapFile`, `LoadMapFile`, `crc32`) |
| PMS allocation limits | `shared/PolyMap.pas` (`MAX_POLYS`, `MAX_SECTOR`, `MAX_PROPS`, `MAX_SPAWNPOINTS`, `MAX_COLLIDERS`) |
| PMS waypoint limits | `shared/Waypoints.pas` (`MAX_WAYPOINTS`, `MAX_CONNECTIONS`) |
