# 💹 Master Manual 06: Quản Lý Danh Mục (Portfolio) & Rủi Ro

Đây là cẩm nang dành cho **Trade Leader** - Người chịu trách nhiệm điều phối hàng loạt tài khoản khách hàng và quản trị rủi ro cho toàn bộ quỹ.

---

## 🏗️ 1. Phân Loại Danh Mục Tài Khoản (Portfolio Categorization)

Trade Leader quản lý tài khoản dựa trên các nhóm chức năng sau trong `account_registry.json`:

| Loại Tài Khoản | Đặc Điểm | Chiến Lược Phù Hợp |
| :--- | :--- | :--- |
| **Strategic (Master)** | Tài khoản gốc, dùng để phát tín hiệu. | EA Scalping Gold v1.0, Swing Forex. |
| **Investor (Copy)** | Tài khoản khách hàng, sao chép lệnh từ Master. | Gắn kết nối thông qua Copy-Trade skill. |
| **Prop-Fund (Quỹ)** | Tài khoản FTMO/The5ers, rủi ro cực thấp. | EA Fund Specialist (Strict SL). |

---

## ⚖️ 2. Quản Trị Rủi Ro & Phân Bổ Nguồn Lực

Trade Leader không chỉ "nhìn" mà phải "hành động":

- **Cân Bằng Vốn**: Nếu EA Vàng đang thắng lớn, Leader có thể yêu cầu Finance Agent tăng hạn mức vốn cho máy chạy Vàng.
- **Cắt Lỗ Tập Trung**: Nếu thị trường có biến động cực lớn (Thiên nga đen), Leader ra lệnh cho toàn bộ Staff: *"Đóng toàn bộ vị thế và ngừng giao dịch trong 24h."*
- **Đánh Giá Hiệu Suất**: Mỗi cuối tuần, Leader quét log để chấm điểm từng EA. Nếu EA nào ROI thấp, sẽ bị đưa vào danh sách "Sửa đổi" gửi cho R&D.

---

## 🛠️ 3. Vòng Đời Phát Triển Chiến Lược (EA Lifecycle)

Đây là quy trình hiệp động giữa Trading -> CEO -> R&D:

1. **Phát hiện nhu cầu**: Trade Leader báo cáo: *"Hiện tại Gold Scalper v1 đang gặp khó khi Spread cao, cần một EA tập trung vào Scalping đêm (Late Night)."*
2. **Giao việc**: CEO Agent nhận lệnh, tạo Ticket và gửi xuống **Phòng R&D**.
3. **Phát triển**: AI Developer (R&D) viết code, tích hợp vào một **ZeroClaw Skill** mới.
4. **Nghiệm thu**: R&D chạy Backtest 99% data, gửi báo cáo cho Trade Leader.
5. **Triển khai**: Trade Leader cài đặt Skill mới lên 2 máy TV Box để chạy thử (Forward Test) trước khi áp dụng cho tài khoản quỹ.

---

## 💾 4. Mẫu Dữ Liệu Quản Lý (account_registry.json)

```json
{
  "accounts": [
    {
      "id": "AC-9988",
      "type": "strategic",
      "owner": "Founder",
      "strategy": "gold_scalper_v2",
      "status": "active",
      "risk_limit": "2%"
    },
    {
      "id": "AC-7766",
      "type": "prop_fund",
      "broker": "FTMO",
      "strategy": "fund_strict_logic",
      "status": "active",
      "risk_limit": "0.5%"
    }
  ]
}
```
