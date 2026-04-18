# AI Agent Technical Documentation

Tài liệu này chi tiết về cách thức hoạt động của hệ thống AI Agent cục bộ trong module `ctrader`.

## 1. Lựa chọn Model

Hệ thống sử dụng **Qwen 2.5 - 1.5B Instruct** phiên bản **Quantized GGUF (Q4_K_M)**.
- **Lý do**: Cân bằng hoàn hảo giữa khả năng hiểu ý định giao dịch và tài nguyên phần cứng (RAM/CPU).
- **Quantization**: Giúp giảm kích thước model từ ~3GB xuống còn ~1GB mà vẫn duy trì độ chính xác cao.

## 2. Kiến trúc Hệ thống

Hệ thống AI được xây dựng dựa trên thư viện **Candle (Rust-native ML framework)**:
- **Auto-Download**: Sử dụng `hf-hub` để tải model trực tiếp từ Hugging Face.
- **Inference Engine**: Xử lý suy luận trên CPU, tối ưu hóa bằng tập lệnh **SIMD/NEON** cho chip ARM.
- **Safety**: Mỗi lần suy luận AI được bọc trong `tokio::task::spawn_blocking` để không gây treo hệ thống async của Bot.

## 3. Cấu trúc Prompt (System Prompt)

AI được cấu hình với một System Prompt cố định để đảm bảo đầu ra luôn là JSON:

```text
Bạn là bộ não điều hành của module cTrader trong hệ sinh thái ZeroClaw.
Nhiệm vụ của bạn là nhận tin nhắn từ chủ thể (Hưng), phân tích ý định và trả về DUY NHẤT một mã JSON.
...
```

## 4. Context Awareness (Nhận diện ngữ cảnh)

Trước khi gửi yêu cầu tới LLM, hệ thống tự động chèn **State Snapshot** vào prompt:
- Số dư tài khoản thực (Total Real Balance).
- Vốn thực (Total Real Equity).
- Các vị thế đang mở (Current Open Positions).

Điều này giúp AI có "tầm nhìn" để đưa ra lời khuyên hợp lý (ví dụ: "Tài khoản của bạn đang có 3 lệnh XAUUSD, tôi khuyên bạn nên đóng bớt trước khi mở lệnh mới").

## 5. Quản lý Bộ nhớ

Vì model chiếm khoảng 1GB RAM, hệ thống hỗ trợ cơ chế nạp/xả thủ công:
- `AgentOn`: Load model weights vào RAM.
- `AgentOff`: Thả model, giải phóng toàn bộ RAM cho các tác vụ khác.

---
*Tài liệu kỹ thuật iZTrade AI.*
