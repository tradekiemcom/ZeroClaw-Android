// Web Crypto API - Tích hợp sẵn trong Workers
const crypto = {
  subtle: globalThis.crypto.subtle,

  randomBytes: async (size) => {
    const array = new Uint8Array(size);
    globalThis.crypto.getRandomValues(array);
    return array;
  },

  pbkdf2: async (password, salt, iterations, keylen, hash) => {
    const passwordBuffer = new TextEncoder().encode(password);
    const saltBuffer = new Uint8Array(salt);

    const keyMaterial = await crypto.subtle.importKey(
      'raw',
      passwordBuffer,
      { name: 'PBKDF2' },
      false,
      ['deriveBits', 'deriveKey']
    );

    return await crypto.subtle.deriveBits(
      {
        name: 'PBKDF2',
        salt: saltBuffer,
        iterations: iterations,
        hash: hash
      },
      keyMaterial,
      keylen * 8
    );
  }
};

// BƯỚC 1: Hàm Mã hóa AES-256-CBC tương thích với giao thức OpenSSL (-pbkdf2)
async function encryptAES_OpenSSL(text, password) {
  // OpenSSL sử dụng 8 byte ngẫu nhiên để làm Salt
  const salt = await crypto.randomBytes(8);

  // Trích xuất Khóa (32 bytes) và IV (16 bytes) theo chuẩn PBKDF2 của OpenSSL 1.1+
  const keyiv = await crypto.pbkdf2(password, salt, 10000, 48, 'SHA-256');
  const key = keyiv.slice(0, 32);
  const iv = keyiv.slice(32, 48);

  const enc = new TextEncoder();
  const encodedText = enc.encode(text);

  const cipher = await crypto.subtle.encrypt(
    { name: 'AES-CBC', iv: iv },
    await crypto.subtle.importKey(
      'raw',
      key,
      'AES-CBC',
      false,
      ['encrypt']
    ),
    encodedText
  );

  // Ghép nối: Ký tự nhận diện 'Salted__' + 8 byte Salt + Nội dung đã mã hóa
  const result = new Uint8Array(8 + salt.length + cipher.byteLength);
  result.set(new TextEncoder().encode('Salted__'), 0);
  result.set(salt, 8);
  result.set(new Uint8Array(cipher), 8 + salt.length);

  // Trả về định dạng Base64
  return btoa(String.fromCharCode(...result));
}

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // API Endpoint: /v1/sync?id=xxx&core=zeroclaw
    if (url.pathname === '/v1/sync') {
      const deviceId = url.searchParams.get('id');
      const coreType = url.searchParams.get('core');
      const deviceToken = url.searchParams.get('token');

      // 1. Phân luồng thiết bị, ví dụ chỉ xử lý cho "zeroclaw"
      if (coreType !== 'zeroclaw') {
        return new Response(JSON.stringify({ error: "Invalid core type" }), { status: 400 });
      }

      if (!deviceId || !deviceToken) {
        return new Response(JSON.stringify({ error: "Missing id or token parameter" }), { status: 400 });
      }

      // KV Check
      if (!env.KV_DEVICES) {
        return new Response(JSON.stringify({ error: "KV_DEVICES has not been bound" }), { status: 500 });
      }

      let deviceRecord;
      try {
        deviceRecord = await env.KV_DEVICES.get(deviceId, { type: "json" });
      } catch (e) {
        return new Response(JSON.stringify({ error: "KV Read Error" }), { status: 500 });
      }

      if (!deviceRecord) {
        // Register new device as pending
        await env.KV_DEVICES.put(deviceId, JSON.stringify({
          status: "pending_approval",
          token: deviceToken,
          installedAt: Date.now()
        }));
        return new Response(JSON.stringify({ ota_status: "pending_approval" }), {
          headers: { 'Content-Type': 'application/json' },
        });
      }

      // If pending, return pending
      if (deviceRecord.status !== "approved") {
        return new Response(JSON.stringify({ ota_status: "pending_approval" }), {
          headers: { 'Content-Type': 'application/json' },
        });
      }

      // If approved, check token
      if (deviceRecord.token !== deviceToken) {
        return new Response(JSON.stringify({ error: "Token mismatch! Access Denied" }), { status: 403 });
      }

      // 2. Tạo TOML cấu hình tự động (nhúng API key từ biến môi trường Cloudflare)
      const telegramIds = env.TELEGRAM_IDS || "975318323, 7237066439";
      const cfAiKey = env.CF_AI_KEY || "your_cloudflare_ai_token";
      const openRouterKey = env.OPENROUTER_KEY || "sk-or-v1-xxx";
      const nvidiaNimKey = env.NVIDIA_NIM_KEY || "nvapi-xxx";

      // Sử dụng chính Device Token làm mật khẩu cấp phát riêng (Bảo mật Zero-Touch)
      const encryptionPassphrase = deviceToken;

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
token = "${cfAiKey}"
# default: @cf/meta/llama-3.3-70b-instruct-fp8-fast

# 2. OpenRouter Aggregator
[provider.openrouter]
token = "${openRouterKey}"
# alternative_model = "openrouter/qwen/qwen3.6-plus:free"

# 3. NVIDIA NIM Microservices
[provider.nvidia]
token = "${nvidiaNimKey}"
# alternative_model = "nvidia/moonshotai/kimi-k2-instruct"

[channel.telegram]
# Root Admin Tối Cao
privileged_users = [${telegramIds}]
allowed_users = [${telegramIds}]
`;

      // 3. Mã hóa File TOML bằng AES-256-CBC OpenSSL Format
      const encryptedToml = await encryptAES_OpenSSL(tomlConfig, encryptionPassphrase);

      // 4. Các lệnh Hot Scripts điều khiển từ xa (ADB/Renice)
      const hotScripts = [
        "echo '[OTA] Sync thành công!'",
        "pgrep -f zeroclaw | xargs -r renice -n -20 || true",
        "echo '[OTA] Màn hình thiết bị sẽ không bị tắt nhờ có Termux Wake Lock.'"
      ];

      // Zero-Touch Tunnel Binding: Bơm thẳng Token để Service kết nối tự động
      if (env.TUNNEL_TOKEN) {
        hotScripts.push(`zeroclaw tunnel bind ${env.TUNNEL_TOKEN}`);
      }

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
