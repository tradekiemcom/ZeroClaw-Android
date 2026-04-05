# Hướng Dẫn Tao Cloudflare Worker Cho OTA Môi Trường ZeroClaw

Trạm OTA (Over-The-Air) đóng vai trò là lõi cung cấp cấu hình thiết lập ban đầu (API keys, Token, Mật khẩu hệ thống) một cách bảo mật tới điện thoại chạy Termux mà bạn không cần phải copy-paste rườm rà.

Bạn có thể tự tay tạo Trạm OTA này bằng cách triển khai mã nguồn trên thư mục `ota-server/` thông qua 2 cách:

---

## 🛠 Cách 1: Tạo Thủ Công Trực Tiếp Trên Trình Duyệt (Cloudflare Dashboard)

Cách này phù hợp nếu bạn không muốn cài đặt các công cụ lập trình hay kết nối rườm rà.

### Bước 1: Tạo Worker Mới
1. Đăng nhập vào trang quản trị [Cloudflare Dashboard](https://dash.cloudflare.com/).
2. Chọn mục **Workers & Pages** ở cột menu bên trái.
3. Bấm nút **Create application** (Tạo ứng dụng).
4. Bấm **Create Worker** (Tạo Worker).
5. Đặt tên là `zeroclaw-ota-server` và bấm **Deploy** để khởi tạo Worker mặc định.

### Bước 2: Dán Mã Nguồn
1. Bấm nút **Edit code** trên đầu trang quản trị của Worker bạn vừa tạo.
2. Trên máy tính của bạn, mở file `ota-server/src/index.js` bằng Notepad hoặc VS Code.
3. Copy **toàn bộ nội dung** trong file `index.js` đó.
4. Trở lại trình duyệt Cloudflare, xóa sạch mã code mặc định ở khung soạn thảo, dán phần nội dung bạn vừa cài đặt vào.
5. Bấm nút **Save and deploy** (Lưu và triển khai).

### Bước 3: (Cực Kỳ Quan Trọng) Bật `nodejs_compat` 
Bởi hệ thống mã hóa OTA của chúng ta dùng thư viện `crypto` của Node.js, bạn bắt buộc phải bật cờ tương thích.
1. Quay lại trang thông tin chi tiết của Worker `zeroclaw-ota-server`.
2. Trỏ tới Tab **Settings** (Cài đặt) -> Cột bên trái chọn **Functionality** (Chức năng).
3. Cuộn xuống phần **Compatibility flags** (Cờ tương thích) và bấm **Edit**.
4. Gõ thêm chữ `nodejs_compat` vào danh sách cờ rồi lưu lại.

### Bước 4: Khai Báo Biến Môi Trường (Environment Variables)
Đến đây bạn khai báo Mật Khẩu và các API Key quan trọng.
1. Tại Tab **Settings**, chọn mục **Variables** (Biến số).
2. Cuộn xuống phần **Environment Variables** và bấm **Edit variables** (Thêm biến/Sửa biến).
3. Lần lượt bấm Add variable và nhập ĐÚNG các tên biến sau bên cột trái, và Cấu hình mẫu ở cột phải:
   - `OTA_VERSION`: Nhập `1.2.6` (Hoặc số phiên bản bạn lưu ý).
   - `TELEGRAM_IDS`: Nhập `975318323, 7237066439` (ID của Sếp để cấp quyền lệnh bot Telegram).
   - `ENCRYPTION_KEY`: Nhập một Mật Khẩu bí mật bất kì, ví dụ `TradeKiemCom123@!`. *(**Lưu ý**: Đây chính xác là cái Passphrase điện thoại sẽ hỏi ở Bước Dán Cấu Hình lúc Setup)*
   - `TUNNEL_TOKEN`: Nhập Token do Cloudflare cấp của Tunnel Zero Trust.
   - `OPENROUTER_KEY`: Nhập khóa API của OpenRouter.
   - `CF_AI_KEY`: Nhập khóa API cho mô hình nội bộ của Cloudflare (nếu có).
   - `NVIDIA_NIM_KEY`: Nhập khóa NVIDIA (nếu có).
4. Bấm **Save and deploy**.

### Bước 5: (Tùy chọn) Ràng Buộc Tên Miền
1. Trở về Tab **Settings**, chọn **Triggers**.
2. Tại phần **Custom Domains**, thêm và gõ `ota.tradekiem.com` (Phải đảm bảo tên miền tradekiem.com đã kết nối cho Cloudflare của bạn).

Trạm OTA bây giờ đã sẵn sàng lắng nghe ở địa chỉ Cloudflare `.workers.dev` gốc hoặc tên miền bạn gán. Mọi điện thoại lúc cài `install.sh` đều có thể tải xuống.

---

## 💻 Cách 2: Triển Khai Chuyên Nghiệp Bằng Trình Lệnh (Wrangler CLI)

Nếu bạn có sẵn MacOS/Linux/Windows Terminal.

### Bước 1: Cấu hình mã nguồn
1. Mở file `ota-server/wrangler.toml` trên máy bạn.
2. Điền sẵn các giá trị cấu hình vào khu vực `[vars]`. (Ví dụ: `ENCRYPTION_KEY = "MatDauBaoMatCuaSep"`).
3. Sửa định tuyến (Routes) nếu bạn muốn đẩy thẳng lên tên miền bạn sở hữu.

### Bước 2: Tải Wrangler và Đẩy Mạng
1. Mở cửa sổ Terminal tại khu vực thư mục `ota-server/`:
   ```bash
   cd ZeroClaw-Android/ota-server
   ```
2. Cài đặt Wrangler và ra lệnh đẩy Code (Yêu cầu máy tính cài sẵn Node.JS):
   ```bash
   npm i -g wrangler
   npx wrangler login 
   # Trình duyệt sẽ mở ra, bạn đăng nhập bấm Allow để cấp quyền lệnh CLI
   npx wrangler deploy
   ```

3. Mọi công việc cấu hình, cờ `nodejs_compat`, khai báo biến đều đã được `Wrangler` tự động đọc từ file `wrangler.toml` rồi mang lên máy chủ của Cloudflare nhanh chóng. Quá trình mất khoảng 30s.

---
**🎉 Khởi Tạo Thành Công!** 
- Khi trạm OTA đã "lên sóng", ở bên điện thoại chạy lệnh `bash ~/.zeroclaw/ota_sync.sh` sẽ nhận được Cấu Hình Giải Mã thành công. Lỗi thiết lập sẽ hoàn toàn biến mất.
