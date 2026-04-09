# Hướng dẫn Khởi tạo Tính năng Remote ADB & Theo dõi Pin (v7.3)

Để bản ZeroClaw-Android hoạt động full tính năng "Siêu cấp", anh cần thực hiện 2 bước đơn giản sau:

### 1. Cài đặt Phụ trợ Termux:API
Đây là "cầu nối" để ZeroClaw đọc được thông số Pin và phần cứng máy anh.
- **Link tải F-Droid (Khuyên dùng):** [Termux:API F-Droid](https://f-droid.org/en/packages/com.termux.api/)
- **Lưu ý:** Anh phải tải Termux và Termux:API cùng một nơi (cùng F-Droid hoặc cùng Play Store) thì chúng mới "nói chuyện" được với nhau.
- **Sau khi cài xong:** Anh hãy mở app Termux:API lên một lần để hệ thống cấp quyền.

### 2. Bật ADB Wireless (Gỡ lỗi qua WiFi)
Tính năng này cho phép ZeroClaw điều khiển máy từ xa và thực hiện các lệnh hệ thống cấp cao.
- Bước 1: Vào **Cài đặt** > **Thông tin điện thoại** > Bấm 7 lần vào **Số hiệu bản dựng** để mở "Tùy chọn cho nhà phát triển".
- Bước 2: Vào **Tùy chọn cho nhà phát triển** > Bật **Gỡ lỗi USB**.
- Bước 3: Tìm mục **Gỡ lỗi qua Wi-Fi** (Wireless Debugging) và bật nó lên.
- **Lưu ý:** ZeroClaw sẽ tự động thực hiện lệnh `adb connect localhost:5555` ngay sau khi anh bật tính năng này.

### 3. Tắt tối ưu hóa Pin (Battery Optimization)
Để đảm bảo ZeroClaw không bị Android "giết" khi anh tắt màn hình:
- Vào **Cài đặt** > **Ứng dụng** > **Termux**.
- Chọn **Pin** > Chọn **Không hạn chế** (Unrestricted).

---
*Bản v7.3 đã được cấu hình sẵn để tự động nhận diện các thiết lập này ngay khi anh kích hoạt.*
