# iZFx.Trade – Hướng Dẫn Cấu Hình & Sử Dụng

> **iZFx.Trade** là Trading Execution Hub viết bằng Rust, kết nối trực tiếp với cTrader Open API.  
> Điều khiển đa tài khoản, đa bot, đa nguồn qua Telegram và REST API.

---

## Mục Lục

1. [Yêu cầu hệ thống](#1-yêu-cầu-hệ-thống)
2. [Cài đặt](#2-cài-đặt)
3. [Cấu hình](#3-cấu-hình)
4. [Khởi động](#4-khởi-động)
5. [Telegram Bot – Danh sách lệnh](#5-telegram-bot--danh-sách-lệnh)
6. [Quản lý API Keys](#6-quản-lý-api-keys)
7. [REST API Reference](#7-rest-api-reference)
8. [Nguồn lệnh (Sources)](#8-nguồn-lệnh-sources)
9. [Risk Engine](#9-risk-engine)
10. [Tích hợp ZeroClaw](#10-tích-hợp-zeroclaw)
11. [TradingView Integration](#11-tradingview-integration)
12. [MT5 Integration](#12-mt5-integration)
13. [Troubleshooting](#13-troubleshooting)
14. [Price Feed API](#14-price-feed-api)

---

## 1. Yêu Cầu Hệ Thống

| Môi trường | Yêu cầu |
|-----------|---------|
| **Mac M1/M2** | Rust 1.70+, 500MB RAM |
| **Android Termux** | RAM ≥ 3GB, Storage ≥ 2GB, Rust (via pkg) |
| **VPS Linux** | Ubuntu 20.04+, 512MB RAM |

**Dependencies:**
- `cargo` (Rust build tool)
- `sqlite3` (tự động tạo database file)
- Internet để kết nối Telegram + cTrader

---

## 2. Cài Đặt

### 2.1 Cài đặt tự động (Termux)

```bash
cd ZeroClaw-Android/ctrader
bash scripts/iztrade-setup.sh
```

Script sẽ hỏi 5 bước cấu hình và tự động build.

### 2.2 Cài đặt thủ công

```bash
# Clone project
git clone https://github.com/tradekiemcom/ZeroClaw-Android
cd ZeroClaw-Android/ctrader

# Copy template config
cp .env.example .env

# Điền thông tin vào .env (xem mục 3)
nano .env

# Build
cargo build --release

# Chạy
./target/release/iztrade
```

### 2.3 Cài đặt trên Android Termux

```bash
# Cài dependencies
pkg install rust clang make

# Linker fix cho Android
export CC=clang && export CXX=clang++
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=clang
export RUSTFLAGS="-C linker=clang"

# Build (15-20 phút lần đầu)
cargo build --release -j 1

# Copy binary vào PATH
cp target/release/iztrade $PREFIX/bin/iztrade
```

---

## 3. Cấu Hình

### 3.1 File `.env`

Tạo file `.env` trong thư mục `ctrader/`:

```env
# == cTrader Open API ==========================
CTRADER_CLIENT_ID=12739_your_client_id
CTRADER_SECRET=your_secret_key
CTRADER_HOST=openapi.ctrader.com
CTRADER_PORT=5035
CTRADER_MODE=mock     # mock | live

# == Telegram Bot ==============================
TELEGRAM_BOT_TOKEN=6972526694:AAHyyy...
TELEGRAM_ADMIN_IDS=975318323        # ID Telegram admin (nhiều ID: 111,222,333)
TELEGRAM_NOTIFY_CHAT_ID=@TradeKiemGold  # Channel/Group nhận thông báo

# == REST API ==================================
API_KEY=izt_master_secret_key      # Master key (tạo thêm per-client keys qua /key add)
API_PORT=7381

# == Database ==================================
DATABASE_URL=sqlite://iztrade.db

# == Logging ===================================
LOG_LEVEL=info                      # error | warn | info | debug
```

### 3.2 Lấy cTrader credentials

1. Truy cập: https://openapi.ctrader.com/apps
2. Đăng nhập bằng tài khoản cTrader
3. Tạo Application mới
4. Copy **Client ID** và **Secret**

### 3.3 Tạo Telegram Bot

1. Nhắn `@BotFather` trên Telegram
2. Gõ `/newbot` và làm theo hướng dẫn
3. Copy **Bot Token** (`123456:AAHxxx...`)
4. Lấy Telegram ID của bạn: nhắn `@userinfobot`

### 3.4 Lấy Channel/Group ID

- **@username**: Dùng trực tiếp nếu channel có username (vd: `@TradeKiemGold`)
- **Numeric ID**: Forward một tin nhắn từ group vào `@getidsbot`

---

## 4. Khởi Động

```bash
# Chạy trực tiếp
iztrade

# Chạy với mode mock (test)
CTRADER_MODE=mock iztrade

# Chạy background trên Termux
nohup iztrade > iztrade.log 2>&1 &

# Xem logs
tail -f iztrade.log
```

**Khi khởi động thành công:**
```
╔══════════════════════════════════════════╗
║   iZFx.Trade – Trading Execution Hub    ║
║   Version 2.0.0  │  Multi-Account       ║
╚══════════════════════════════════════════╝

[START] Starting iZFx.Trade v2.0.0
Mode: MOCK
API Port: 7381
[OK] Database ready
[MOCK] cTrader client initialized
REST API listening on http://0.0.0.0:7381
[INFO] Telegram Bot started | Polling...
[SYSTEM] Risk Monitor started (interval: 60s)
```

---

## 5. Telegram Bot – Danh Sách Lệnh

### 5.1 Lấy Telegram User ID của bạn
Nhắn `@userinfobot` để lấy ID. Điền vào `TELEGRAM_ADMIN_IDS` trong `.env`.

### 5.2 System Commands

| Lệnh | Mô tả |
|------|--------|
| `/help` | Xem danh sách lệnh |
| `/status` | **Trạng thái chi tiết hệ thống** |
| `/accounts` | Danh sách tài khoản |
| `/bots` | Danh sách bots |

**Ví dụ output `/status`:**
```
[SYSTEM STATUS] iZFx.Trade
--------------------
Mode: Live | Uptime: 2h 30m

Accounts:
- Total: 3 (Real: 1 | Demo: 2)
- Connected: 3/3 | Autotrade: 3/3

Finance (Real):
- Balance: $10,500.00
- Equity: $10,320.50
- [LOSS] Float P&L: $-179.50
- [PROFIT] Daily P&L: $+250.00

Bots: 3/5 active
Positions: 4
API Keys: 3/4 active
```

### 5.3 Autotrade Control

| Lệnh | Mô tả |
|------|--------|
| `/a` | Bật autotrade tất cả accounts |
| `/d` | Tắt autotrade tất cả accounts |

### 5.4 Bot Control

| Lệnh | Mô tả |
|------|--------|
| `/on <bot_id>` | Bật bot theo ID |
| `/off <bot_id>` | Tắt bot theo ID |

```
/on gold_scalper
/off trend_bot
```

### 5.5 Close Lệnh

| Lệnh | Mô tả |
|------|--------|
| `/c` | Đóng TẤT CẢ lệnh + tắt autotrade |
| `/c <bot_id>` | Đóng lệnh của một bot cụ thể |

```
/c                    → Đóng tất cả
/c gold_scalper       → Đóng lệnh của gold_scalper
```

### 5.6 Xem Positions & Report

| Lệnh | Mô tả |
|------|--------|
| `/p` | Xem positions đang mở + float P&L |
| `/r` | Account report (P&L theo account) |
| `/rp` | Report theo bot |

### 5.7 Trade Trực Tiếp

```
Cú pháp:
/buy <SYMBOL> <VOLUME> <BOT_ID> [sl=<SL>] [tp=<TP>]
/sell <SYMBOL> <VOLUME> <BOT_ID> [sl=<SL>] [tp=<TP>]

Ví dụ:
/buy XAUUSD 0.1 gold_scalper sl=3280 tp=3350
/sell EURUSD 0.5 trend_bot
/buy BTCUSD 0.01 crypto_bot tp=90000
```

---

## 6. Quản Lý API Keys

Mỗi ứng dụng kết nối (MT5, TradingView, ZeroClaw...) nên dùng **key riêng** để:
- Theo dõi số lần sử dụng
- Có thể bật/tắt độc lập
- Xác định nguồn của lệnh

### 6.1 Lệnh Telegram

| Lệnh | Mô tả |
|------|--------|
| `/key list` | Xem tất cả API keys |
| `/key add <tên> <source>` | Tạo key mới |
| `/key del <id>` | Xóa key (dùng 8 ký tự đầu của ID) |
| `/key on <id>` | Bật key |
| `/key off <id>` | Tắt key |

### 6.2 Ví Dụ

```
# Tạo key cho MT5 EA
/key add MT5_Note10 MT5

# Tạo key cho TradingView
/key add TradingView_Gold TRADINGVIEW

# Tạo key cho ZeroClaw
/key add ZeroClaw_Agent ZEROCLAW

# Xem danh sách
/key list

# Tắt key MT5 (dùng 8 ký tự đầu ID)
/key off a1b2c3d4

# Xóa key cũ
/key del a1b2c3d4
```

### 6.3 Output `/key list`

```
API Keys (3)
--------------------
[ON] a1b2c3d4 - MT5_Note10 (MT5)
   KEY: izt_4a9f2b8c1d3e5f6a...
   STATS: 142 requests | 04/11 14:32

[ON] b2c3d4e5 - TradingView_Gold (TRADINGVIEW)
   KEY: izt_7b8c9d0e1f2a3b4c...
   STATS: 28 requests | 04/11 10:15

[OFF] c3d4e5f6 - Old_MT5 (MT5)
   KEY: izt_1a2b3c4d5e6f7a8b...
   STATS: 0 requests | Chưa dùng
--------------------
Management: /key on|off|del <id>
```

---

## 7. REST API Reference

**Base URL:** `http://localhost:7381`

**Authentication:**
```http
Authorization: Bearer <API_KEY>
```

### 7.1 Health Check

```http
GET /health
```

Response:
```json
{
  "status": "ok",
  "service": "iZFx.Trade",
  "version": "2.0.0"
}
```

### 7.2 Gửi Lệnh Trade

```http
POST /api/order
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

**Body:**
```json
{
  "bot_id": "gold_scalper",
  "action": "OPEN",
  "account_scope": "all",
  "account_ids": [],
  "symbol": "XAUUSD",
  "side": "buy",
  "volume": 0.1,
  "sl": 3280.0,
  "tp": 3350.0
}
```

**Actions:**
- `OPEN` – Mở lệnh mới
- `CLOSE` – Đóng lệnh theo bot
- `CLOSE_ALL` – Đóng tất cả + tắt autotrade
- `ENABLE_BOT` / `DISABLE_BOT`
- `ENABLE_AUTOTRADE` / `DISABLE_AUTOTRADE`

**account_scope:**
- `all` – Tất cả accounts
- `single` – Một account (dùng `account_ids: [123]`)
- `list` – Nhiều accounts (dùng `account_ids: [123, 456]`)

### 7.3 Xem Positions

```http
GET /api/positions
```

### 7.4 Danh Sách Accounts

```http
GET /api/accounts
```

### 7.5 Bot Control

```http
POST /api/bots/gold_scalper/enable
POST /api/bots/gold_scalper/disable
```

### 7.6 Autotrade

```http
POST /api/autotrade/on
POST /api/autotrade/off
```

### 7.7 Report

```http
GET /api/report
```

---

## 8. Nguồn Lệnh (Sources)

| Source | Description |
|--------|--------|
| `TELEGRAM` | Telegram Bot |
| `MT5` | MetaTrader 5 EA |
| `TRADINGVIEW` | TradingView Webhook |
| `ZEROCLAW` | ZeroClaw AI Agent |
| `OPENCLAW` | OpenClaw |
| `WEBHOOK` | Generic Webhook |
| `WEB` | Web Dashboard |
| `API` | REST API |
| `CRON` | Scheduled Task |

Nguồn được ghi lại cùng với mỗi lệnh và hiển thị trong positions/notifications.

---

## 9. Risk Engine

Risk Monitor tự động kiểm tra mỗi 60 giây.

### 9.1 Cấu hình Risk theo Account

Qua database hoặc API:
- `daily_target_profit`: Khi đạt → tắt autotrade account đó
- `daily_max_loss`: Khi vượt → tắt autotrade account đó

### 9.2 Cấu hình Risk theo Bot

- `daily_target_profit`: Bot sẽ bị pause khi đạt target
- `daily_max_loss`: Bot sẽ bị pause khi lỗ quá mức

### 9.3 Daily Reset

P&L hàng ngày được reset lúc **00:00 UTC** tự động.

### 9.4 Risk Alert Telegram

Khi trigger risk, hệ thống gửi alert vào group:
```
[RISK ALERT]

Account: #1 Main_Account
STATS PnL: -500.00
[INFO] Daily max loss trigger: -500.00

[OFF] Autotrade has been disabled automatically.
```

---

## 10. Tích Hợp ZeroClaw

ZeroClaw gửi lệnh đến iZFx.Trade qua REST API với source `ZEROCLAW`.

### 10.1 Cấu hình ZeroClaw

Thêm vào config ZeroClaw:
```toml
[iztrade]
enabled = true
url = "http://localhost:7381"
api_key = "izt_zeroclaw_key_here"
```

### 10.2 Ví dụ gửi lệnh từ ZeroClaw

```bash
curl -X POST http://localhost:7381/api/order \
  -H "Authorization: Bearer izt_zeroclaw_key" \
  -H "Content-Type: application/json" \
  -d '{
    "bot_id": "zeroclaw_ai",
    "action": "OPEN",
    "symbol": "XAUUSD",
    "side": "buy",
    "volume": 0.1,
    "sl": 3280,
    "tp": 3350
  }'
```

---

## 11. TradingView Integration

### 11.1 Tạo API Key cho TradingView

```
/key add TradingView_Gold TRADINGVIEW
```

Lưu key được tạo: `izt_xxxxxxxxxxxxxxxxxxxxxxxx`

### 11.2 Cấu hình TradingView Alert

Trong TradingView → Alert → Webhook URL:
```
http://YOUR_SERVER_IP:7381/api/order
```

**Headers:** (TradingView không hỗ trợ custom headers, dùng token trong body)

**Alert Message (JSON):**
```json
{
  "bot_id": "tv_gold_signal",
  "action": "OPEN",
  "symbol": "{{ticker}}",
  "side": "{{strategy.order.action}}",
  "volume": 0.1,
  "sl": {{strategy.order.price}} - 20,
  "tp": {{strategy.order.price}} + 40
}
```

> **Lưu ý:** API Key của TradingView nên được nhúng vào body hoặc tạo một endpoint riêng cho webhook không cần auth (planned feature).

---

## 12. MT5 Integration

### 12.1 Tạo API Key cho MT5

```
/key add MT5_Note10 MT5
```

### 12.2 Gửi lệnh từ MT5 EA (MQL5)

```mql5
string api_url = "http://192.168.1.100:7381/api/order";
string api_key = "izt_xxxxxxxxxxxxxxxxxxxxxxxx";

string payload = StringFormat(
  "{\"bot_id\":\"mt5_ea\",\"action\":\"OPEN\","
  "\"symbol\":\"%s\",\"side\":\"%s\","
  "\"volume\":%.2f,\"sl\":%.5f,\"tp\":%.5f}",
  Symbol(),
  type == OP_BUY ? "buy" : "sell",
  lots, sl, tp
);

string headers = "Content-Type: application/json\r\nAuthorization: Bearer " + api_key;
char post[], result[];
StringToCharArray(payload, post);
WebRequest("POST", api_url, headers, 5000, post, result, headers);
```

---

## 13. Troubleshooting

### Error: Address already in use

```bash
# Tìm process đang dùng port
ss -tlnp | grep 7381
# hoặc
lsof -i :7381

# Kill process
kill -9 <PID>
```

### Error: Telegram Bot not responding

1. Kiểm tra `TELEGRAM_BOT_TOKEN` trong `.env`
2. Đảm bảo bot đã được activate bởi `@BotFather`
3. Kiểm tra `TELEGRAM_ADMIN_IDS` có đúng ID của bạn không
4. Xem logs: `LOG_LEVEL=debug iztrade`

### Error: Invalid API key

- Đảm bảo Bearer token đúng
- Key có thể đã bị tắt: `/key list` để kiểm tra
- Dùng Master Key từ `.env` → `API_KEY`

### Error: Build failed on Termux

```bash
# Đảm bảo đủ dung lượng
df -h $HOME

# Cài đủ dependencies
pkg install rust clang make binutils

# Set linker
export CC=clang
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=clang
export RUSTFLAGS="-C linker=clang"

# Build với RAM tiết kiệm
cargo build --release -j 1
```

### Error: SQLite locked

```bash
# Xóa lock file
rm -f iztrade.db-wal iztrade.db-shm

# Restart iztrade
```

---

## 14. Price Feed API

Module `ctrader` có tích hợp sẵn **Price Feed API** cho phép bất kỳ ứng dụng nào (ZeroClaw agent, TradingView, bot khác...) lấy giá thị trường real-time mà **không cần Bearer token**.

### Endpoint

| Method | URL | Auth | Description |
|--------|-----|------|-------|
| `GET` | `/api/prices` | PUBLIC | List all prices |
| `GET` | `/api/prices/{symbol}` | PUBLIC | Single price |
| `POST` | `/api/prices/update` | [AUTH] Bearer | Push price |

### Ví dụ sử dụng

**Lấy giá vàng (XAUUSD):**
```bash
curl http://localhost:7381/api/prices/XAUUSD
```

**Phản hồi:**
```json
{
  "success": true,
  "symbol": "XAUUSD",
  "bid": 3299.50,
  "ask": 3300.50,
  "mid": 3300.00,
  "spread": 1.0,
  "source": "ctrader",
  "timestamp": "2026-04-12T12:00:00Z",
  "age_secs": 3,
  "stale": false
}
```

> **Lưu ý:** Giá mặc định là **mock** khi chưa kết nối cTrader thật. Khi kết nối cTrader OpenAPI, giá sẽ được cập nhật realtime qua tick subscription và trường `"source"` sẽ đổi thành `"ctrader"`.

---

## Version History

| Version | Thay đổi |
|---------|---------|
| v2.0.0 | Initial release: Multi-account, Telegram Bot, REST API, Risk Engine, SQLite |
| v2.1.0 | Per-client API keys, /status, /key management, TradingView/OpenClaw/Webhook sources |

---

*iZFx.Trade – Powered by TradeKiem.Com*
