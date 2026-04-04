# 🐾 Cẩm Nang Vận Hành ZeroClaw-Android Cấp Tốc

Tài liệu này bao gồm toàn bộ hướng dẫn từ A-Z để vận hành, cấu hình, và xử lý triệt để các vấn đề thường gặp khi chạy **ZeroClaw trên điện thoại (Galaxy Note 10+)**.

---

## 🚀 1. Lệnh Chạy Cốt Lõi (Khởi động hệ thống)

Sau khi cài xong, ZeroClaw không tự động chạy ngầm. Bạn phải "mở cửa công ty" bằng cách chạy Daemon (Tiến trình chủ).

### 🌐 Mở truy cập Dashboard mạng LAN (Bản đồ UI)
Mặc định ZeroClaw chỉ trỏ vào `127.0.0.1` (Chỉ cái điện thoại mới vào được). Để dùng Laptop/PC trong cùng mạng Wifi truy cập vào điện thoại, bạn khởi động bằng lệnh:
```bash
zeroclaw daemon --host 0.0.0.0 --port 42617
```
👉 *Lúc này, bạn dùng Laptop gõ địa chỉ IP Wifi của điện thoại (Ví dụ: `http://192.168.1.5:42617`) để vào Dashboard đồ họa (Web UI).*

### ⚡ Bỏ qua việc xác nhận "Approve" trên Telegram
ZeroClaw trang bị tính năng bảo mật: Mọi thao tác động chạm đến hệ thống (Kiểm tra model, đọc file, chạy lệnh) qua Telegram đều bị AI chặn lại và bắt người chủ **phải gõ "Y" trên màn hình Termux** để duyệt.
Để tắt tính năng phiền toái này và cấp toàn quyền cho iZChat, bạn hãy thêm cờ tự động duyệt:
```bash
zeroclaw daemon --host 0.0.0.0 --auto-approve
```
*(Nếu phiên bản đang dùng lấy config từ tệp, bạn có thể thiết lập `auto_approve = true` trong file `~/.config/zeroclaw/config.toml`).*

---

## 📡 2. Kích Hoạt Cloudflare Tunnel (Truy Cập Từ Xa)

Tin vui là ZeroClaw đã tích hợp thẳng Cloudflared vào nhân của ứng dụng. Bạn không cần dùng lệnh ngoài.
**Bước 1:** Dán Token mà bạn đã tạo trên trang Cloudflare Zero Trust:
```bash
zeroclaw tunnel bind <TOKEN_CUẢ_BẠN>
```
**Bước 2:** Chạy tiến trình Daemon như hệ thống máy chủ:
```bash
zeroclaw daemon --host 0.0.0.0 --auto-approve
```
👉 *Sau khi khởi chạy, Tunnel sẽ tự động được đào. Bây giờ bạn có thể ở bất kỳ đâu trên thế giới, bật 4G và dùng Domain (VD: `boss.iz.life`) để truy cập vào Dashboard của ZeroClaw.*

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
