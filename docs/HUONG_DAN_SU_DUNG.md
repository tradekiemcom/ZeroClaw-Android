# 🐾 ZeroClaw-Android: Tài Liệu Hướng Dẫn Kỹ Thuật Chi Tiết

Dự án **ZeroClaw-Android** cung cấp một giải pháp tự động hóa agent AI chạy hoàn toàn trên điện thoại (đặc biệt tối ưu cho Samsung Galaxy Note 10+) thông qua môi trường Termux.

---

## 📖 1. Giới Thiệu Chung

### ZeroClaw-Android là gì?
Đây là một "Hub" điều khiển AI Agent được thiết kế để biến chiếc điện thoại Android cũ của bạn thành một máy chủ AI hoạt động 24/7. Nó kết hợp sức mạnh của:
- **ZeroClaw Core**: Bộ não AI có khả năng xử lý tác vụ, quản lý tệp tin và thực thi lệnh.
- **Tính di động**: Hoạt động bền bỉ trên Android mà không cần máy tính.
- **Khả năng mở rộng**: Bạn có thể dạy cho nó các "Skills" (kỹ năng) mới để làm việc thay bạn.

### Tại sao nên dùng?
- Tận dụng phần cứng điện thoại cũ làm Server AI giá rẻ.
- An toàn & Riêng tư: Code và dữ liệu nằm trên thiết bị của bạn.
- Luôn kết nối: Truy cập từ xa mọi lúc mọi nơi qua Cloudflare Tunnel.

---

## 🛠 2. Khâu Chuẩn Bị (Trước khi cài đặt)

Trước khi bắt đầu chạy script, bạn cần chuẩn bị đầy đủ các yếu tố sau:

### 📱 Yêu cầu phần cứng
- **Thiết bị**: Tối ưu nhất cho **Samsung Galaxy Note 10+** (Android 12).
- **Cấu hình tối thiểu**:
    - CPU: ARM64 (64-bit).
    - RAM: Tối thiểu 4GB (Khuyến nghị 8GB+ để chạy mượt).
    - SSD/Bộ nhớ trong: Còn trống ít nhất 2GB.
- **Phần mềm**: Cài đặt sẵn ứng dụng **Termux** và **Termux:Boot** từ F-Droid.

### ☁️ Lấy mã Cloudflare Tunnel Token
Dịch vụ này giúp bạn truy cập bảng điều khiển từ xa mà không cần mở port modem.
1. Truy cập **[Cloudflare Zero Trust Dashboard](https://one.dash.cloudflare.com/)**.
2. Vào mục **Networks > Tunnels**.
3. Chọn **Create a Tunnel** (Loại: Cloudflared).
4. Đặt tên (VD: `zeroclaw-android`).
5. Trong phần "Install connector", chọn "Linux" và kiến trúc "ARM64".
6. Copy đoạn mã sau chữ `service install`. Chuỗi ký tự dài bắt đầu bằng `ey...` chính là **Token** bạn cần.

---

## 🚀 3. Các Bước Cài Đặt

### Bước 1: Cài đặt trên điện thoại (Termux)
Mở Termux và dán các lệnh sau:
```bash
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
chmod +x setup.sh && ./setup.sh
```
Script sẽ tự động cài đặt Node.js, Cloudflared và các thư viện cần thiết.

### Bước 2: Kích hoạt lớp bảo vệ (Shield)
Đây là bước cực kỳ quan trọng đối với dòng Samsung (Android 12+).
1. Kết nối điện thoại với máy tính qua cáp USB.
2. Trên máy tính, chạy script bảo vệ:
```bash
chmod +x shield/setup-shield.sh && ./shield/setup-shield.sh
```
*Lưu ý: Lệnh này sẽ tắt tính năng "Phantom Process Killer" của Android để tránh việc hệ thống tự động tắt AI của bạn.*

---

## 🖥 4. Công Năng & Cách Sử Dụng

### 1. Bảng điều khiển Admin (Dashboard)
Truy cập qua trình duyệt tại: `http://localhost:7643`
- **Công dụng**: Theo dõi nhiệt độ chip, dung lượng RAM, trạng thái Pin và Bật/Tắt các dịch vụ (SSH, Tunnel) chỉ bằng 1 cú chạm.
- **Tại sao cần?**: Giúp bạn quản lý server dễ dàng ngay trên màn hình cảm ứng nhỏ mà không cần gõ lệnh CLI phức tạp.

### 2. Giao diện dòng lệnh (ZeroClaw CLI)
Đây là nơi bạn tương tác trực tiếp với AI. Trong Termux, hãy dùng:
- `zeroclaw agent`: Bắt đầu trò chuyện với AI.
- `zeroclaw onboard`: Thiết lập API Key của các nhà cung cấp (OpenAI, Anthropic, etc.).
- `zeroclaw status`: Kiểm tra tình trạng sức khỏe hệ thống.

---

## 🧩 5. Khả Năng Mở Rộng & Kỹ Năng (Skills)

### Kỹ năng hỗ trợ sẵn:
- **Quản lý file**: AI có thể đọc, viết và sửa lỗi code trực tiếp trong Termux.
- **Web Research**: Tìm kiếm thông tin trên internet.
- **Tự động hóa**: Lập lịch chạy các tác vụ định kỳ qua lệnh `cron`.

### Tự phát triển kỹ năng mới:
Bạn hoàn toàn có thể tự viết thêm kỹ năng cho ZeroClaw bằng ngôn ngữ Markdown hoặc các script hỗ trợ. AI của bạn có thể học cách điều khiển các ứng dụng khác qua bộ API của Termux.

---

## ⚠️ Lưu Ý Quan Trọng
- **Nhiệt độ**: Hãy đảm bảo điện thoại đặt ở nơi thoáng mát khi chạy các tác vụ AI nặng.
- **Bảo mật**: Tuyệt đối không chia sẻ file `~/.zeroclaw/config.toml` vì nó chứa mã khóa API của bạn.

---
*Tài liệu được biên soạn bởi Antigravity dành cho dự án ZeroClaw-Android.*
