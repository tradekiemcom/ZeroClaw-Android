# ZeroClaw-Android — Termux Automation setup

Automated environment for the Galaxy Note 10+ using Termux.

## 🚀 Quick Start (on Termux Android)

1.  **Clone**: 
    ```bash
    git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
    cd ZeroClaw-Android
    ```
2.  **Setup**:
    ```bash
    chmod +x setup.sh && ./setup.sh
    ```
3.  **Shield (on PC/Mac)**:
    Connect your phone via USB and run this script from your computer:
    ```bash
    chmod +x shield/setup-shield.sh && ./shield/setup-shield.sh
    ```
4.  **Launch**:
    Services (SSHD, Tunnel, Dashboard) will start immediately and automatically after each Android restart thanks to `Termux:Boot`.

## 🛡 Features
- **Auto-Boot**: Persistence via `Termux:Boot`.
- **Battery & Process Shield**: Prevents Android 12+ from killing Termux processes.
- **Admin Dashboard**: Lightweight monitoring and service control ([http://localhost:7643](http://localhost:7643)).
- **Cloudflare Tunnel**: Remote access via your domain configured on Cloudflare.
