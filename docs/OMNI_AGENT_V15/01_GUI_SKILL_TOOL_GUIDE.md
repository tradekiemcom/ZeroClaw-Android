# 🖥️ Master Manual 01: Vận Hành Qua Giao Diện Web (GUI)

Tài liệu này hướng dẫn bạn cách thiết lập **Tool**, **Skill** và **Workflow** trực tiếp trên Dashboard của ZeroClaw mà không cần can thiệp sâu vào dòng lệnh Terminal.

---

## 🔐 1. Đăng Nhập Hệ Thống (Login)

1. **Khởi chạy Gateway**: Đảm bảo lệnh `zeroclaw gateway` đang chạy trên Termux (Port 42617).
2. **Truy cập**: Mở trình duyệt và gõ địa chỉ `http://localhost:42617` (hoặc domain Tunnel của bạn).
3. **Mã xác thực**: Copy mã **Auth Code** hiện ở log Termux dán vào trình duyệt để đăng nhập.

---

## 🛠️ 2. Cách Tạo Công Cụ (Add New Tool)

1. Tại Dashboard, tìm tab **"Tools"** (Công cụ).
2. Bấm nút **"Create New Tool"**.
3. **Khai báo thông tin**:
   - **Name**: Tên công cụ (Ví dụ: `check_gold_price`).
   - **Command**: Lệnh thực thi (Ví dụ: `node ~/.zeroclaw/skills/trading_bridge.js --get-price`).
   - **Description**: Mô tả tác dụng để Agent biết khi nào cần dùng.
4. Bấm **"Save"**. Công cụ này sẽ ngay lập tức xuất hiện trong kho vũ khí của Agent.

---

## 🎒 3. Cách Tạo Kỹ Năng (Add New Skill)

1. Chuyển sang tab **"Skills"**.
2. Bấm **"Add Skill"**.
3. **Nhóm công cụ**: Tích chọn các **Tools** bạn vừa tạo liên quan đến kỹ năng này (Ví dụ: Chọn tool lấy giá và tool đẩy lệnh để tạo Skill "Giao dịch").
4. **Hệ thống hóa**: Viết một đoạn **System Prompt** ngắn cho Skill này để Agent biết "vai diễn" của nó khi dùng bộ công cụ này.
5. Bấm **"Activate"**.

---

## 🔄 4. Thiết Lập Luồng Công Việc (Workflow)

Trong ZeroClaw, **Workflow** được điều phối chủ yếu qua **Master System Prompt** tại trang cấu hình chính (Settings):

1. Đi tới **Settings** -> **Agent Configuration**.
2. Tại đây, bạn viết kịch bản chuỗi cho Agent (The Orchestration Prompt).
   *Ví dụ: "Khi nhận được yêu cầu về thị trường, hãy thực hiện theo chuỗi: 1. Dùng tool lấy tin tức -> 2. Dùng tool phân tích tin -> 3. Báo cáo kết quả."*
3. Agent sẽ tự động nhận diện và xâu chuỗi các Tool/Skill bạn đã cài đặt.

---

## 🏁 Kết luận
Giao diện GUI giúp bạn quản lý "Siêu Trợ Lý" một cách trực quan và nhanh chóng. Mọi thay đổi trên Web sẽ tự động được đồng bộ vào file `.toml` trên thiết bị Note 10+.
