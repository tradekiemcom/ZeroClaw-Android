# 🐾 Cẩm Nang Vận Hành ZeroClaw-Android Cấp Tốc

Tài liệu này bao gồm toàn bộ hướng dẫn từ A-Z để vận hành, cấu hình, và xử lý triệt để các vấn đề thường gặp khi chạy **ZeroClaw trên điện thoại (Galaxy Note 10+)**.

---

## 🚀 1. Lệnh Chạy Cốt Lõi (Khởi động máy chủ Server)

Để kết nối Tunnel và tắt bảo mật, ZeroClaw dùng tệp cấu hình thay vì truyền cờ dòng lệnh dông dài. Bạn tiến hành mở file cấu hình gốc ra để sửa:

```bash
nano ~/.config/zeroclaw/config.toml
```

Trong file đó, bạn chỉnh sửa hoặc bổ sung các dòng sau để cấp quyền mở cổng mạng LAN và tự động duyệt lệnh:
```toml
# Chấp nhận tắt xác minh thủ công qua Telegram
auto_approve = true

# Mở cổng cho mạng LAN và Tunnel
[server]
host = "0.0.0.0"
port = 42617
```
*(Ấn `Ctrl + X`, bấm `Y` và `Enter` để lưu lại trên Termux).*

Sau khi lưu cấu hình, bạn khởi động máy chủ nền (Gateway) của ZeroClaw bằng một trong các lệnh chạy liên tục (Hãy thử xem lệnh nào máy bạn hỗ trợ):
```bash
zeroclaw serve
```
*(Hoặc `zeroclaw daemon` / `zeroclaw server` tuỳ phiên bản).*

---

## 📡 2. Kích Hoạt Cloudflare Tunnel (Truy Cập Dashboard)

Lỗi `connection refused` từ Cloudflared là do hệ thống máy chủ ZeroClaw chưa được bật lên (hoặc bị tắt ngầm). Bạn phải chắc chắn tiến trình `zeroclaw serve` đang chạy liên tục không bị văng.

**Bước 1:** Dán Token Tunnel:
```bash
zeroclaw tunnel bind <TOKEN_CUẢ_BẠN>
```
**Bước 2:** Bật Server và treo máy:
```bash
zeroclaw serve
```
👉 *Lúc này Cloudflared sẽ có đích đến là cổng 42617. Nó sẽ đưa Dashboard Web UI lên domain `note.iz.life` của anh!*

---

## 👔 3. Quản Lý Mô Hình (Models) và Đặc Vụ (Agents)

Vì ZeroClaw lấy nền tảng iZ.Life làm gốc, mọi thứ đều cấu hình dễ dàng bằng giao diện đồ họa Web UI. Nhưng để cài thủ công qua dòng lệnh Termux, hãy làm như sau:

### ⚙️ Thêm và cấu hình Model
Sử dụng lệnh `config` để khai báo model mặc định cho toàn hệ thống:
```bash
zeroclaw config set default_model "openrouter/google/gemini-2.0-flash"
zeroclaw config set provider.openrouter.api_key "sk-or-v1-xxxxxxxx"
```
Kiểm tra danh sách các model đang hoạt động:
```bash
zeroclaw models list
```

### 👤 Tạo Đặc Vụ Mới (Agent)
ZeroClaw định nghĩa các Agent bằng file `.toml`. Để tạo một đặc vụ mới có tên là **Marketing**:
1. Tạo một thư mục chứa (vd: `~/agents`).
2. Tạo file `marketing.toml`:
```toml
[agent]
name = "CMO Agent"
description = "Giám đốc Marketing"
system_prompt = "Bạn là GĐ Marketing. Mục tiêu của bạn là phân tích thị trường và viết nội dung siêu chuyển đổi."
model = "openrouter/google/gemini-1.5-pro"
```
3. Gọi đặc vụ đó trực tiếp:
```bash
zeroclaw agent --config ~/agents/marketing.toml
```

---

## 📌 4. Sét Đặt Chạy Ngầm Bằng Termux:Boot (Nâng Cao)
Nếu muốn điện thoại khởi động lại cũng tự bật hệ thống luôn:
1. Cài app **Termux:Boot** từ F-Droid.
2. Trong Termux, gõ:
```bash
mkdir -p ~/.termux/boot/
echo 'zeroclaw daemon --host 0.0.0.0 --auto-approve &' > ~/.termux/boot/start_zeroclaw.sh
chmod +x ~/.termux/boot/start_zeroclaw.sh
```
Khởi động lại máy, Note 10+ của bạn sẽ chính thức thành một Server AI chạy bằng cơm, luôn luôn bật 24/7!
