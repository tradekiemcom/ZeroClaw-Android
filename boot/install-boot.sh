#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw Boot Installer
# Installs Termux:Boot integration
# ============================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BOOT_DIR="$HOME/.termux/boot"
BOOT_SCRIPT="$BOOT_DIR/zeroclaw-boot.sh"

echo "╔══════════════════════════════════════════╗"
echo "║    ZeroClaw Boot Installer               ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# Check if Termux:Boot is installed
if ! command -v termux-boot &>/dev/null && [ ! -d "$HOME/.termux/boot" ]; then
    echo "⚠  Termux:Boot not detected."
    echo "   Install from F-Droid: https://f-droid.org/packages/com.termux.boot/"
    echo "   After installing, open the app once to initialize."
    echo ""
    echo "   Creating boot directory anyway..."
fi

# Create boot directory
mkdir -p "$BOOT_DIR"
echo "✓ Boot directory: $BOOT_DIR"

# Copy boot script
cp "$SCRIPT_DIR/termux-boot.sh" "$BOOT_SCRIPT"
chmod 755 "$BOOT_SCRIPT"
echo "✓ Boot script installed: $BOOT_SCRIPT"

# Create log directory
mkdir -p "$HOME/zeroclaw/logs"
echo "✓ Log directory: $HOME/zeroclaw/logs"

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║  ✓ Boot installation complete!           ║"
echo "║                                          ║"
echo "║  ZeroClaw will auto-start on reboot.     ║"
echo "║  Test: ~/.termux/boot/zeroclaw-boot.sh   ║"
echo "╚══════════════════════════════════════════╝"
