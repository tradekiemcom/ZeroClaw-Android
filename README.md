# 🐾 ZeroClaw-Android

Dự án cài đặt **[ZeroClaw Core](https://github.com/zeroclaw-labs/zeroclaw)** chuyên biệt để chạy tự nhiên (native) trên môi trường **Termux (Android)** mà không cần quyền Root hay bất kỳ công cụ Linux Proot nào.

---

## 🌟 Chức Năng Cốt Lõi
- Cung cấp kịch bản cài đặt tĩnh (tải bản Binary aarch64 native) lược bỏ quyền sudo.
- Thiết lập sẵn các công cụ giao tiếp liên mạng thông qua Cloudflare Tunnel cho Android.
- Tự động hóa đánh giá năng lực phần cứng để khuyến cáo sử dụng phù hợp.

## 🆕 Tính Năng Mới (v8.2 - Autonomous Release)
- **Admin Dashboard**: Quản lý thiết bị trực quan qua giao diện Web Cyberpunk.
- **Duyệt máy Một-Chạm**: Admin phê duyệt thiết bị ngay trên Web thay vì sửa DB.
- **Granular Auto-Update**: Bật/Tắt tự động cập nhật phần mềm cho từng thiết bị riêng biệt.
- **OTA Daemon (vĩnh cửu)**: Tiến trình chạy ngầm tự động đồng bộ cấu hình và tự nâng cấp phiên bản lõi.

## 🚀 Cài Đặt Nhanh (trên thiết bị Android > Termux)
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
chmod +x install.sh && ./install.sh
```

## 🏢 Hệ Sinh Thái Doanh Nghiệp AI (Corporate OS)
ZeroClaw không chỉ là tool kỹ thuật, nó là giải pháp cho bài toán kinh doanh:
- 👑 **[THE FULL AI CORPORATE BLUEPRINT (v10.0)](docs/BLUEPRINT_CORPORATE/99_HUONG_DAN_TRIEN_KHAI_FULL.md)**: Hướng dẫn A-Z xây dựng công ty AI 5 phòng ban.
- 💡 [Tầm nhìn Hệ sinh thái Doanh nghiệp](docs/HE_SINH_THAI_DOAN_NGHIEP.md): Trading, Content, Sales.
- 📋 [Thư viện Flow mẫu cho nhân sự AI](docs/FLOWS_MAU_DOAN_NGHIEP.md): Copy-paste cấu hình Agent.
- 🏬 [Mô hình Agent nội bộ (Internal RPC)](docs/MO_HINH_CONG_TY_AI.md): Cách Agent kết nối không dùng Telegram.

## 📙 Tài liệu kỹ thuật
- 👉 **[Hướng dẫn sử dụng Dashboard v8.2](docs/HUONG_DAN_SU_DUNG.md)**
- 👉 **[Cách thiết lập OTA Server (Worker & Dashboard)](docs/HUONG_DAN_TAO_OTA_WORKER.md)**
- 👉 **[Tối ưu Pin & Remote ADB](docs/BATTERY_ADB_SETUP.md)**

*(Sau khi cài đặt thành công, đơn giản là truy cập `/admin` trên Worker của bạn để quản lý)*
