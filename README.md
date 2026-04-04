# 🐾 ZeroClaw-Android

Dự án cài đặt **[ZeroClaw Core](https://github.com/zeroclaw-labs/zeroclaw)** chuyên biệt để chạy tự nhiên (native) trên môi trường **Termux (Android)** mà không cần quyền Root hay bất kỳ công cụ Linux Proot nào.

---

## 🌟 Chức Năng Cốt Lõi
- Cung cấp kịch bản cài đặt tĩnh (tải bản Binary aarch64 native) lược bỏ quyền sudo.
- Thiết lập sẵn các công cụ giao tiếp liên mạng thông qua Cloudflare Tunnel cho Android.
- Tự động hóa đánh giá năng lực phần cứng để khuyến cáo sử dụng phù hợp.

## 📙 Hướng Dẫn Sử Dụng
Mọi chi tiết về cách cấu hình ZeroClaw sau cài đặt đều nằm ở đây:
👉 **[TÀI LIỆU HƯỚNG DẪN KỸ THUẬT (Tiếng Việt)](docs/HUONG_DAN_SU_DUNG.md)**

## 🚀 Cài Đặt Nhanh (trên thiết bị Android > Termux)
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
chmod +x install.sh && ./install.sh
```

*(Sau đó gõ `zeroclaw agent` để lập tức trò chuyện)*
