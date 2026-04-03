# 🐾 ZeroClaw-Android

**Termux-based automation system for Galaxy Note 10+**

A self-healing, always-on Android agent platform with Cloudflare tunnel access, admin dashboard, and boot persistence.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 Galaxy Note 10+                  │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │ Termux   │  │ Shield   │  │ Termux:Boot   │  │
│  │ Runtime  │  │ (ADB)    │  │ Auto-Start    │  │
│  └────┬─────┘  └──────────┘  └───────┬───────┘  │
│       │                              │           │
│  ┌────▼──────────────────────────────▼────────┐  │
│  │           ZeroClaw Services                │  │
│  │  ┌────────────┐  ┌─────────────────────┐   │  │
│  │  │ cloudflared│  │  Admin Dashboard    │   │  │
│  │  │ tunnel     │  │  :7643              │   │  │
│  │  └─────┬──────┘  └─────────────────────┘   │  │
│  └────────┼───────────────────────────────────┘  │
└───────────┼──────────────────────────────────────┘
            │
    ┌───────▼───────┐
    │  Cloudflare   │
    │  claw.iz.life │
    └───────────────┘
```

## Quick Start

```bash
# 1. Clone into Termux
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android

# 2. Run master setup
chmod +x setup.sh
./setup.sh

# 3. Start services
./zeroclaw.sh start

# 4. Access dashboard
# Local:  http://localhost:7643
# Remote: https://claw.iz.life
```

## Components

| Module | Description |
|---|---|
| `boot/` | Termux:Boot auto-start scripts |
| `shield/` | ADB scripts to disable Phantom Process Killer & battery optimization |
| `tunnel/` | Cloudflare tunnel config for `claw.iz.life` |
| `dashboard/` | Zero-dependency Node.js admin panel with dark UI |
| `setup.sh` | Master installer |
| `zeroclaw.sh` | CLI: `start`, `stop`, `status`, `logs` |

## Default Credentials

- **Username**: `admin`
- **Password**: `ZeroClaw@2026`
- Change password via dashboard Settings after first login.

## Requirements

- Galaxy Note 10+ (or Android 12+ device)
- Termux (F-Droid version)
- Termux:Boot (F-Droid)
- Termux:API (F-Droid)
- Node.js (`pkg install nodejs-lts`)
- ADB access (for shield setup — one-time from PC)

## License

MIT
