# Invite-operation policies

Use these when the stack is on a real HTTPS domain for invited players.

## Privacy

- Guest names and resume tokens are session data. They are not sold and they are not written into the JSON store.
- Ban records store an IP, a reason, and whether the ban is permanent. Temporary bans expire in memory and are not restored after restart.
- Audit lines record actor, command, and a redacted target. Do not ship raw packet captures from `/metrics`.

## Moderation

- Room passwords, mutes, kicks, and bans are the operator tools. There is no automatic profanity filter.
- An admin password on hello is the only remote-admin path. It is not a shell.
- `/DRAIN` sets drain (`/ready` → 503) and stops new joins. `/UNDRAIN` clears it. Finish or record the match, then restart.

## Edge

- A non-localhost deploy must set `SITE_ADDRESS` and `PUBLIC_ORIGIN=https://<domain>` (`ALLOWED_ORIGIN`).
- `TRUSTED_PROXIES=private` (compose default) trusts Caddy's socket and the rightmost `X-Forwarded-For` hop. Guests cannot mint a client IP.
- `/metrics` is not on `:80`/`:443`. Optional Caddy `basic_auth` is documented in `infrastructure/Caddyfile`.
- Invite IP allowlist: uncomment `remote_ip` in the Caddyfile. No CDN/WAF product.

## Rollback

1. Keep the previous image digest when you deploy (`node scripts/record-image-digests.mjs`).
2. `/DRAIN`, wait for `/ready` to report `drain`, then stop the container.
3. Start the previous digest with the same `DATA_DIR` volume. Bans and config come back; live matches do not.
4. `/UNDRAIN` if the restored process came up drained.

## Caching and compression

- Caddy already serves the built web client. Enable gzip/zstd at the edge for `/assets`.
- Map packages are content-addressed; a client may keep a hash-matched copy.
