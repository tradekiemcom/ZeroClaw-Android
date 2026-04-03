#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
# ZeroClaw-Android — Battery & Process Shield
# (To be run from PC/Mac via ADB)
# ============================================================

set -e

echo "🛡 ZeroClaw-Android: Initializing Battery & Process Shield..."

# Disable Phantom Process Killer (Android 12+)
adb shell settings put global settings_config_tracker_max_phantom_processes 2147483647
adb shell settings put global phantom_process_killer_enable false

# Whitelist Termux from Battery Optimization
# Note: User may still need to do this manually in Settings > Apps > Termux > Battery
adb shell dumpsys deviceidle whitelist +com.termux

# Optional: Set Termux to 'Unrestricted' battery usage (Samsung specific)
# adb shell cmd battery-stats --unrestricted com.termux 

echo "✅ Shield configuration applied! Please check Termux Battery settings for 'Unrestricted' manually."
