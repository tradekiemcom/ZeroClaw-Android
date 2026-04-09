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

## 🏢 Cẩm Nang Doanh Nghiệp AI (Autonomous Corp v12.0)
Hệ thống tài liệu **"Cầm tay chỉ việc"** để thiết lập 15+ nhân sự AI trên một thiết bị:
- 👑 **[MASTER MANUAL: CORE PA & CEO](docs/MASTER_MANUAL/01_CORE_STRUCTURE.md)**: Hướng dẫn thiết lập bộ não điều hành.
- 💹 **[MARKET ANALYST & TRADING](docs/MASTER_MANUAL/02_TRADING_DASHBOARD.md)**: Hệ thống soi kèo và thực thi lệnh.
- 📣 **[CONTENT CREATOR & SOCIAL](docs/MASTER_MANUAL/03_MARKETING_ENGINE.md)**: Nhà máy sản xuất nội dung tự động.
- 🏦 **[FINANCE, SALES & R&D](docs/MASTER_MANUAL/04_FINANCE_SALES_RD.md)**: Quản lý dòng tiền, chốt sale và săn công nghệ.
- 🛠️ **[KỸ THUẬT CHẠY ĐA AGENT (SINGLE-DEVICE)](docs/MASTER_MANUAL/04_FINANCE_SALES_RD.md)**: Cách chạy hàng chục Agent trên 1 máy Note 10+.

## 📙 Tài liệu kỹ thuật
- 👉 **[Hướng dẫn sử dụng Dashboard v8.2](docs/HUONG_DAN_SU_DUNG.md)**
- 👉 **[Cách thiết lập OTA Server (Worker & Dashboard)](docs/HUONG_DAN_TAO_OTA_WORKER.md)**

*(Sau khi cài đặt thành công, hãy truy cập `/admin` trên Worker của bạn để phê duyệt thiết bị)*
