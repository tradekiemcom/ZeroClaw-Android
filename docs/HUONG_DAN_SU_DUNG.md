# 🐾 ZeroClaw-Android: Cẩm Nang Vận Hành

Tài liệu này hướng dẫn cài đặt và thiết lập lõi tự động hóa **ZeroClaw-Android**. Phiên bản này được xây dựng độc quyền nhằm cài đặt `zeroclaw` thẳng lên môi trường Termux (Linux Bionic) của điện thoại Android, tự động triệt tiêu các lệnh Root/Sudo gây lỗi hệ thống.

---

## 🎒 1. Khâu Chuẩn Bị Tối Quan Trọng
- **Bắt buộc:** Thiết bị Android ARM64, cài Termux từ kho ứng dụng `F-Droid` (Không cài trên CH PLAY do chặn quyền).
- Tải mã nguồn cài đặt tự động cực nhanh:
  ```bash
  termux-setup-storage
  pkg install -y git
  git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
  cd ZeroClaw-Android
  chmod +x install.sh && ./install.sh
  ```

---

## ⚙️ 2. Trình Tự Khởi Động Chuẩn Xác (Bắt Buộc)

ZeroClaw được phân mảng thành nhiều luồng, để một hệ thống vừa có thể Chat Telegram, vừa tính toán ngầm, vừa mở Web Dashboard, bạn **phải** chạy theo đúng trình tự sau đây:

### Bước 1: Khai báo API Key (Chỉ làm 1 lần)
Ngay sau khi cài đặt xong, bạn gõ lệnh sau để mở giao diện làm quen. Hệ thống sẽ hỏi để bạn nhập API Key của Gemini hoặc OpenRouter:
```bash
zeroclaw onboard
```

### Bước 2: Tắt bảo mật để chat từ xa (Tuỳ chọn)
Để iZChat hoặc bất kỳ hệ thống ngoài nào gọi ZeroClaw mà không bị hỏi xác nhận `[y/N]`, bạn mở cấu hình:
```bash
nano ~/.config/zeroclaw/config.toml
```
Và thêm dòng này vào để tự động duyệt: `auto_approve = true`

### Bước 3: Dán Token Mạng Ra Trạm Cloudflare (Chỉ làm 1 lần)
Để Dashboard có thể kết nối với mạng internet bên ngoài (qua domain của bạn), hãy nối Tunnel:
```bash
zeroclaw tunnel bind <TOKEN_CUẢ_BẠN>
```

### Bước 4: Khởi chạy Nhân Nền (Daemon)
Đây là trái tim của hệ thống. Dù bạn có mở Dashboard hay không, quá trình tự động hoá, nhắc lịch (cron), thu thập dữ liệu ngầm và gửi tin nhắn về Telegram đều do Daemon phụ trách. Bạn nên chạy nó ẩn:
```bash
zeroclaw daemon &
```
*(Dấu `&` giúp tiến trình chạy chìm, trả lại dòng lệnh cho bạn thao tác tiếp).*

### Bước 5: Khởi chạy Giao Diện Web & Tunnel (Gateway)
Cuối cùng, để kích hoạt bảng điều khiển (Dashboard) và báo cho Cloudflare Tunnel mở cửa đón khách từ domain, bạn chạy lệnh Mở Cổng:
```bash
zeroclaw gateway
```
👉 *Done! Lúc này tiến trình Gateway sẽ thức dậy. Bạn truy cập qua Web (`boss.iz.life...`) là sẽ thấy Dashboard đồ họa xịn xò 100% không còn báo lỗi Connection Refused, đồng thời Telegram vẫn đang hoạt động thông suốt nhờ Daemon ở Bước 4.*

---

## 👔 5. Cách Gọi Đặc Vụ Nhanh (Không cần Dashboard)
Môi trường CLI của ZeroClaw vô cùng nhạy bén:

- Xin báo cáo nhanh bằng cờ `--model` linh hoạt (thay vì phụ thuộc hệ thống phụ):
  ```bash
  zeroclaw agent --model gemini-1.5-flash -m "Dịch đoạn văn này sang tiếng Pháp..."
  ```
- Treo máy ngầm đọc báo mỗi sáng (Cronjob):
  ```bash
  zeroclaw cron add "0 8 * * *" "Kiểm tra giá Vàng và gửi lên Telegram"
  ```
