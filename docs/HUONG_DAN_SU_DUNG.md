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

👉 *Tiến trình Gateway sẽ thức dậy và liên tục chiếm dụng cửa sổ số 2 này.* 

### Bước 6: Đăng nhập Dashboard Web UI
Bây giờ hệ thống đã hoàn toàn sẵn sàng, bạn có thể trở về màn hình chính điện thoại và mở trình duyệt lên:
1. Gõ **đường link Domain** mà bạn đã cấu hình bên trong hệ thống Cloudflare Zero Trust (Ví dụ do bạn tự đặt: `admin.ten-mien-cua-ban.com`).
2. Giao diện ZeroClaw Dashboard sẽ hiện lên yêu cầu mật khẩu/Mã xác thực.
3. Bạn quay lại Termux (Ở cái Session cửa sổ số 2 đang chạy lệnh `zeroclaw gateway`), nhìn kỹ trên màn hình log sẽ thấy một hàng chữ cung cấp **Mã Bảo Mật (Auth Code / Password)**.
4. Copy mã đó dán ngược lại vào trình duyệt là bạn sẽ chính thức làm chủ được giao diện điều khiển!

*(Trong lúc đó, Telegram cũng vẫn đang được trả lời tức thì bởi tiến trình Daemon chạy ở màn Session số 1).*

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

---

## 🛰 6. Quản Lý Điều Khiển Tập Trung (OTA Sync)
Tính năng OTA giúp "Sếp" kiểm soát các thiết bị Note 10+ từ xa mà không sợ lộ Key nội bộ, cơ chế tải và giải mã theo từng thiết bị như sau:

1. **Khi chạy Cài Đặt (install.sh)**: ZeroClaw tự động thiết lập module thứ 6, tích hợp luôn `android-tools` (ADB) và `termux-services` để biến cái điện thoại thành cỗ xe Zombie chờ lệnh Sếp.
2. **Khởi động**: Tệp `~/.termux/boot/start_ota.sh` đảm bảo mỗi lần reset máy, quá trình đồng bộ OTA luôn tự kích hoạt đầu tiên.
3. **Mã khoá nội bộ**: Khóa để mã hóa cấu hình do chính tay anh cấu hình trong một Cloudflare Worker có tên là `ota.tradekiem.com`.
4. **Vận hành**: Bất cứ khi nào anh muốn đẩy cấu hình hoặc chạy lệnh điều khiển ADB từ xa, anh chỉ cần sửa cấu hình ở Worker Cloudflare của anh (trong thư mục `ota-server/`). Thiết bị tải về file mã hóa và sẽ **tự giải mã** bằng Mật Khẩu (Passphrase) anh nhập duy nhất một lần trên Note 10+.
```bash
~/.zeroclaw/ota_sync.sh
```

---

## 🖥 7. Quản Trị Hệ Thống Tập Trung (v8.2 Dashboard)

Phiên bản v8.2 giới thiệu **Cyberpunk Admin Dashboard**, cho phép anh quản lý dàn máy Note 10+ và TV Box trực quan qua Web.

### 1. Cách truy cập
- **Địa chỉ**: `https://ten-worker-ota.workers.dev/admin` (Thay bằng tên Worker của anh).
- **Mã truy cập**: Sử dụng giá trị của biến môi trường `AdminPass` (Mặc định: `TradeKiemCom888`).
- **Cách đăng nhập**: Khi được hỏi mật khẩu, hãy nhập mã và bấm **DECRYPT ACCESS**.

### 2. Các chức năng "Quyền lực" dành cho Sếp
- **Duyệt máy Một-Chạm (Approval)**: 
  - Khi một máy mới được cài đặt, nó sẽ hiện ở bảng **PENDING**.
  - Anh chỉ cần bấm nút **APPROVE** trên Dashboard để kích hoạt máy từ xa.
- **Công tắc Auto-Update (Tắt/Bật)**:
  - Mỗi hàng thiết bị sẽ có một nút gạt (Toggle).
  - **ON**: Máy tự động tải và cài đặt bản nâng cấp mới nhất khi anh đổi bản Software Version.
  - **OFF**: Máy sẽ giữ nguyên phiên bản cũ, giúp anh cô lập rủi ro nếu có bản phân phối mới bị lỗi.
- **Phòng Phân Phối Cấu Hình (Global Config)**:
  - Tại bảng trên đầu, anh có thể sửa `Version` (Ví dụ 1.2, 2.0) và `Binary URL` (Link tải phần mềm mới).
  - Bấm **SAVE CONFIG** để lệnh nâng cấp này lan tỏa ra toàn bộ dàn máy (chỉ những máy đang bật ON Auto-Update).

### 3. Kiểm soát Pin & Nhiệt Độ
Dashboard sẽ báo cáo lần cuối thiết bị Online giúp anh biết v6.1 Note 10+ có đang hoạt động tốt hay không. Nếu máy lâu không Online, hãy dùng **Remote ADB** để kiểm tra!
