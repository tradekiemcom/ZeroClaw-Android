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
- **Tắt Battery Optimization**: Vào cài đặt Android, chọn Termux là "Unrestricted" (Không hạn chế) để dàn nhân sự không bị "ngủ quên".

---

## 🏁 Kết Luận
Với cấu trúc này, máy Note 10+ của bạn không còn là điện thoại nữa, nó là một **Data Center Thu Nhỏ** vận hành cả một tập đoàn AI.
