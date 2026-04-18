# iZFx.Trade (ZeroClaw Hub) - Đánh Giá Kỹ Thuật & Lộ Trình Tiến Hóa (Audit Report)

*Ngày lập: 12/04/2026 (Cập nhật 13/04/2026)*

> [!IMPORTANT]
> **Tài liệu Kỹ thuật Chi tiết (Full Technical Spec):**
> Xem hướng dẫn tổng thể về kiến trúc, module và state tại: [SYSTEM_GUIDE.md](./SYSTEM_GUIDE.md)

## 1. Đánh Giá Cấu Trúc Hiện Tại (Audit)

### 📊 Điểm Sáng
-   **Kiến trúc Đa Nguồn (Multi-Source)**: Hệ thống đã tách biệt rõ ràng các nguồn nhận lệnh (Telegram, HTTP API, và Command Line Interface).
-   **Quản Lý State (AppState)**: Sử dụng `Arc<AppState>` kết hợp `RwLock` cho phép truy cập an toàn từ nhiều luồng. Đã tích hợp hệ thống báo cáo (Reporting) và bộ nhớ in-memory có đồng bộ tĩnh cho Account/API Key.
-   **Tầng Storage Hoàn Thiện**: Các lỗi liên quan đến trình mượn biến trong Rust (Borrow Checker) và Data Mapping (kiểu `Option<String>` so với kiểu `String`) trong `sqlx` đã được xử lý triệt để tại `storage/db.rs`. Toàn bộ chức năng thao tác Database đều ổn định.
-   **CLI v2.1 Hiện Đại (Context-Aware)**: Đã thiết kế thành công Console thao tác CLI chia làm 2 tầng (Global Scope và Account Scope). Toàn bộ sức mạnh và độ chi tiết của trình phiên dịch cú pháp (IzParser) giờ đây đã áp dụng mượt mà từ Telegram sang CLI.

### ⚠️ Các Vấn Đề Cần Giải Quyết 
-   **Hợp Nhất Socket Mạng (Multiplexing)**: Hiện tại app vẫn làm hình nộm (Mock Mode) hoặc 1-core Account connection. Nó cần một Pool kết nối có khả năng chứa và duy trì PING (heartbeat) cho hàng trăm phiên TCP (TLS) độc lập cho các account khác nhau (Multi-Account Multi-Connect).
-   **Cơ Chế Live Feed Cục Bộ**: Chuyển đổi từ `mock_prices` sang luồng tin (stream) thực tế dựa trên tin nhắn `ProtoOASpotEvent`.

---

## 2. Phân Tích Hiện Trạng Logic

| Thành phần | Cơ chế hiện tại | Đánh giá | Rủi ro / Mức độ sẵn sàng |
| :--- | :--- | :--- | :--- |
| **CLI Console** | `State Machine` đa tầng nội tại | Rất nhanh, tinh gọn lệnh thao tác khẩn cấp | **Sẵn sàng** |
| **Telegram Bot** | `teloxide` webhook / polling | Cấu trúc phím Inline rành mạch | **Sẵn sàng** |
| **Storage Layer**| `sqlx::SqlitePool` Async Queries| Các API tương tác với SQLite chuẩn xác | **Sẵn sàng** |
| **Memory State** | Khóa `RwLock<HashMap>` | Thread-safe, hỗ trợ thao tác P&L song song | **Sẵn sàng** |
| **Parser (Lưới Lệnh)**| `IzParser` với Regex / Tách chuỗi | Đọc thông số lưới Order phức tạp mạnh mẽ | **Sẵn sàng** |
| **Order Gateway / Mạng**| Mock / Đơn luồng | Phải xây dựng tầng ConnectionManager | **Chờ Tiến Hành (Next Step)** |

---

## 3. Kế Hoạch Nâng Cấp Nền Tảng (Multi-Connection / Multi-Account)

Mục Tiêu Trọng Điểm: Xây Cấu Trúc **"Parallel WebSocket Connection Pool"** thay thế hệ thống giả lập nhằm kích hoạt live-trading cho n Tài khoản.

### Giai đoạn 1: Refactor cTrader Core (Cấu trúc Pool Mạng)
-   **Xây dựng `ConnectionManager`**: Triển khai 1 struct kiểm soát một bộ danh sách (HashMap) chứa các TCP Stream kết nối thực của cTrader.
-   **Kiến trúc `AccountSession`**:
    -   Mỗi Account có 1 TLS TCP Stream riêng.
    -   1 luồng (Task) chuyên chịu trách nhiệm gửi PING (Heartbeat).
    -   1 luồng (Task) chuyên chờ nhận thông điệp (Listener) giải mã `ProtoOASpotEvent` / `ExecutionEvent` đẩy kết quả vô `AppState`.
-   **Auto-Authentication**: Bổ sung hàm tự động (ví dụ `boot_sessions`). App bật lên sẽ duyệt list các UserAccount và gửi yêu cầu xác thực ứng dụng (`AppAuth`) & tài khoản (`AccountAuth`) tự động.

### Giai đoạn 2: Khớp Nối Bộ Thực Thi Cốt Lõi (Execution Engine)
-   **Dispatch & Route**: Lệnh từ CLI hay Telegram (vd: Lưới Mua) khi lọt qua `IzParser` thay vì lặp giả lập, bộ định tuyến (Router) sẽ bưng thông điệp đó thả thẳng vào `AccountSession` tương ứng của Account thông qua Channel.
-   **Báo cáo Lãi/Lỗ Thực Tế (Live Risk/P&L)**: Viết thêm Task nghe luồng Event thực tế cập nhật giá trị vốn thực và margin của mọi account 24/24 vào `AppState`.

---

## 4. Góp Ý Kỹ Thuật Bám Sát

> [!TIP]
> **Quản lý Hàng Trăm Sockets Với Năng Lượng Rất Thấp:**
> Rust sẽ tỏa sáng ở điểm này: Việc giữ 500 TCP Stream TLS bằng các tác vụ Tokio (Green Threads) tiêu thụ RAM cực kì thấp. Nó không tốn chi phí ThreadContext Switching của Hệ điều hành. Note 10+ với 8GB hoặc M1 Mac chạy nhẹ tựa lông hồng.

> [!IMPORTANT]
> **Nhiệm vụ trực diện phải làm tiếp theo:**
> 1. Viết `ConnectionManager` và tích hợp proto cTrader vào mã nguồn.
> 2. Đổi logic thao tác đánh lệnh của `core.rs` (Order Engine) để nó gửi Bytes lên cổng Mạng thay vì mock log.
