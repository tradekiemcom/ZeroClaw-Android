# 🏁 Blueprint 99: Hướng Dẫn Triển Khai A-Z (Master Deployment)

Chúc mừng bạn đã sở hữu bộ khung của công ty AI "A.I Do Anything". Đây là hướng dẫn thực thi để biến các tài liệu trên thành một hệ thống máy móc hoạt động thực sự.

---

## 🏗️ Bước 1: Hạ Tầng Phần Cứng (Hardware Stack)

- **1x Samsung Galaxy Note 10+ (hoặc đời cao hơn)**: Dùng làm Trung tâm điều hành (CEO & PA).
- **5-10x Android TV Box (ARM64)**: Phân bổ cho các phòng ban (Trading, Marketing...).
- **Mạng**: Đảm bảo toàn bộ dàn máy kết nối cùng một lớp mạng WiFi (Local RPC) hoặc cài đặt Tailscale/Cloudflare Tunnel để quản lý từ xa.

---

## ☁️ Bước 2: Thiết Lập OTA Server (Trạm Trung Chuyển)

Trước khi cài từng máy, bạn phải có "Trạm Phát Lệnh":
1. Deploy **Cloudflare Worker** (Sử dụng mã nguồn trong folder `ota-server/`).
2. Cấu hình các biến môi trường: `AdminPass`, `ENCRYPTION_KEY`, `SOFTWARE_VERSION`.
3. Truy cập Dashboard `/admin` để sẵn sàng phê duyệt thiết bị.

---

## 📲 Bước 3: Cài Đặt Hàng Loạt (Mass Installation)

Trên mỗi thiết bị (Note 10+, Box), mở Termux và chạy lệnh cài đặt duy nhất:
```bash
git clone https://github.com/tradekiemcom/ZeroClaw-Android.git
cd ZeroClaw-Android && bash install.sh
```

---

## 🎭 Bước 4: Phân Vai Nhân Sự (Role Assignment)

Đây là lúc bạn áp dụng các Blueprint đã đọc:
1. Mở Dashboard Web `/admin`.
2. Thấy thiết bị mới hiện lên -> Bấm **APPROVE**.
3. **Customize Config**: Đối với từng máy, bạn hãy nạp mẫu `config.toml` tương ứng từ các Blueprint:
   - Máy A -> Copy nội dung từ `01_PHONG_TRADING.md`.
   - Máy B -> Copy nội dung từ `03_PHONG_MARKETING.md`.
   - ...

---

## 📡 Bước 5: Kích Hoạt Chuỗi Liên Lạc (Internal Gateway)

Trên máy Note 10+ (Đóng vai PA/CEO), hãy thực hiện:
1. Mở `config.toml`.
2. Thêm các Agents khác vào mục `[tools]` bằng địa chỉ IP của chúng.
3. Chạy `zeroclaw gateway`.

Bây giờ, khi bạn nhắn tin cho PA: *"Hãy yêu cầu phòng Marketing lên kịch bản video cho Trading hôm nay"*, PA sẽ tự động gọi CEO, CEO gọi Marketing Staff, và quy trình công ty bắt đầu vận hành!

---

## 🛡️ Bảo Trì & Nâng Cấp
- Sử dụng Dashboard để theo dõi tình trạng Online của các Agent.
- Tận dụng tính năng **Auto-Update** của ZeroClaw v8.2 để nâng cấp toàn bộ dàn nhân sự chỉ với 1 click.

**🚀 CHÚC CÔNG TY "A.I DO ANYTHING" CỦA BẠN THÀNH CÔNG RỰC RỠ!**
