#!/bin/bash
# ============================================================
# ZeroClaw Shield Verification
# Run from PC/Mac via ADB to confirm settings
# ============================================================

echo "╔══════════════════════════════════════════╗"
echo "║   ZeroClaw Shield Verification           ║"
echo "╚══════════════════════════════════════════╝"
echo ""

if ! command -v adb &>/dev/null; then
    echo "✗ ADB not found."
    exit 1
fi

PASS=0
WARN=0
FAIL=0

check() {
    local label="$1"
    local cmd="$2"
    local expected="$3"
    local result
    result=$(adb shell "$cmd" 2>/dev/null | tr -d '\r\n')

    if [ "$result" = "$expected" ]; then
        echo "  ✓ $label = $result"
        PASS=$((PASS + 1))
    elif [ -n "$result" ]; then
        echo "  ⚠ $label = $result (expected: $expected)"
        WARN=$((WARN + 1))
    else
        echo "  ✗ $label — could not read"
        FAIL=$((FAIL + 1))
    fi
}

echo "── Phantom Process Killer ────────────────"
check "Monitor phantom procs" \
    "settings get global settings_enable_monitor_phantom_procs" \
    "0"

check "Max phantom processes" \
    "device_config get activity_manager max_phantom_processes" \
    "2147483647"

echo ""
echo "── Battery Whitelist ─────────────────────"
WHITELIST=$(adb shell "dumpsys deviceidle whitelist" 2>/dev/null)

if echo "$WHITELIST" | grep -q "com.termux"; then
    echo "  ✓ com.termux is whitelisted"
    PASS=$((PASS + 1))
else
    echo "  ✗ com.termux NOT whitelisted"
    FAIL=$((FAIL + 1))
fi

if echo "$WHITELIST" | grep -q "com.termux.boot"; then
    echo "  ✓ com.termux.boot is whitelisted"
    PASS=$((PASS + 1))
else
    echo "  ○ com.termux.boot not whitelisted (optional)"
    WARN=$((WARN + 1))
fi

echo ""
echo "── Background Permissions ────────────────"
RUN_BG=$(adb shell "cmd appops get com.termux RUN_IN_BACKGROUND" 2>/dev/null | tr -d '\r')
if echo "$RUN_BG" | grep -q "allow"; then
    echo "  ✓ RUN_IN_BACKGROUND: allowed"
    PASS=$((PASS + 1))
else
    echo "  ⚠ RUN_IN_BACKGROUND: $RUN_BG"
    WARN=$((WARN + 1))
fi

echo ""
echo "══════════════════════════════════════════"
echo "  Results: ✓ $PASS passed | ⚠ $WARN warnings | ✗ $FAIL failed"
echo "══════════════════════════════════════════"

if [ "$FAIL" -gt 0 ]; then
    echo "  → Run setup-shield.sh to fix failed checks."
    exit 1
fi
