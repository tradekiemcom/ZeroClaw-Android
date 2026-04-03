# ZeroClaw-Android — Termux/Android Optimization Rules

These rules apply to any AI agent working within the ZeroClaw-Android project.

## 📱 Termux Environment Constraints

- **Paths**: ALWAYS use absolute paths for scripts, starting with `/data/data/com.termux/files/usr/` for system binaries or `/data/data/com.termux/files/home/` for home-dir related tasks.
- **Dependencies**: Use `pkg install` instead of `apt-get` or other package managers.
- **Node.js**: Use `nodejs-lts` for stability.
- **Architecture**: The target is `aarch64` (ARM64). Avoid any x86/x64 specific binaries.

## 🛡 Battery & Process Persistence

- **Wake Lock**: Every boot script must acquire a wake-lock using `termux-wake-lock` to prevent the CPU from sleeping.
- **Persistence**: Any long-running service (Tunnel, Dashboard, etc.) MUST be registered in the `~/.termux/boot/` directory via a shell wrapper.
- **Shielding**: Advise the user to perform the one-time ADB shield setup to prevent the "Phantom Process Killer" from terminating background Node.js processes.

## ☁ Cloudflare Tunnel Management

- **Binary**: Always use the ARM64 binary downloaded from official Cloudflare GitHub releases.
- **Security**: Tunnel tokens and credentials must be treated as sensitive.

## 🐚 Shell Scripting Style

- Use `#!/data/data/com.termux/files/usr/bin/bash` for the shebang.
- Implement robust error handling (`set -e`).
- Use informative logging/echo statements for debugging.
