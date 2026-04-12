# iZFx.Trade (cTrader Module)

Đây là REST API Gateway kết nối trực tiếp với **cTrader OpenAPI**, cho phép giao dịch đa tài khoản, quản lý Bot và cấp giá (Price Feed) tự động hóa.

## 🎯 Tính năng

- **Quản lý Vị thế (Positions):** Đóng từng lệnh, đóng một phần (Scale out), trượt dừng lỗ.
- **Lệnh chờ (Pending Orders):** Quản lý và hủy các lệnh chờ (Limit/Stop).
- **Quản lý Bot:** Tắt/Bật bot, hỗ trợ Telegram Bot 2 tầng (Context-aware).
- **Market Data Feed:** Cung cấp API cập nhật giá Real-time cho ZeroClaw Agent và các ứng dụng bên ngoài.
- **Telegram Notification:** Đẩy tín hiệu (Open/Close/Modify/Error) về Telegram.

## 🛠 Cấu trúc thư mục

```
ctrader/
├── src/
│   ├── main.rs         # Điểm vào, khởi chạy HTTP Server
│   ├── api/            # REST API Routes (controllers)
│   ├── ctrader/        # cTrader OpenAPI Client (Protobuf / TCP)
│   ├── engine/         # Xử lý Logic (Dispatcher, Execution, State)
│   ├── models/         # Các định nghĩa Model (Account, Bot, Position, PriceQuote)
│   ├── storage/        # Sqlite3 + thao tác DB
│   └── telegram/       # Bot Telegram (Teloxide) 2 lớp Menu
└── Cargo.toml          # Rust dependencies
```

## 📡 REST API Endpoints

### 1. Price Feed API (Public - Không yêu cầu Bearer token)
Hệ thống sử dụng cơ chế cấp giá thời gian thực cho các Bot/Agent ngoài.

| Method | Endpoint | Mô tả |
|--------|----------|-------|
| `GET`  | `/api/prices` | Lấy bảng giá của tất cả các symbol phổ biến |
| `GET`  | `/api/prices/{symbol}` | Lấy giá một mã cụ thể (Ví dụ: `XAUUSD`) |

### 2. Giao dịch & Quản lý (Protected - Yêu cầu Bearer token)

Gửi request với header: `Authorization: Bearer <API_KEY>`

| Method | Endpoint | Mô tả |
|--------|----------|-------|
| `GET`  | `/api/accounts` | Danh sách tài khoản cTrader được quản lý |
| `GET`  | `/api/bots`     | Danh sách Bot giao dịch tự động |
| `GET`  | `/api/positions`| Các vị thế đang mở và lệnh chờ |
| `GET`  | `/api/report`   | Báo cáo tổng thể PnL và hệ thống |
| `POST` | `/api/order`    | Thực hiện giao dịch (Mở/Đóng lệnh) |
| `POST` | `/api/prices/update` | Cập nhật giá thủ công qua webhook |
| `POST` | `/api/bots/{id}/enable` | Bật hoạt động Bot |
| `POST` | `/api/bots/{id}/disable`| Tắt Bot |

## ⚙️ Môi trường (.env)

Hệ thống yêu cầu thiết lập file `.env` trước khi khởi chạy:
```ini
# Server
PORT=8080
DATABASE_URL=sqlite://data.db

# cTrader Credentials
CTRADER_CLIENT_ID=your_id
CTRADER_SECRET=your_secret
CTRADER_ACCOUNT=1234567

# Telegram
TELEGRAM_BOT_TOKEN=your_telegram_bot_token
ADMIN_USER_ID=your_telegram_id
```

## 🚀 Khởi chạy

```bash
cargo build --release
cargo run --release
```
