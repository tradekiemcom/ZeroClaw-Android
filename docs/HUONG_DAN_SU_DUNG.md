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

## ⚙️ 2. Onboarding (Thiết lập ban đầu)
Ngay sau khi cài đặt thành công thuật toán lõi, hệ thống cần được định danh và khai báo API Key. Bạn gõ lệnh sau để mở giao diện làm quen:
```bash
zeroclaw onboard
```
*(Hãy làm theo hướng dẫn hỏi-đáp trên trạm kiểm soát để dán API Key của Gemini hoặc OpenRouter vào).*

---

## 🔗 3. Cấu Hình Tắt Chặn Bảo Mật Telegram (Auto-Approve)
Để hệ thống không bị khựng lại hỏi xác nhận `[y/N]` mỗi khi chat quản lý qua mạng (vd như iZChat v.v..), bạn hãy mở file cấu hình gốc để tắt chặn:

1. Gõ lệnh sửa file cấu hình (nếu chưa có, bạn thử gõ lệnh `zeroclaw config` để hệ thống tự sinh file):
```bash
nano ~/.config/zeroclaw/config.toml
```

2. Bổ sung hoặc đổi thành `true`:
```toml
auto_approve = true
```

---

## 💻 4. Truy Cập Giao Diện Web Dashboard

Bản chất ZeroClaw là chạy theo mô hình ngầm, nếu muốn mở giao diện điều khiển (Web UI / Dashboard) trực quan, bạn dùng trình quản lý **Gateway**:

**Bước 1: Kết nối Tunnel nội bộ**
Bộ cài ZeroClaw đã tích hợp Cloudflared bản quyền gốc. Chỉ việc dán Token của Cloudflare bằng lệnh sau (Chỉ dán 1 lần duy nhất):
```bash
zeroclaw tunnel bind <TOKEN_CUẢ_BẠN>
```

**Bước 2: Khởi động Động cơ Gateway vĩnh viễn**
Thay vì gọi `daemon` hay `serve`, lệnh chuẩn xác cuối cùng để đánh thức đồng thời cả **Dashboard** và mở cổng **Cloudflare Tunnel** kết nối với internet là:
```bash
zeroclaw gateway
```
👉 *Done! Lúc này, chỉ cần mở trình duyệt lên gõ đường link Domain (vd: `boss.iz.life`), bạn sẽ thấy Web Dashboard được bung ra sắc nét, không còn báo lỗi Connection Refused hay Time-out nữa.*

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
