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
Đây là trái tim của hệ thống. Quá trình tự động hoá, nhắc lịch (cron), thu thập dữ liệu ngầm và gửi tin nhắn về Telegram đều do Daemon phụ trách. 
Bạn gõ lệnh sau để chạy:
```bash
zeroclaw daemon
```
👉 *Lưu ý: Sau khi lệnh này chạy, hệ thống sẽ liên tục đổ log (nhật ký) ra màn hình. **Bạn tuyệt đối không bấm Ctrl+C để thoát**.*

### Bước 5: Khởi chạy Giao Diện Web & Tunnel (Gateway) trên Session Kế Tiếp
Để kích hoạt trạm điều khiển (Dashboard) và báo cho Cloudflare Tunnel mở cửa đón khách từ domain đồng thời không làm gián đoạn Daemon ở Bước 4, bạn tiến hành như sau:

1. Vuốt từ cạnh trái của màn hình Termux sang phải.
2. Chọn **"New Session"** (Tạo phiên cửa sổ Terminal mới).
3. Tại cửa sổ Terminal số 2 này, bạn gõ lệnh Mở Cổng:
```bash
zeroclaw gateway
```

👉 *Tiến trình Gateway sẽ thức dậy và liên tục chiếm dụng cửa sổ số 2 này. Bây giờ hệ thống đã hoàn toàn sẵn sàng, bạn có thể trở về màn hình chính điện thoại, mở trình duyệt lên gõ đường link Domain (`boss.iz.life...`) là sẽ thấy Dashboard đồ họa sắc nét bung ra, trong khi Telegram cũng được trả lời tức thì bởi Daemon (đang chạy ở màn số 1).*

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
