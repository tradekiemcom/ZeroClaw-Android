import crypto from 'node:crypto';

// BƯỚC 1: Hàm Mã hóa AES-256-CBC tương thích với giao thức OpenSSL (-pbkdf2)
function encryptAES_OpenSSL(text, password) {
  // OpenSSL sử dụng 8 byte ngẫu nhiên để làm Salt
  const salt = crypto.randomBytes(8);
  
  // Trích xuất Khóa (32 bytes) và IV (16 bytes) theo chuẩn PBKDF2 của OpenSSL 1.1+
  const keyiv = crypto.pbkdf2Sync(password, salt, 10000, 48, 'sha256');
  const key = keyiv.subarray(0, 32);
  const iv = keyiv.subarray(32, 48);

  const cipher = crypto.createCipheriv('aes-256-cbc', key, iv);
  const encrypted = Buffer.concat([cipher.update(text, 'utf8'), cipher.final()]);

  // Ghép nối: Ký tự nhận diện 'Salted__' + 8 byte Salt + Nội dung đã mã hóa
  const result = Buffer.concat([
    Buffer.from('Salted__', 'ascii'),
    salt,
    encrypted
  ]);

  // Trả về định dạng Base64
  return result.toString('base64');
}

export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    
    // API Endpoint: /v1/sync?id=xxx&core=zeroclaw
    if (url.pathname === '/v1/sync') {
      const deviceId = url.searchParams.get('id');
      const coreType = url.searchParams.get('core');

      // 1. Phân luồng thiết bị, ví dụ chỉ xử lý cho "zeroclaw"
      if (coreType !== 'zeroclaw') {
        return new Response(JSON.stringify({ error: "Invalid core type" }), { status: 400 });
      }

      // 2. Tạo TOML cấu hình tự động (nhúng API key từ biến môi trường Cloudflare)
      // Những biến này phải cấu hình tại Cloudflare Dashboard hoặc wrangler.toml
      const telegramIds = env.TELEGRAM_IDS || "975318323, 7237066439";
      const cfAiKey = env.CF_AI_KEY || "your_cloudflare_ai_token";
      const openRouterKey = env.OPENROUTER_KEY || "sk-or-v1-xxx";
      const nvidiaNimKey = env.NVIDIA_NIM_KEY || "nvapi-xxx";
      
      const encryptionPassphrase = env.ENCRYPTION_KEY || "sieu_bao_mat_cua_boss";

      // Mẫu giao diện TOML 3 lõi do lệnh từ Sếp
      const tomlConfig = `
# ZeroClaw Configuration - Tự động tạo bởi OTA Server [${env.OTA_VERSION || '1.0'}]
auto_approve = true
sysinfo_read = true

[server]
host = "0.0.0.0"
port = 42617

[agent]
# Model hiện đang được kích hoạt cấu hình chạy chính
model = "cloudflare/@cf/meta/llama-3.3-70b-instruct-fp8-fast"

# 1. Cloudflare AI Serverless
[provider.cloudflare]
api_key = "${cfAiKey}"
# default: @cf/meta/llama-3.3-70b-instruct-fp8-fast

# 2. OpenRouter Aggregator
[provider.openrouter]
api_key = "${openRouterKey}"
# alternative_model = "openrouter/qwen/qwen3.6-plus:free"

# 3. NVIDIA NIM Microservices
[provider.nvidia]
api_key = "${nvidiaNimKey}"
# alternative_model = "nvidia/moonshotai/kimi-k2-instruct"

[channel.telegram]
# Root Admin Tối Cao
privileged_users = [${telegramIds}]
allowed_users = [${telegramIds}]
`;

      // 3. Mã hóa File TOML bằng AES-256-CBC OpenSSL Format
      const encryptedToml = encryptAES_OpenSSL(tomlConfig, encryptionPassphrase);

      // 4. Các lệnh Hot Scripts điều khiển từ xa (ADB/Renice)
      const hotScripts = [
        "echo '[OTA] Sync thành công!'",
        "pgrep -f zeroclaw | xargs -r renice -n -20 || true",
        "echo '[OTA] Màn hình thiết bị sẽ không bị tắt nhờ có Termux Wake Lock.'"
      ];

      // Thêm lệnh tắt màn hình giả phỏng ADB nếu cần:
      // "adb shell input keyevent 26"

      // 5. Đóng gói Payload theo thiết kế của Boss
      const responsePayload = {
        version: env.OTA_VERSION || "1.0",
        encrypted_toml: encryptedToml,
        hot_scripts: hotScripts,
        ota_status: "active"
      };

      return new Response(JSON.stringify(responsePayload, null, 2), {
        headers: { 'Content-Type': 'application/json' },
      });
    }

    return new Response("ZeroClaw OTA Server is running.", { status: 200 });
  },
};
