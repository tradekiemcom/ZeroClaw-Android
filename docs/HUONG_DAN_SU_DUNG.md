# 🐾 ZeroClaw-Android: Cẩm Nang Vận Hành

Tài liệu này hướng dẫn cài đặt và thiết lập lõi tự động hóa **ZeroClaw-Android**. Phiên bản này được xây dựng độc quyền nhằm cài đặt `zeroclaw` thẳng lên môi trường Termux (Linux Bionic) của điện thoại Android, tự động triệt tiêu các lệnh Root/Sudo gây lỗi hệ thống.

---

## 🎒 1. Khâu Chuẩn Bị Tối Quan Trọng

### 1.1 Yêu cầu thiết bị
- Bất kỳ thiết bị Android nào (Từ v8.0 trở lên, kiến trúc ARM64).
- Cài đặt **Termux** từ kho ứng dụng bảo mật `F-Droid` (KHÔNG CÀI TRÊN CH PLAY).
- Để dùng được các dòng AI, bạn cần có một API Key (VD: OpenRouter, Gemini, OpenAI v.v.)

### 1.2 Môi trường cài đặt
Lõi thuật toán đã được đóng gọi lại để tự động cài các gói tĩnh `ca-certificates`, giải nén `tar` hệ thống native cho Android nên thời gian cài đặt sẽ vô cùng nhanh chóng.

---

## 🛠 2. Quá Trình Cài Đặt (Native Download)

**Bước 1:** Bật Termux và cấp quyền lưu trữ qua lệnh:
```bash
termux-setup-storage
```

**Bước 2:** Tải mã nguồn trình cài đặt
```bash
pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
```

**Bước 3:** Chạy cài đặt tự động (Thay thế hoàn hảo cho lệnh sudo lỗi trên bản gốc)
```bash
chmod +x install.sh
./install.sh
```

---

## ⚙️ 3. Thiết Lập Tư Duy (Onboarding)

Sau khi hệ thống gắn thành công `zeroclaw` vào Android, bạn thao tác cung cấp API Key như sau:
```bash
zeroclaw onboard
```
*Làm theo hướng dẫn trên màn hình điện thoại để khai báo.*

---

## 👔 4. Cách Giao Việc Cho Đặc Vụ ZeroClaw

Môi trường lý tưởng nhất của ZeroClaw là thực thi mã code và quét thông tin web thông qua CLI.
Bạn có thể giao phó nhiệm vụ:

### Giao việc một lần (One-shot)
```bash
zeroclaw agent -m "Vào trang web zingnews và lấy cho tôi 5 tiêu đề hot nhất hôm nay."
zeroclaw agent -m "Tạo và chạy một script python kiểm tra địa chỉ ip mạng hiện tại"
```

### Trò Chuyện (Interactive Chat)
```bash
zeroclaw agent
```

### Quản Lý Tiến Độ Bằng Lịch (Cron)
Ví dụ bạn muốn ZeroClaw chạy ngầm mỗi sáng phân tích tài chính:
```bash
zeroclaw cron add "0 8 * * *" "Lấy tỷ giá đô la báo cáo"
```

---

## 📡 5. Tích Hợp Cloudflare Tunnel & Telegram

- **Cloudflare Tunnel**: Bộ cài đặt đã đính kèm `cloudflared` gốc. Để kích hoạt mạng mở rộng ra internet:
  ```bash
  cloudflared service install [MÃ_TOKEN_CUẢ_BẠN]
  ```

- **Telegram**: Thay vì gõ lệnh nhọc nhằn, bạn có thể tạo Bot chat qua @BotFather và móc nối:
  ```bash
  zeroclaw channel bind-telegram <TOKEN>
  zeroclaw daemon
  ```

Hệ thống đã sẵn sàng làm việc liên tục!
