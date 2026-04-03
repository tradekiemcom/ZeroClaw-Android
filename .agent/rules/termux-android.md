# Termux & Android Optimization Rules

**Target device: Samsung Galaxy Note 10+ (SM-N975F) — Android 12+ / API 31+**

## Package Management

- Use `pkg install` — NEVER `apt install` directly.
- Always run `pkg update && pkg upgrade` before installing.
- Install from Termux repos only. If unavailable, build from source for `aarch64`.

## Path Conventions

| Variable | Path |
|---|---|
| `$HOME` | `/data/data/com.termux/files/home` |
| `$PREFIX` | `/data/data/com.termux/files/usr` |
| `$TMPDIR` | `/data/data/com.termux/files/usr/tmp` |
| Boot scripts | `~/.termux/boot/` |
| Termux config | `~/.termux/` |

- **NEVER** hardcode `/usr/bin` or `/etc` — these don't exist in Termux.
- Use `$PREFIX/bin`, `$PREFIX/etc` instead.
- Scripts must use `#!/data/data/com.termux/files/usr/bin/bash` or `#!/usr/bin/env bash`.

## Architecture

- **CPU**: ARM64 (aarch64) — always use ARM64 binaries.
- **No root assumed**: Scripts should work without root unless explicitly stated.
- **No systemd**: Use `nohup`, `setsid`, or Termux:Boot for persistence.
- **No cron**: Use `crond` from `termux-services` or `termux-job-scheduler`.

## Battery & Process Survival

- Always acquire wake-lock: `termux-wake-lock`
- Termux notification must stay visible (foreground service).
- Phantom Process Killer must be disabled via ADB (see `shield/`).
- Request battery optimization exemption.

## Networking

- Default ports may conflict with Android services. Use ports > 7000.
- `localhost` and `127.0.0.1` both work inside Termux.
- For external access, use Cloudflare Tunnel (never expose ports directly).

## File Permissions

- Scripts must be `chmod +x` to execute.
- Termux:Boot scripts must be executable to auto-run.
- Avoid `777` — use `755` for scripts, `644` for configs.

## Node.js in Termux

- Install: `pkg install nodejs-lts`
- Native modules may need: `pkg install python make clang`
- Prefer zero-dependency implementations when possible.
- Use `process.env.PREFIX` to detect Termux environment.

## Logging

- Log to `$HOME/zeroclaw/logs/` — NOT to `/var/log/`.
- Rotate logs manually or with a simple script.
- Include timestamps in ISO 8601 format.
