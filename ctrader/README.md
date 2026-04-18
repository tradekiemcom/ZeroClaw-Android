# ZeroClaw cTrader Module - Hệ thống Giao dịch AI Local

Module cTrader của hệ sinh thái ZeroClaw là giải pháp quản lý giao dịch tập trung, hỗ trợ đa tài khoản và tích hợp **AI Agent cục bộ (Local LLM)** để xử lý ngôn ngữ tự nhiên.

## Tinh nang noi bat

- **Local AI Agent**: Tích hợp model **Qwen 2.5 - 1.5B** chạy trực tiếp trên thiết bị (ARM/Android/PC). Không cần API Key bên ngoài, bảo mật tuyệt đối.
- **Quản lý Đa tài khoản**: Kết nối đồng thời nhiều tài khoản cTrader qua Open API.
- **Bot Management**: Bật/tắt và giám sát hiệu suất từng Bot riêng lẻ bằng lệnh rút gọn (/a1, /d1, /c1).
- **Giao diện Song song**: 
    - **CLI (Console)**: Giao diện dòng lệnh chuyên nghiệp hỗ trợ chế độ Dashboard thời gian thực.
    - **Telegram Bot**: Điều khiển từ xa qua chatbot với bàn phím chức năng đầy đủ.
- **Báo cáo chuyên sâu**: Phân loại lệnh theo Account hoặc theo Bot, thống kê P&L Float và Lịch sử hàng ngày.

## Yeu cau he thong

- **Bộ nhớ**: Tối thiểu 1.5GB trống để chứa modelweights và tokenizer.
- **RAM**: Khuyến nghị > 2GB (Model chiếm ~1GB khi hoạt động).
- **Phần cứng**: Hỗ trợ tốt kiến trúc ARM (Android-Termux) và x86 (Linux/Windows/Mac).

## Huong dan nhanh

### 1. Khởi động AI Agent
Mặc định AI sẽ ở trạng thái OFF để tiết kiệm tài nguyên.
- Gõ `/agent on`: Để nạp model vào RAM và bắt đầu trò chuyện.
- Gõ `/agent off`: Để giải phóng bộ nhớ khi không cần dùng.

### 2. Lệnh Telegram / CLI phổ biến
- `/r`: Xem danh sách lệnh đang mở (nhóm theo Bot/Account).
- `/rb`: Báo cáo hiệu suất chi tiết của các Bot.
- `/status`: Dashboard tổng quan toàn hệ thống.
- `#BUY XAUUSD 0.1`: Lệnh vào lệnh trực tiếp (Không qua AI).
- "Mua vàng giúp anh với risk 1%": Lệnh ngôn ngữ tự nhiên (Qua AI Agent).

## Tai lieu chi tiet

- [Hướng dẫn sử dụng chi tiết](docs/USER_GUIDE.md)
- [Kiến trúc Kỹ thuật & AI Agent](docs/AI_AGENT.md)
- [Cấu hình hệ thống](docs/GUIDE.md)

---
*Phát triển bởi ZeroClaw Team & Hưng.*
