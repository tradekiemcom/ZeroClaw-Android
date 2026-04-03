# 🏢 ZeroClaw-Android (A.I Company Edition)

Bản tùy biến kiến trúc dành cho dự án **[zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)** để chạy tự nhiên (native) trên môi trường **Termux (Android)**.
Dự án quy hoạch lại hệ thống ZeroClaw mặc định thành mô hình **Công ty Đa Đặc Vụ (Multi-Agent A.I Company)**.

## 🌟 Chức Năng Cốt Lõi
- Khởi tạo các "Phòng Ban AI" chuyên biệt: `CEO`, `Trợ lý`, `R&D`, `Marketing`, `Trading`.
- Điều phối giao việc bằng giao diện dòng lệnh `company-mgr`.
- Dự phòng nội bộ: Tự động đánh giá cấu hình điện thoại và fallback về TinyLLM (LlamaCPP) nếu không có API Key.
- Giám sát tiến độ qua Telegram hoặc Cron scheduling.

## 📙 Hướng Dẫn Sử Dụng
Mọi chi tiết về cách cài đặt, chuẩn bị cấu hình và lệnh điều hành công ty đều nằm ở đây:
👉 **[TÀI LIỆU HƯỚNG DẪN KỸ THUẬT (Tiếng Việt)](docs/HUONG_DAN_SU_DUNG.md)**

## 🚀 Cài Đặt Nhanh (trên thiết bị Android > Termux)
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
chmod +x install.sh && ./install.sh
```

*(Sau đó gõ `company-mgr` để bắt đầu quản lý)*
