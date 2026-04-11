# iZtrader – Production System Specification (v2)

---

## 1. Overview

**iZtrader** là Trading Execution Hub trung tâm:

* được viết bằng rush dựa trên ctrader open api 
* Thay thế hoàn toàn cBot
* Control toàn bộ hệ thống trading
* Multi-account / Multi-bot / Multi-source
* Điều khiển qua Telegram + API

---

## 2. Core Objectives

* Nhận lệnh từ nhiều nguồn (Telegram, Web, MT5, ZeroClaw, AI…)
* Gắn định danh bot vào từng lệnh
* Quản lý autotrade & bot theo account
* Kiểm soát risk theo bot & account
* Báo cáo realtime

---

## 3. System Architecture

External Systems → API Gateway → Core Engine → Broker (cTrader)
↑
Telegram

---

## 4. Core Modules

### 4.1 Account Manager

```rust
struct Account {
  id: u64,
  name: String,
  connected: bool,
  autotrade: bool,
  daily_target_profit: f64,
  daily_max_loss: f64
}
```

👉 NEW:

* Target PnL cấp độ account
* Khi đạt target → disable autotrade

---

### 4.2 Bot Manager

Không giới hạn số lượng bot (không còn 9 cố định)

```rust
struct Bot {
  id: String,
  name: String,
  enabled: bool,
  symbol: String,
  timeframe: String,
  daily_target_profit: f64,
  daily_max_loss: f64
}
```

---

### 4.3 Source Manager

Quản lý nguồn lệnh

```rust
enum Source {
  TELEGRAM,
  API,
  WEB,
  ZEROCLAW,
  MT5
}
```

---

### 4.4 Auth Manager

Quản lý API key cho từng hệ thống

```rust
struct ApiClient {
  name: String,
  api_key: String,
  allowed_sources: Vec<Source>
}
```

---

### 4.5 Order Model (Chuẩn hóa toàn hệ)

```json
{
  "request_id": "uuid",
  "source": "telegram",
  "account_scope": "all | single | list",
  "account_ids": [123],
  "bot_id": "gold_scalper",
  "action": "OPEN | CLOSE | MODIFY",
  "symbol": "XAUUSD",
  "side": "buy",
  "volume": 0.1,
  "sl": 2300,
  "tp": 2350
}
```

---

### 4.6 Action System (Chuẩn hóa)

```text
OPEN
CLOSE
CLOSE_ALL
MODIFY
ENABLE_BOT
DISABLE_BOT
ENABLE_AUTOTRADE
DISABLE_AUTOTRADE
```

---

### 4.7 Position Tracking

```rust
struct Position {
  order_id: String,
  account_id: u64,
  bot_id: String,
  source: String
}
```

---

## 5. Telegram System

### 5.1 Config

```env
TELEGRAM_BOT_TOKEN=xxxxx
TELEGRAM_ADMIN_IDS=975318323
TELEGRAM_NOTIFY_GROUP_ID=-100xxxx
```

---

### 5.2 Command System

#### Autotrade

* `/a` → bật autotrade
* `/d` → tắt autotrade

---

#### Bot Control (dynamic)

* `/on <bot>`
* `/off <bot>`

---

#### Close

* `/c` → đóng tất cả + tắt autotrade
* `/c <bot>` → đóng theo bot

---

#### Positions

* `/p` → positions
* `/o` → pending orders

---

#### Reports

* `/r` → account report
* `/rp` → report theo bot

---

#### Trade trực tiếp

```
/buy XAUUSD 0.1 gold_scalper sl=2300 tp=2350
/sell BTCUSD 0.01 trend_bot
```

---

### 5.3 Event Notification

Push về Telegram Group:

```text
[OPEN]
Account: 123
Bot: gold_scalper
Symbol: XAUUSD
Volume: 0.1
```

---

## 6. API System

### Auth

```http
Authorization: Bearer <API_KEY>
```

---

### Multi-account control

```json
"account_scope": "all"
```

---

## 7. Risk Engine

### Bot level

* daily profit target
* daily max loss

### Account level

* tổng pnl
* nếu vượt → disable toàn bộ

---

## 8. Data Storage

Tables:

* accounts
* bots
* positions
* orders
* api_clients
* requests

---

## 9. Logging

Log đầy đủ:

* request
* telegram command
* trade execution
* error

---

## 10. Setup Guide

### 10.1 Clone

```bash
git clone https://github.com/tradekiemcom/ZeroClaw-Android/ctrader
cd ctrader
```

---

### 10.2 Run

```bash
cargo run
```

---

### 10.3 Env config

```env
API_KEY=secret
TELEGRAM_BOT_TOKEN=xxx
```

---

## 11. One-line Install (future)

```bash
curl -sL ctrader.tradekiem.com | bash
```

---

## 12. Deployment

* Dev: Mac M1
* Prod: Android Termux

---

## 13. Principles

* Không dùng cBot
* Mọi lệnh phải có bot_id
* Idempotency bắt buộc
* Track full lifecycle

---

## 14. Summary

> iZtradeRush = Trading Brain + Execution Engine + Control System + API Gateway
