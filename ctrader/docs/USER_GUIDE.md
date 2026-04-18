# Hướng dẫn sử dụng Hệ thống iZTrade cTrader

Chào mừng bạn đến với hệ thống quản lý giao dịch iZTrade. Tài liệu này hướng dẫn cách sử dụng các tính năng từ cơ bản đến nâng cao.

## 1. Hệ thống ra lệnh (Commands)

Hệ thống hỗ trợ 3 cách ra lệnh linh hoạt:

### A. Lệnh viết tắt (Fast Commands) - TỐC ĐỘ CAO
Ưu tiên dùng khi cần thao tác nhanh trên các Bot (áp dụng cho Telegram & CLI):
- `/a[N]`: Kích hoạt (Enable) Bot số N. (Ví dụ: `/a1`)
- `/d[N]`: Vô hiệu hóa (Disable) Bot số N. (Ví dụ: `/d1`)
- `/c[N]`: Đóng toàn bộ lệnh và vô hiệu hóa Bot số N. (Ví dụ: `/c1`)

### B. Lệnh cấu trúc (Direct Commands)
Dùng để quản lý hệ thống:
- `/r`: Xem danh sách tất cả vị thế (Nhóm theo Bot/Account).
- `/rb`: Xem báo cáo hiệu suất của các Bot (P&L hôm nay, số lệnh).
- `/status`: Xem Dashboard tổng quan hệ thống.
- `/list`: Danh sách các tài khoản đang kết nối.

### C. Giao dịch thủ công (Manual Trade)
Cấu trúc: `#ACTION SYMBOL VOLUME`
- `#BUY XAUUSD 0.1`
- `#SELL BTCUSD 0.05`
- `#CLOSE XAUUSD`: Đóng toàn bộ lệnh Vàng.

## 2. Sử dụng AI Agent (Trợ lý thông minh)

AI Agent là "bộ não" có khả năng hiểu tiếng Việt và phân tích dữ liệu tài khoản giúp bạn.

### Cách sử dụng:
1. **Bật AI**: Gõ `/agent on`. Đợi vài giây để model nạp vào RAM.
2. **Ra lệnh**: Chat trực tiếp như người thật.
   - *Ví dụ*: "Tài khoản sao rồi em, có lệnh nào đang lỗ nhiều không?"
   - *Ví dụ*: "Vàng đang đẹp, mua cho anh 0.1 ở tài khoản chính đi."
3. **Tắt AI**: Gõ `/agent off` để giải phóng bộ nhớ (khuyên dùng khi không có nhu cầu dùng AI).

## 3. Quản lý trên Telegram

Giao diện Telegram được tối ưu với các hàng phím bấm:
- **Hàng 1**: Info, Dashboard, Quay lại.
- **Hàng 2**: Truy cập nhanh Bots và Các vị thế.
- **Hàng 3**: Bật/Tắt trợ lý AI.
- **Hàng 4**: Các loại Báo cáo chuyên sâu.

## 4. Quản lý trên CLI (Dòng lệnh)

Khi chạy trực tiếp trên PC hoặc Termux:
- Nhập lệnh trực tiếp vào Console.
- Sử dụng phím `tab` (nếu hỗ trợ) hoặc gõ `help` để xem danh sách lệnh.
- Chế độ Dashboard tự động cập nhật mỗi khi có thay đổi.

---
*Lưu ý: Luôn kiểm tra kết nối internet và trạng thái cTrader Open API để đảm bảo lệnh được thực thi chính xác.*
