# 🏛️ Master Manual 01: Thiết Lập Nhân Sự Cốt Lõi (PA & CEO)

Tài liệu này hướng dẫn chi tiết cách thiết lập 2 nhân sự quan trọng nhất: **PA Agent** (Giao tiếp với Founder) và **CEO Agent** (Điều hành công ty) chạy trên cùng một máy Note 10+.

---

## 1. PA Agent (Personal Assistant) - Port: 42617

### A. Tệp cấu hình: `~/.zeroclaw/agents/pa/config.toml`
```toml
[agent]
name = "PA_Founder_Assistant"
model = "meta-llama/llama-3.3-70b-instruct" # Chạy qua NVIDIA NIM
system_prompt = \"\"\"
Bạn là Trợ lý Cá nhân (PA) duy nhất của Ngài sáng lập (Founder). 
Nhiệm vụ:
1. Tiếp nhận mọi yêu cầu từ Sếp và chuyển cho CEO.
2. Không bao giờ tự ý hứa hẹn với khách hàng nếu chưa có lệnh từ CEO.
3. Luôn báo cáo súc tích, trung thực.
Bạn có quyền truy cập 'CEO_Tool' để gửi lệnh xuống bộ máy công ty.
\"\"\"

[tools]
enabled = ["ceo_command"]

[tool.ceo_command]
url = "http://localhost:42618/v1/chat"
description = "Gửi yêu cầu công việc cho CEO Agent để điều phối các phòng ban."
```

---

## 2. CEO Agent (Chief Executive Officer) - Port: 42618

### A. Tệp cấu hình: `~/.zeroclaw/agents/ceo/config.toml`
```toml
[agent]
name = "CEO_ZeroClaw_Corp"
model = "anthropic/claude-3.5-sonnet" # Chạy qua OpenRouter
system_prompt = \"\"\"
Bạn là CEO của công ty A.I Do Anything. 
Dưới quyền bạn là 5 Trưởng phòng: Trading, Marketing, Finance, Sales, R&D.
Nguyên tắc vận hành:
- Nhận lệnh từ PA.
- Chia nhỏ nhiệm vụ và gọi Tools của các phòng ban tương ứng.
- Tuyệt đối không làm việc chi tiết của nhân viên, chỉ điều phối.
- Định kỳ tóm tắt trạng thái của các dự án để báo cáo ngược cho PA.
\"\"\"

[tools]
enabled = ["marketing_dept", "trading_dept", "finance_dept"]

[tool.trading_dept]
url = "http://localhost:42619/v1/chat"
description = "Phòng Trading: Xử lý phân tích vàng, btc và giao dịch quỹ."

[tool.marketing_dept]
url = "http://localhost:42620/v1/chat"
description = "Phòng Marketing: Xây dựng content và quản lý kênh social."
```

---

## 🔄 3. Luồng Hoạt Động (The Founder Flow)

1. **Founder**: "Hãy chuẩn bị báo cáo thị trường vàng và kịch bản video TikTok cho ngày mai."
2. **PA**: Nhận lệnh -> Gọi `ceo_command` với nội dung yêu cầu.
3. **CEO**: Phân tích yêu cầu -> Gọi `trading_dept` (xin dữ liệu vàng) -> Gọi `marketing_dept` (yêu cầu viết kịch bản dựa trên dữ liệu vàng).
4. **Phòng ban**: Trả kết quả về CEO.
5. **CEO**: Kiểm tra chất lượng -> Gửi báo cáo tổng hợp cho PA.
---

## 🎭 4. Case Study: Chiến Dịch Truyền Thông Chiến Lược (v13.0)

Đây là ví dụ về cách CEO Agent điều phối một yêu cầu phức tạp từ Founder (Sếp).

**Yêu cầu của Sếp**: *"Hãy triển khai một kịch bản bán gói 'Gold Scalper VIP' mới cho tháng này."*

### Luồng xử lý đa tầng (Orchestration):
1. **PA Agent**: Nhận lệnh -> Chuyển cho CEO với mức ưu tiên `STRICT`.
2. **CEO Agent**: Phân rã yêu cầu thành các "Ticket" công việc:
   - **Gửi R&D**: "Nghiên cứu các đối thủ đang bán gói Vàng và tìm ra 3 điểm yếu của họ."
   - **Gửi Trading**: "Cung cấp báo cáo lợi nhuận (myfxbook/backtest) tốt nhất của tháng trước để làm bằng chứng (Proof)."
   - **Gửi Marketing**: "Dựa trên dữ liệu R&D và Trading, hãy viết 5 kịch bản video TikTok và 1 bài landing page."
   - **Gửi Sales**: "Lập danh sách khách hàng tiềm năng từ dữ liệu cũ và chuẩn bị kịch bản tư vấn mới."
3. **CEO Agent**: Theo dõi kết quả từ các Port API local (`42619-42627`).
4. **Kết thúc**: CEO tổng hợp toàn bộ thành một "Chiến lược tổng thể" và báo cáo PA gửi Sếp.

---

## 📡 5. Xác minh Xử lý Song song (Concurrency)
Toàn bộ quy trình trên diễn ra **đồng thời**. Trong khi R&D đang nghiên cứu, Marketing đã có thể bắt đầu lên khung sườn Content. Note 10+ xử lý các luồng JSON-RPC này trong hàng chục mili giây nhờ Model được offload sang trạm Cloud.
