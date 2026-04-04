# 💼 ZeroClaw-Android (A.I Company Edition)

Hệ thống điều hành A.I tự trị dành riêng cho môi trường **Sử dụng trên nguồn mở Termux/Android** không cần quyền Root.
Dự án được tối ưu hóa đặc biệt bằng cơ chế **Build from Source** qua Cargo Rust để đảm bảo hiệu suất tốt nhất trên điện thoại, đồng thời ứng dụng bộ khung nhân sự **18 Đặc Vụ AI Chuyên Sâu** thay vì một thực thể duy nhất.

---

## 🏗 Kiến Trúc Kỹ Thuật
Dự án sẽ tải trực tiếp mã nguồn của [ZeroClaw Core](https://github.com/zeroclaw-labs/zeroclaw) xuống thiết bị Android và tiến hành biên dịch (compile) gốc bằng trình biên dịch chuẩn C/C++ & Rust từ Termux ngay trên vi xử lý của điện thoại. Quá trình này **loại bỏ sự cần thiết của quyền Sudo/Root**. 

## 👔 Bản Đồ Tổ Chức
Cấu trúc "Company" được định nghĩa lại xoay quanh mục tiêu cốt lõi: Mang lại doanh thu cho Founder (anh Hưng).
1. **Lãnh Đạo & Vận Hành**: CEO, Trợ Lý (Thảo Agent), GĐ Vận Hành (CPO), Chuyên gia luồng/Tối ưu.
2. **Nghiên Cứu & Phát Triển (R&D)**: CTO, Coder, Research, Analytics.
3. **Kinh Tế (Finance / Trading)**: CFO, TradeFx, TradeGold, TradeFund, Kế toán (Analyst).
4. **Lan Tỏa Dịch Vụ**: CMO (Quản lý Marketing), CS (Chăm sóc Khách hàng), Sales.

## 📙 Hướng Dẫn Sử Dụng
Mọi chi tiết về cách cài đặt, chuẩn bị cấu hình và lệnh điều hành công ty đều nằm ở đây:
👉 **[TÀI LIỆU HƯỚNG DẪN KỸ THUẬT & TOML](docs/HUONG_DAN_SU_DUNG.md)**

## 🚀 Lệnh Cài Đặt Ban Đầu (Trên Termux)
```bash
pkg update -y && pkg install -y git
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android
git submodule update --init --recursive
chmod +x install.sh && ./install.sh
```

*(Sau đó gõ `company-mgr` để bắt đầu quản lý)*
