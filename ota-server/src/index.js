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
    const authHeader = request.headers.get('Authorization');

    // =========================================================================
    // ADMIN API: Quản trị thiết bị tập trung (Dành cho Sếp)
    // =========================================================================
    if (url.pathname.startsWith('/admin')) {
      const adminToken = env.ADMIN_TOKEN || "TradeKiemCom888"; // Mã bảo mật mặc định
      if (authHeader !== `Bearer ${adminToken}`) {
        return new Response("Unauthorized Admin Access", { status: 401 });
      }

      // GET /admin/list: Liệt kê tất cả thiết bị
      if (url.pathname === '/admin/list') {
        const list = await env.KV_DEVICES.list();
        const devices = {};
        for (const key of list.keys) {
          devices[key.name] = await env.KV_DEVICES.get(key.name, { type: "json" });
        }
        return new Response(JSON.stringify(devices, null, 2), { headers: { 'Content-Type': 'application/json' } });
      }

      // POST /admin/approve: Duyệt thiết bị (Dùng: ?id=xxx)
      if (url.pathname === '/admin/approve' && request.method === 'POST') {
        const id = url.searchParams.get('id');
        if (!id) return new Response("Missing ID", { status: 400 });
        const record = await env.KV_DEVICES.get(id, { type: "json" });
        if (!record) return new Response("Device Not Found", { status: 404 });
        
        record.status = "approved";
        record.approvedAt = Date.now();
        await env.KV_DEVICES.put(id, JSON.stringify(record));
        return new Response(`Device ${id} Approved!`, { status: 200 });
      }
    }

    // =========================================================================
    // CLIENT API: Đồng bộ & Tự động cập nhật (v8.0)
    // =========================================================================
    if (url.pathname === '/v1/sync') {
      if (!env.KV_DEVICES) return new Response("KV Binding Missing", { status: 500 });

      const deviceId = url.searchParams.get('id');
      const coreType = url.searchParams.get('core');
      const deviceToken = url.searchParams.get('token');

      if (coreType !== 'zeroclaw') return new Response("Invalid core", { status: 400 });
      if (!deviceId || !deviceToken) return new Response("Missing params", { status: 400 });

      let deviceRecord = await env.KV_DEVICES.get(deviceId, { type: "json" });

      if (!deviceRecord) {
        // Đăng ký thiết bị mới (Discovery Mode)
        await env.KV_DEVICES.put(deviceId, JSON.stringify({
          status: "pending_approval",
          token: deviceToken,
          registeredAt: Date.now(),
          lastSeen: Date.now()
        }));
        return new Response(JSON.stringify({ ota_status: "pending_approval" }), { headers: { 'Content-Type': 'application/json' } });
      }

      // Cập nhật thời gian kết nối cuối
      deviceRecord.lastSeen = Date.now();
      await env.KV_DEVICES.put(deviceId, JSON.stringify(deviceRecord));

      if (deviceRecord.status !== "approved") {
        return new Response(JSON.stringify({ ota_status: "pending_approval" }), { headers: { 'Content-Type': 'application/json' } });
      }

      const encryptionPassphrase = env.ENCRYPTION_KEY || deviceToken;
      const telegramIds = env.TELEGRAM_IDS || "975318323, 7237066439";

      const tomlConfig = `
# ZeroClaw Auto-Generated Config v${env.OTA_VERSION || '8.0'}
auto_approve = true
[server]
port = 42617
[channel.telegram]
privileged_users = [${telegramIds}]
      `;

      const encryptedToml = await encryptAES_OpenSSL(tomlConfig, encryptionPassphrase);

      const responsePayload = {
        version: env.SOFTWARE_VERSION || "1.0",
        binary_url: env.BINARY_URL || "", // URL tải binary mới nếu cần update
        encrypted_toml: encryptedToml,
        hot_scripts: [
          "echo '[OTA] Hệ thống đã được duyệt và đồng bộ!'"
        ],
        ota_status: "active"
      };

      return new Response(JSON.stringify(responsePayload, null, 2), { headers: { 'Content-Type': 'application/json' } });
    }

    return new Response("ZeroClaw OTA Node v8.0 is active.", { status: 200 });
  },
};
