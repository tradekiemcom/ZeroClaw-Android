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

### Bước 4: Khai Báo Biến Môi Trường (v8.2)

Tại Tab **Settings** > **Variables and Secrets**, anh hãy khai báo các biến sau:

| Tên Biến | Cấu hình mẫu | Mô tả |
| :--- | :--- | :--- |
| `AdminPass` | `TradeKiemCom888` | Mật khẩu để đăng nhập Dashboard `/admin`. |
| `ENCRYPTION_KEY`| `TradeKiemCom123@!` | Mã mã hóa chung (Shared Secret) cho file config. |
| `OTA_VERSION` | `8.2.0` | Phiên bản OTA hiện tại. |
| `TELEGRAM_IDS` | `975318323, 7237066439` | Danh sách ID admin được quyền chat Telegram. |
| `SOFTWARE_VERSION`| `1.0.0` | Phiên bản phần mềm ZeroClaw Core. |
| `BINARY_URL` | `https://.../zeroclaw` | Link tải bản cập nhật binary (nếu cần Auto-Update). |

---

## 🖥 IV. QUẢN TRỊ QUA DASHBOARD (v8.2)

Từ phiên bản v8.2, anh không cần sửa KV thủ công nữa. Hãy sử dụng giao diện Web:

1. **Truy cập**: `https://ten-worker-cua-anh.workers.dev/admin`
2. **Đăng nhập**: Nhập `AdminPass` đã cài ở Bước 4.
3. **Phê duyệt**: Các máy mới cài đặt sẽ hiện ở bảng **CONNECTED DEVICES**. Anh chỉ cần bấm **APPROVE** là xong.
4. **Cấu hình Version**: Anh có thể đổi bản `Software Version` và `Binary URL` ngay trên Web. Các máy đang bật **Auto-Update** sẽ tự động thấy và nâng cấp sau mỗi 10 phút.

---

*(Mọi thiết bị điện thoại giờ đây đều có thể Discovery tự động, Sếp chỉ việc ngồi trước màn hình và Duyệt!)*
