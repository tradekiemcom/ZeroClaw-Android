#!/bin/bash
# ============================================================
# ZeroClaw Process & Battery Shield
# Run from PC/Mac via ADB (one-time setup)
# Target: Galaxy Note 10+ (Android 12+)
# ============================================================

set -e

echo "╔══════════════════════════════════════════╗"
echo "║   ZeroClaw Shield Setup (via ADB)        ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# Check ADB connection
if ! command -v adb &>/dev/null; then
    echo "✗ ADB not found. Install Android SDK Platform-Tools."
    echo "  macOS:   brew install android-platform-tools"
    echo "  Windows: scoop install adb"
    exit 1
fi

DEVICE_COUNT=$(adb devices | grep -c "device$")
if [ "$DEVICE_COUNT" -eq 0 ]; then
    echo "✗ No device connected. Enable USB Debugging and connect."
    exit 1
fi

DEVICE_MODEL=$(adb shell getprop ro.product.model 2>/dev/null | tr -d '\r')
ANDROID_VER=$(adb shell getprop ro.build.version.release 2>/dev/null | tr -d '\r')
echo "✓ Device: $DEVICE_MODEL (Android $ANDROID_VER)"
echo ""

# ── 1. Disable Phantom Process Killer ────────────────────────
echo "[1/5] Disabling Phantom Process Killer..."
adb shell "settings put global settings_enable_monitor_phantom_procs false" 2>/dev/null && \
    echo "  ✓ Phantom process monitoring disabled" || \
    echo "  ○ settings_enable_monitor_phantom_procs (may not exist on this version)"

adb shell "device_config set_sync_disabled_for_tests persistent" 2>/dev/null && \
    echo "  ✓ DeviceConfig sync disabled for persistence" || \
    echo "  ○ device_config sync (may require higher API)"

adb shell "device_config put activity_manager max_phantom_processes 2147483647" 2>/dev/null && \
    echo "  ✓ Max phantom processes set to MAX_INT" || \
    echo "  ○ max_phantom_processes (fallback method below)"

# Fallback for older approaches
adb shell "cmd device_config put activity_manager max_phantom_processes 2147483647" 2>/dev/null && \
    echo "  ✓ (Fallback) cmd device_config applied" || \
    echo "  ○ cmd fallback not needed"

echo ""

# ── 2. Whitelist Termux from Battery Optimization ────────────
echo "[2/5] Whitelisting Termux from battery optimization..."
adb shell "dumpsys deviceidle whitelist +com.termux" 2>/dev/null && \
    echo "  ✓ com.termux added to battery whitelist" || \
    echo "  ✗ Failed to whitelist com.termux"

adb shell "dumpsys deviceidle whitelist +com.termux.boot" 2>/dev/null && \
    echo "  ✓ com.termux.boot added to battery whitelist" || \
    echo "  ○ com.termux.boot not installed"

adb shell "dumpsys deviceidle whitelist +com.termux.api" 2>/dev/null && \
    echo "  ✓ com.termux.api added to battery whitelist" || \
    echo "  ○ com.termux.api not installed"

echo ""

# ── 3. Disable USAP (Unspecialized App Process) ─────────────
echo "[3/5] Disabling USAP zygote pool..."
adb shell "setprop persist.device_config.runtime_native.usap_pool_enabled false" 2>/dev/null && \
    echo "  ✓ USAP pool disabled" || \
    echo "  ○ USAP setting (may require root)"

echo ""

# ── 4. Disable Samsung-specific App Sleep ────────────────────
echo "[4/5] Disabling Samsung app sleep/deep sleep..."
# Samsung-specific: prevent putting Termux in "sleeping apps" or "deep sleeping apps"
adb shell "cmd appops set com.termux RUN_IN_BACKGROUND allow" 2>/dev/null && \
    echo "  ✓ RUN_IN_BACKGROUND allowed for Termux" || \
    echo "  ○ RUN_IN_BACKGROUND (may not apply)"

adb shell "cmd appops set com.termux RUN_ANY_IN_BACKGROUND allow" 2>/dev/null && \
    echo "  ✓ RUN_ANY_IN_BACKGROUND allowed for Termux" || \
    echo "  ○ RUN_ANY_IN_BACKGROUND (may not apply)"

echo ""

# ── 5. Lock Termux in Recent Apps ────────────────────────────
echo "[5/5] Additional optimizations..."
# Prevent Android from killing Termux on low memory
adb shell "cmd appops set com.termux START_FOREGROUND allow" 2>/dev/null && \
    echo "  ✓ START_FOREGROUND allowed" || echo "  ○ Already set"

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║  ✓ Shield setup complete!                ║"
echo "║                                          ║"
echo "║  IMPORTANT: Lock Termux in Recent Apps   ║"
echo "║  (long-press Termux in recents → Lock)   ║"
echo "║                                          ║"
echo "║  Run verify-shield.sh to confirm.        ║"
echo "╚══════════════════════════════════════════╝"
