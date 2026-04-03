# 🐾 ZeroClaw-Android: Cẩm Nang Sử Dụng Toàn Tập

Chào mừng bạn đến với **ZeroClaw-Android** - dự án biến một chiếc điện thoại Android cũ hoặc dư thừa thành một **Trạm điều khiển AI (AI Agent Hub)** hoạt động liên tục 24/7.

---

## 🌟 1. Giới Thiệu: ZeroClaw-Android Có Thể Làm Gì?

### Tầm Nhìn Dự Án
ZeroClaw-Android đưa sức mạnh của một AI Agent (chạy lõi ZeroClaw) lên môi trường Termux của thiết bị di động. Nó không chỉ là một chatbot, mà là một **"nhân viên ảo" có khả năng thực thi hành động**, quản lý tệp tin và duy trì tự động hóa một cách bền bỉ, tiết kiệm điện năng.

### Khả Năng Cốt Lõi (Dựa trên hệ sinh thái ZeroClaw)
- **Tự động hóa & Kịch bản (Shell/Scripting)**: AI có toàn quyền (trong không gian Termux) để đọc/ghi tệp, phân tích log, chạy script (Python, Node.js, Bash). Đóng vai trò như một quản trị viên hệ thống thu nhỏ.
- **Tương tác đa kênh**: Giao tiếp và nhận lệnh từ bạn thông qua **Telegram**, **Discord** hoặc trực tiếp qua **Terminal/SSH**.
- **Web Research (Nghiên cứu Web)**: Khả năng tìm kiếm, trích xuất dữ liệu từ các trang web để tổng hợp báo cáo.
- **Lên lịch tác vụ (Cron)**: Quản lý tiến độ và thực thi các công việc lặp đi lặp lại một cách tự động (ví dụ: quét tin tức mỗi sáng, thu thập dữ liệu giá coin).
- **Hỗ trợ lập trình**: Sinh code, sửa lỗi, và thậm chí chạy thử nghiệm code trực tiếp trong môi trường của nó.
- **Kiểm soát phần cứng di động**: Thông qua Termux:API, AI (nếu được cấp quyền và cung cấp skill) có thể đọc SMS, kiểm tra pin, lấy vị trí GPS, nhưng **không** trực tiếp thao tác (bấm/vuốt) trên màn hình ứng dụng Android khác.

### Khả Năng Mở Rộng
- **Thêm kỹ năng (Skills)**: Bạn có thể cài thêm các Skill (dạng file Markdown hướng dẫn kết hợp Tool) để dạy AI làm kế toán, theo dõi chứng khoán, hay quản lý file.
- **Tích hợp API bất kỳ**: Do nằm trong môi trường Linux, bạn dễ dàng cung cấp cho AI quyền gọi bất kỳ API nào (Notion, Google Sheets, Github...).

---

## 🎒 2. Khâu Chuẩn Bị Tối Quan Trọng

Trước khi bắt đầu, bạn cần chuẩn bị đầy đủ phần cứng, phần mềm và các tài nguyên sau:

### 2.1. Yêu cầu thiết bị Android
*Dù dự án được tối ưu rất tốt cho Samsung Galaxy Note 10+, bạn hoàn toàn có thể chạy trên các thiết bị khác đáp ứng yêu cầu:*
- **Kiến trúc CPU**: Android 64-bit (ARM64 / aarch64).
- **Hệ điều hành**: Android 8.0 trở lên (Android 12+ nếu dùng dòng Samsung cần chú ý phần Shield).
- **RAM**: Tối thiểu 4GB (Khuyến nghị 6GB-8GB để model xử lý mượt mà tác vụ phức tạp).
- **Bộ nhớ trống**: Ít nhất 2GB.

### 2.2. Phần mềm bắt buộc (Tải từ F-Droid, KHÔNG dùng CH Play)
Các phiên bản Termux trên CH Play đã bị bỏ hoang và sẽ gây lỗi.
1. Tải và cài đặt chợ ứng dụng mã nguồn mở **[F-Droid](https://f-droid.org/)**.
2. Mở F-Droid, tìm và cài đặt **Termux** (phiên bản mới nhất).
3. Mở F-Droid, tìm và cài đặt thêm **Termux:Boot** (Dùng để khởi động dịch vụ tự động khi bật máy).
4. Mở F-Droid, tìm và cài đặt **Termux:API** (Giúp AI giao tiếp với phần cứng điện thoại).

### 2.3. Các tài nguyên trực tuyến (API Keys & Tokens)
Bạn cần đăng ký và lấy sẵn các mã sau lưu vào Ghi chú:
1. **API Key Trí Tuệ Nhân Tạo**: (Bắt buộc) ZeroClaw cần "bộ não". Bạn có thể lấy API Key từ **Gemini**, **OpenAI**, **Anthropic**, hoặc **OpenRouter** (khuyến nghị OpenRouter vì có nhiều model rẻ/miễn phí).
2. **Cloudflare Tunnel Token**: (Bắt buộc) Để truy cập Dashboard từ xa mà không cần mở port mạng.
   - Đăng nhập [Cloudflare Zero Trust](https://one.dash.cloudflare.com/).
   - Vào **Networks > Tunnels** -> `Create a Tunnel`.
   - Đặt tên (VD: `zeroclaw-phone`). Setup môi trường Linux > ARM64.
   - Chép lại đoạn mã dài (Token) sau chữ `cloudflared service install`.
3. **Telegram Bot Token**: (Khuyến nghị) Dùng để chat với AI qua Telegram.
   - Mở Telegram, chat với `@BotFather`, tạo Bot mới và lấy `HTTP API Token`.

---

## 🛠 3. Các Bước Cài Đặt Chính Thức

Cầm điện thoại lên, mở ứng dụng **Termux** vừa cài đặt và thực hiện:

### Bước 1: Cấp quyền bộ nhớ
Gõ lệnh sau và nhấn "Cho phép" khi có bảng thông báo hiện lên:
```bash
termux-setup-storage
```

### Bước 2: Tải mã nguồn ZeroClaw-Android
Dán từng dòng lệnh sau vào Termux:
```bash
pkg update -y && pkg upgrade -y
pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
```

### Bước 3: Chạy trình cài đặt chính
```bash
chmod +x setup.sh && ./setup.sh
```
*Script sẽ tự chạy 1 lúc để tải Node.js hệt thống, thiết lập Cloudflare Tunnel và cài đặt các dịch vụ nội bộ.*

### Bước 4: Thiết lập "Khiên bảo vệ" (Process Shield) - RẤT QUAN TRỌNG
*Dành cho máy Android 12+ (Đặc biệt là máy Samsung hay bị tắt ứng dụng ngầm).*
1. Bật "Gỡ lỗi USB" (USB Debugging) trên điện thoại và cắm cáp kết nối vào máy tính (PC/Mac).
2. Tải mã nguồn này về máy tính, mở Terminal/CMD tại thư mục đó và chạy:
```bash
chmod +x shield/setup-shield.sh
./shield/setup-shield.sh
```
*(Nếu dùng Windows, bạn trích xuất các lệnh `adb shell...` trong file đó ra chạy thủ công).*
Thao tác này loại bỏ tính năng sát thủ "Phantom Process Killer" của Android, giúp AI của bạn sống sót chạy ngầm 24/7.

---

## ⚙️ 4. Thiết Lập Ban Đầu (Onboarding)

Sau khi cài xong, ZeroClaw Core cần được cấu hình "bộ não".

1. Trong Termux (vẫn ở thư mục ZeroClaw-Android), gõ lệnh:
   ```bash
   zeroclaw onboard
   ```
2. Cung cấp **Provider** (vd: `gemini` hoặc `openrouter`).
3. Dán **API Key** bạn đã chuẩn bị.
4. Chọn **Model** làm mặc định (vd: `gemini-1.5-pro` hoặc `gpt-4o`).

Để kết nối với **Telegram** (Giúp bạn chat với hệ thống từ xa mà không cần vào Termux):
```bash
zeroclaw channel bind-telegram <TELEGRAM_BOT_TOKEN_CUAT_BAN>
zeroclaw daemon # Lệnh này bắt đầu chạy ngầm hệ thống nghe lệnh từ Telegram
```

---

## 📡 5. Kết Nối & Giám Sát Qua Bảng Điều Khiển (Dashboard)

Dự án có đi kèm một **Web Dashboard** siêu nhẹ để bạn giám sát sức khoẻ thiết bị.

### Cách cấu hình Cloudflare Tunnel
1. Cấu hình file `.env`:
   ```bash
   nano tunnel/.env
   ```
   Thay `your-cloudflare-tunnel-token-here` bằng **Token** lấy ở Mục 2.3. Lưu lại (Ctrl+O, Enter, Ctrl+X).
2. Khởi động toàn bộ dịch vụ phụ trợ:
   ```bash
   ./zeroclaw.sh start
   ```

### Truy cập Giám sát
- **Tại nhà (Cùng WiFi)**: Mở trình duyệt web gõ `http://<IP_dien_thoai_thuộc_mang_LAN>:7643`
- **Từ xa (Internet)**: Vào Cloudflare Dashboard, gán một tên miền (Public Hostname) trỏ về `localhost:7643`. Nhập tên miền đó vào trình duyệt.

**Chức Năng Của Dashboard**: 
- Xem Uptime, RAM (để biết máy có bị tràn RAM hay không).
- Xem trạng thái tiến trình SSH, Tunnel có bị sập không.
- Bật/Tắt các dịch vụ này trực tiếp không cần động vào ĐT.
- Xem file log quá trình khởi động `boot.log`.

---

## 🚀 6. Hướng Dẫn Sử Dụng Và Thiết Lập Nhiệm Vụ

### Giao việc qua Terminal / SSH
Bạn có thể ra lệnh 1 lần (One-shot):
```bash
zeroclaw agent -m "Hãy phân tích log truy cập web ngày hôm qua và tạo file báo cáo.md"
```
Hoặc vào chế độ trò chuyện:
```bash
zeroclaw agent
```

### Quản lý Tiến độ (Cron Scheduler)
ZeroClaw hỗ trợ thiết lập lịch để giao việc định kỳ.
*Ví dụ: Yêu cầu AI lấy tin tức công nghệ mỗi 8h sáng:*
```bash
zeroclaw cron add "0 8 * * *" "Tìm kiếm 5 tin tức AI mới nhất trên mạng và lưu vào tóm-tắt.txt"
```
Kiểm tra danh sách tác vụ đang chạy định kỳ: `zeroclaw cron list`

## 🏁 Tổng Kết
Bạn đã hoàn thành việc biến chiếc điện thoại cũ thành một cỗ máy thông minh. Từ giờ, hãy liên lạc với Agent của bạn qua Telegram, xem trạng thái sức khoẻ qua Dashboard, và sử dụng SSH nếu cần cấu hình kỹ thuật sâu hơn. Chúc bạn làm chủ được AI của mình!
