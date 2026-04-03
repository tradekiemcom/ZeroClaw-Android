# 📘 ZeroClaw-Android: Technical Manual & User Guide

This document provides a deep dive into the architecture, purpose, and usage of the ZeroClaw-Android automation hub.

---

## 🏗 Project Architecture

ZeroClaw-Android is a multi-layered automation stack running inside **Termux** on Android:

1.  **ZeroClaw Core**: The AI agent engine (original zeroclaw-labs project).
2.  **Admin Dashboard**: A zero-dependency Node.js server for remote status and service control.
3.  **Cloudflare Tunnel**: Secure, NAT-traversing entry point for remote access.
4.  **Termux:Boot**: Persistence layer ensuring all services start on phone reboot.
5.  **Android Shield**: OS-level optimizations (Phantom Process Killer / Battery Whitelist).

---

## 🖥 The Admin Dashboard: Why and How?

### Purpose
Operating a CLI-only agent on a small smartphone screen can be tedious. The dashboard ([http://localhost:7643](http://localhost:7643)) provides:
- **One-Touch Control**: Start/Stop services (Tunnel, SSH, etc.) without typing long commands.
- **Health Monitoring**: Real-time view of RAM, CPU load, and Uptime.
- **Service Status**: Visual indicators to confirm if your background services are alive.

---

## 📱 Using ZeroClaw on your Phone (CLI)

While the dashboard manages the *environment*, the **CLI** is where you interact with the AI agent.

### 1. Initial Setup
After running `./setup.sh`, you must initialize your AI provider (OpenRouter, OpenAI, etc.):
```bash
zeroclaw onboard
```
*Tip: Use `zeroclaw onboard --tui` for a more visual setup wizard.*

### 2. Everyday Commands
Once configured, use these commands in Termux:

| Command | Purpose |
|---|---|
| `zeroclaw agent` | Start an interactive chat session with the AI. |
| `zeroclaw agent -m "Task"` | Run a single one-off task/message. |
| `zeroclaw status` | Check current configuration and system summary. |
| `zeroclaw doctor` | Run diagnostics if something feels slow or broken. |
| `zeroclaw models refresh` | Update the list of available models from your provider. |

### 3. File Interaction
ZeroClaw can see and edit files in your Termux home directory. You can ask it to:
- "Create a python script to scrape X website."
- "List all files in my downloads folder."
- "Organize my projects directory."

---

## 🛡 Android Optimization (The Shield)

### Phantom Process Killer
Android 12+ (especially Samsung) includes a "Phantom Process Killer" that terminates background processes (like Node.js or ZeroClaw) if they use too many resources.
- **The Fix**: The `shield/setup-shield.sh` script (run via ADB from PC) disables this killer and whitelists Termux, allowing your agent to run 24/7.

### Pro-Tip: Use SSH
For a much better experience, start the SSH service via the Dashboard and connect from your PC/Mac:
```bash
ssh -p 8022 [your-phone-ip]
```
This gives you a full-sized keyboard and screen to interact with ZeroClaw while it runs on the phone.
