# 🛠️ Master Manual 99: Kỹ Thuật Chạy 15+ Agent Trên Một Thiết Bị

Tài liệu này hướng dẫn cách "lách" giới hạn của ZeroClaw để chạy hàng chục Agent (nhân sự) trên một máy Note 10+ duy nhất mà không bị xung đột.

---

## 1. Cơ Chế Phân Tách Cấu Hình (Configuration Splitting)

Mặc định ZeroClaw đọc tệp tại `~/.config/zeroclaw/config.toml`. Để chạy nhiều máy, chúng ta dùng tệp cấu hình riêng cho từng Agent:

- **Cấu trúc thư mục khuyên dùng**:
  - `~/.zeroclaw/workforce/pa/`
  - `~/.zeroclaw/workforce/ceo/`
  - `~/.zeroclaw/workforce/mkt_creator/`
  - ... và tương tự cho 12 Agent khác.

---

## 2. Cách Chạy Agent Thủ Công (Dùng Port Riêng)

Sử dụng cờ `-c` (hoặc `--config`) và `--port` để định danh:

```bash
# Chạy PA Agent (Cổng 42617)
zeroclaw gateway -c ~/.zeroclaw/workforce/pa/config.toml --port 42617 &

# Chạy CEO Agent (Cổng 42618)
zeroclaw gateway -c ~/.zeroclaw/workforce/ceo/config.toml --port 42618 &
```

---

## 3. Quản Lý Tự Động Với `termux-services` (Khuyên Dùng)

Để công ty tự động "mở cửa" khi bật máy, hãy tạo các script dịch vụ:

1. **Tạo file chạy cho PA**: `mkdir -p $SVDIR/zclaw-pa && nano $SVDIR/zclaw-pa/run`
```bash
#!/bin/bash
exec zeroclaw gateway -c ~/.zeroclaw/workforce/pa/config.toml --port 42617 2>&1
```
2. **Cấp quyền**: `chmod +x $SVDIR/zclaw-pa/run`
3. **Kích hoạt**: `sv up zclaw-pa`

*Lặp lại cho các Agent Leader khác (Trading, Marketing...). Các Agent Staff có thể chạy thủ công bằng `nohup` để tiết kiệm RAM.*

---

## 📉 4. Tối Ưu Hóa Tài Nguyên (RAM & Pin)

- **Sử Dụng API Cloud**: Tuyệt đối không chạy Model LLM nội bộ (Local) trên Note 10+. Hãy dùng NVIDIA NIM hoặc OpenRouter. Việc này giúp máy chỉ tốn ~100MB RAM cho mỗi Agent.
- **Dọn Dẹp Log**: Sử dụng lệnh gộp log để tránh làm đầy bộ nhớ máy.
---

## ⚡ 5. Xác Minh Khả Năng Xử Lý Song Song (Concurrency Proof)

Nhiều người e ngại Note 10+ sẽ bị "đơ" khi chạy 15 nhân viên. Đây là lý do tại sao nó hoạt động:

1. **Kiến trúc Native aarch64**: ZeroClaw chạy trực tiếp trên nhân Linux của Android, không qua máy ảo (Virtual Machine), giúp giảm 80% độ trễ và tiêu thụ RAM.
2. **Model Offloading**: 99% việc tính toán AI diễn ra tại trạm Cloud (NVIDIA/OpenRouter). Android chỉ đóng vai trò "Điều hướng" (Routing).
3. **Mô hình Event-Loop**: Toàn bộ giao tiếp giữa các phòng ban là bất đồng bộ (Asynchronous). Khi phòng R&D đang chờ kết quả quét web, CPU vẫn rảnh để phục vụ phòng Marketing viết bài.
4. **Quản lý Cổng (Ports)**: Kernel Android hỗ trợ hàng ngàn cổng TCP. Việc chia 15 cổng `42617-42631` là cực kỳ nhẹ nhàng, tương đương với việc bạn mở 15 tab trình duyệt nhưng không có giao diện đồ họa.

---

*(Hệ thống v13.0 đã sẵn sàng để biến điện thoại Note 10+ của Sếp thành một 'Trung Tâm Dữ Liệu' thực thụ)*
