# 💼 ZeroClaw-Android: A.I Company Edition (Native Termux)

Dự án này là phiên bản tùy biến tự động hóa để cài đặt trực tiếp **[zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)** xuống môi trường Android (Termux) mà **không cần Root**. Đồng thời quy hoạch hệ thống dưới dạng một **Công ty A.I (Multi-Agent)**.

---

## 🏢 1. Mô Hình "Công Ty A.I" Là Gì?

Thay vì sử dụng một AI duy nhất, dự án này cung cấp lệnh `company-mgr` để khởi tạo các **Đặc Vụ (Agents)** đóng vai riêng biệt, giúp tăng tính chuyên môn khi thực thi nhiệm vụ:

- **CEO**: Đặc vụ chỉ huy, phân tích vấn đề lớn và ra quyết định.
- **Assistant**: Trợ lý cá nhân, tóm tắt thông tin và nhắc việc.
- **R&D**: (Nghiên cứu & Phát triển) Viết code, debug, thao tác Terminal, chạy script.
- **Marketing**: Sáng tạo nội dung, tổng hợp tin tức web.
- **Trading/Financial**: Cập nhật xu hướng coin/forex, phân tích tin tức tài chính.

*Lưu ý: ZeroClaw không có Dashboard Web đồ họa mặc định. Đây là một công cụ sức mạnh dành riêng cho Terminal (CLI).*

---

## 🎒 2. Khâu Chuẩn Bị Tối Quan Trọng

### 2.1 Yêu Cầu Thiết Bị
- **Android**: Bất kỳ thiết bị Android nào (Từ v8.0 trở lên, kiến trúc ARM64).
- **RAM**: Tối thiểu 4GB. (Từ 6GB trở lên nếu muốn chạy mô hình nội bộ TinyLLM trên máy tính toán offline).
- **ROM**: Trống ít nhất 1-2GB.

### 2.2 Phần Mềm (Tải từ F-Droid, KHÔNG dùng CH Play)
Tuyệt đối không dùng bản trên CH Play vì chúng đã ngưng hỗ trợ. Phải cài từ [F-Droid](https://f-droid.org/):
1. Cài app **Termux**.
2. Cài app **Termux:API** (Để AI tương tác được với phần cứng/mạng).

### 2.3 Cấu Hình API Keys
Vì ZeroClaw cần "Lực lượng lao động" (Các LLM), bạn cần API Key của:
- **Gemini / OpenAI / OpenRouter** (Hỗ trợ hầu hết mọi model hiện nay).

---

## 🛠 3. Các Bước Cài Đặt Chính Thức

### Bước 1: Mở ứng dụng Termux và cấp quyền bộ nhớ:
```bash
termux-setup-storage
```

### Bước 2: Tải trình cài đặt
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
```

### Bước 3: Chạy trình thiết lập Công ty A.I
```bash
chmod +x install.sh
./install.sh
```
*Script sẽ kiểm tra phần cứng RAM, tự động tải bản ZeroClaw aarch64-linux-android gốc về, và cấu hình các phòng ban chức năng.*

---

## ⚙️ 4. Thiết Lập Tư Duy (Onboarding)

Sau khi cài xong, bạn cần móc API Key vào cho các nhân viên A.I của mình hoạt động. Gõ:
```bash
zeroclaw onboard
```
Làm theo màn hình để chọn Nhà cung cấp (Vd: gemini) và dán API Key.

*(Tính Năng Mới)*: Nếu thiết bị bạn **có RAM > 4GB** và bạn **không có API Key**, ZeroClaw có thể tự động dự phòng và tải một mô hình **TinyLLM (LlamaCPP)** từ bộ xử lý nội bộ. *(Lưu ý: xử lý bằng chip điện thoại sẽ khá chậm)*.

---

## 👔 5. Hướng Dẫn Điều Hành Công Ty A.I

Bây giờ bạn là Chủ tịch (Founder). Hãy mở Termux lên và gọi các nhân sự bằng lệnh CLI mới: `company-mgr`

### Giao việc một lần (One-shot)
```bash
company-mgr marketing -m "Viết ngay cho tôi một kịch bản tiktok về AI"
company-mgr rd -m "Tạo và chạy một script python kiểm tra ip mạng hiện tại"
```

### Triệu tập họp (Interactive Chat)
Bạn muốn chat trực tiếp dài hạn với Trưởng phòng Trading?
```bash
company-mgr trading
```
*(Hệ thống sẽ mở giao diện chat. Bạn cứ gõ tiếng Việt tự nhiên hỏi xu hướng giá Bitcoin)*.

### Làm sao để quản lý giám sát tiến độ?
ZeroClaw có trình lên lịch (Cron) cực kỳ xịn để AI tự làm việc mỗi ngày.
```bash
zeroclaw cron add "0 8 * * *" "company-mgr trading -m 'Phân tích tin tức báo chí hôm nay và gửi báo cáo'"
```

---

## 📱 6. Kết Nối Telegram Để Điều Khiển Từ Xa
Thao tác trên màn hình nhỏ rất mệt. Bạn có thể kết nối ZeroClaw với Telegram để chat với "Nhân viên" qua tin nhắn:
1. Tạo Bot Telegram qua @BotFather và lấy Token.
2. Ép kênh Telegram:
   ```bash
   zeroclaw channel bind-telegram <TOKEN_CUA_BAN>
   ```
3. Chạy Daemon nền:
   ```bash
   zeroclaw daemon
   ```
(Bạn sẽ chat được với Agent ngay trên Telegram. Bạn có thể dùng `Termux:Boot` để tự động chạy lệnh daemon này mỗi khi khởi động điện thoại lại).
