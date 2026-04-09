# 💰 Blueprint 02: Phòng Tài Chính (Quản Lý Dòng Tiền & Kho Quỹ)

Phòng Tài chính đảm bảo "mạch máu" cho công ty hoạt động, quản lý từ chi phí API đến lợi nhuận từ Trading.

---

## 👨‍💼 1. CFO Agent (Chief Financial Officer)
- **Thiết bị**: Android Box (Duy trì online liên tục).
- **Model**: `meta-llama/llama-3.1-405b` (NVIDIA NIM) - Khả năng tính toán và logic tài chính cực tốt.
- **Nhiệm vụ**: Phê duyệt các khoản chi và lập báo cáo tài chính hàng tuần cho CEO.

---

## 👷 2. Đội Ngũ Nhân Sự (Agent Staff)

### 🧺 Agent Quản Lý Đa Ví (Wallet Manager)
- **Model**: `meta-llama/llama-3.3-70b-instruct`.
- **Nhiệm vụ**: Theo dõi số dư và lịch sử giao dịch của 3 nhóm ví:
  1. **Ví Vận Hành (Operating)**: Trả phí API (OpenRouter, Nvidia), điện, server.
  2. **Ví Đầu Tư (Investment)**: Chứa vốn để phân phối cho Phòng Trading.
  3. **Ví Founder (Sếp)**: Ví chứa lợi nhuận ròng để rút về tài khoản cá nhân của anh.
- **Skill**: Tích hợp quét Blockchain Explorer (Etherscan, Solscan) hoặc MT5 Balance.

### 📈 Agent Phân Tích Dòng Tiền & ROI
- **Model**: `google/gemini-1.5-pro`.
- **Nhiệm vụ**: Tính toán hiệu quả sinh lời (ROI) của từng phòng ban (ví dụ Phòng Trading tháng này lời bao nhiêu %).

---

## 🛠️ 3. Luồng Giao Tiếp (Internal Flow)

1. **Phòng Trading** báo cáo: "Lợi nhuận tuần này là 5,000$".
2. **CFO Agent**: Kiểm tra số dư ví đầu tư xác nhận khớp dữ liệu.
3. **CFO Agent**: Tự động trích 30% về Ví Founder, 10% về Ví Vận Hành, 60% tái đầu tư.
4. **CFO Agent**: Gửi báo cáo hoàn tất cho CEO.

---

## 🔗 Chuyển đến các phòng ban
- 👉 [Phòng Marketing: Sáng tạo & Truyền thông](03_PHONG_MARKETING.md)
- 👉 [Phòng Sale & CS: Chốt đơn & Chăm sóc](04_PHONG_SALES_CS.md)
