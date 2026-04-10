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
    const adminPass = env.AdminPass || "TradeKiemCom888";

    // =========================================================================
    // 1. ADMIN DASHBOARD & API (v16.4 Global UI)
    // =========================================================================
    if (url.pathname.startsWith('/admin')) {
      // Logic Đăng nhập đơn giản qua URL hoặc Header
      const providedPass = url.searchParams.get('pass') || authHeader?.replace('Bearer ', '');
      if (providedPass !== adminPass && url.pathname !== '/admin/login') {
        return new Response(renderLogin(), { headers: { 'Content-Type': 'text/html' } });
      }

      // GET /admin: Trang chủ Dashboard
      if (url.pathname === '/admin') {
        const list = await env.KV_DEVICES.list();
        const devices = [];
        for (const key of list.keys) {
          if (key.name === "__GLOBAL_CONFIG__") continue;
          const data = await env.KV_DEVICES.get(key.name, { type: "json" });
          devices.push({ id: key.name, ...data });
        }
        const config = await env.KV_DEVICES.get("__GLOBAL_CONFIG__", { type: "json" }) || { version: "1.0", binary_url: "" };
        return new Response(renderDashboard(devices, config), { headers: { 'Content-Type': 'text/html' } });
      }

      // POST /admin/approve: Duyệt máy
      if (url.pathname === '/admin/approve' && request.method === 'POST') {
        const id = url.searchParams.get('id');
        const record = await env.KV_DEVICES.get(id, { type: "json" });
        if (record) {
          record.status = "approved";
          record.auto_update = true; // Mặc định bật khi duyệt
          await env.KV_DEVICES.put(id, JSON.stringify(record));
        }
        return new Response("OK");
      }

      // POST /admin/toggle-update: Bật/Tắt Auto-Update từng máy
      if (url.pathname === '/admin/toggle-update' && request.method === 'POST') {
        const id = url.searchParams.get('id');
        const record = await env.KV_DEVICES.get(id, { type: "json" });
        if (record) {
          record.auto_update = !record.auto_update;
          await env.KV_DEVICES.put(id, JSON.stringify(record));
        }
        return new Response("OK");
      }

      // POST /admin/save-global: Lưu cấu hình phiên bản
      if (url.pathname === '/admin/save-global' && request.method === 'POST') {
        const data = await request.json();
        await env.KV_DEVICES.put("__GLOBAL_CONFIG__", JSON.stringify(data));
        return new Response("OK");
      }
    }

    // =========================================================================
    // 2. CLIENT SYNC API (v16.4 Gateway)
    // =========================================================================
    if (url.pathname === '/v1/sync') {
      const deviceId = url.searchParams.get('id');
      const deviceToken = url.searchParams.get('token');
      const coreType = url.searchParams.get('core');

      if (!deviceId || coreType !== 'zeroclaw') return new Response("Invalid", { status: 400 });

      let deviceRecord = await env.KV_DEVICES.get(deviceId, { type: "json" });
      if (!deviceRecord) {
        await env.KV_DEVICES.put(deviceId, JSON.stringify({
          status: "pending_approval",
          token: deviceToken,
          auto_update: true,
          lastSeen: Date.now()
        }));
        return new Response(JSON.stringify({ ota_status: "pending_approval" }));
      }

      deviceRecord.lastSeen = Date.now();
      await env.KV_DEVICES.put(deviceId, JSON.stringify(deviceRecord));

      if (deviceRecord.status !== "approved") {
        return new Response(JSON.stringify({ ota_status: "pending_approval" }));
      }

      // Lấy cấu hình toàn cầu
      const globalConfig = await env.KV_DEVICES.get("__GLOBAL_CONFIG__", { type: "json" }) || { version: "1.0", binary_url: "" };
      
      const responsePayload = {
        version: globalConfig.version,
        binary_url: deviceRecord.auto_update ? globalConfig.binary_url : "", // Chặn update nếu máy bị tắt Auto-Update
        encrypted_toml: await encryptAES_OpenSSL("auto_approve = true\n[server]\nport = 42617", env.ENCRYPTION_KEY || deviceToken),
        hot_scripts: ["echo '[v16.4] Hệ thống đã sẵn sàng.'"],
        ota_status: "active"
      };

      return new Response(JSON.stringify(responsePayload, null, 2), { headers: { 'Content-Type': 'application/json' } });
    }

    return new Response("ZeroClaw OTA Gateway v16.4 is active.");
  },
};

// =========================================================================
// UI TEMPLATES: Cyberpunk Admin Dashboard
// =========================================================================

function renderLogin() {
  return `<!DOCTYPE html><html><head><title>ZeroClaw Login</title><style>
    body { background: #0a0a0a; color: #00ff41; font-family: 'Courier New', monospace; display: flex; justify-content: center; align-items: center; height: 100vh; }
    .box { border: 1px solid #00ff41; padding: 40px; box-shadow: 0 0 20px rgba(0,255,65,0.2); text-align: center; }
    input { background: #000; border: 1px solid #00ff41; color: #00ff41; padding: 10px; margin: 10px 0; outline: none; }
    button { background: #00ff41; color: #000; border: none; padding: 10px 20px; cursor: pointer; font-weight: bold; }
  </style></head><body><div class="box"><h2>ACCESS DENIED</h2><p>Please provide AdminPass</p>
  <input type="password" id="p" placeholder="Password"><br><button onclick="location.href='?pass='+document.getElementById('p').value">DECRYPT ACCESS</button>
  </div></body></html>`;
}

function renderDashboard(devices, config) {
  const rows = devices.map(d => `
    <tr>
      <td>${d.id}</td>
      <td><span class="status ${d.status}">${d.status.toUpperCase()}</span></td>
      <td>
        <label class="switch">
          <input type="checkbox" ${d.auto_update ? 'checked' : ''} onclick="fetch('/admin/toggle-update?id=${d.id}&pass='+new URLSearchParams(window.location.search).get('pass'), {method:'POST'})">
          <span class="slider"></span>
        </label>
      </td>
      <td>${new Date(d.lastSeen).toLocaleString()}</td>
      <td>
        ${d.status === 'pending_approval' ? `<button onclick="approve('${d.id}')">APPROVE</button>` : ''}
      </td>
    </tr>
  `).join('');

  return `<!DOCTYPE html><html><head><title>ZeroClaw Dashboard v16.4</title><style>
    body { background: #050505; color: #00ff41; font-family: 'Segoe UI', Tahoma, sans-serif; padding: 40px; }
    h1 { color: #00ff41; text-shadow: 0 0 10px #00ff41; border-bottom: 2px solid #00ff41; padding-bottom: 20px; }
    .card { background: #111; border: 1px solid #333; padding: 20px; margin-bottom: 30px; border-radius: 8px; }
    table { width: 100%; border-collapse: collapse; margin-top: 20px; }
    th { text-align: left; border-bottom: 1px solid #333; padding: 15px; color: #888; }
    td { padding: 15px; border-bottom: 1px solid #222; }
    .status.approved { color: #00ff41; font-weight: bold; }
    .status.pending_approval { color: #ffab00; font-style: italic; }
    input[type=text] { background: #000; border: 1px solid #333; color: #00ff41; padding: 8px; border-radius: 4px; }
    button { background: transparent; border: 1px solid #00ff41; color: #00ff41; padding: 5px 15px; border-radius: 4px; cursor: pointer; }
    button:hover { background: #00ff41; color: #000; }
    /* Switch design */
    .switch { position: relative; display: inline-block; width: 40px; height: 20px; }
    .switch input { opacity: 0; width: 0; height: 0; }
    .slider { position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #333; transition: .4s; border-radius: 20px; }
    .slider:before { position: absolute; content: ""; height: 14px; width: 14px; left: 3px; bottom: 3px; background-color: white; transition: .4s; border-radius: 50%; }
    input:checked + .slider { background-color: #00ff41; }
    input:checked + .slider:before { transform: translateX(20px); }
  </style></head><body>
    <h1>SYSTEM COMMAND CENTER : v16.4</h1>
    
    <div class="card">
      <h3>GLOBAL DISTRIBUTION CONFIG</h3>
      Version: <input type="text" id="gv" value="${config.version}">
      Binary URL: <input type="text" id="bu" value="${config.binary_url}" style="width:400px">
      <button onclick="saveGlobal()">SAVE CONFIG</button>
    </div>

    <div class="card">
      <h3>CONNECTED DEVICES</h3>
      <table>
        <thead><tr><th>DEVICE ID</th><th>STATUS</th><th>AUTO-UPDATE</th><th>LAST SEEN</th><th>ACTION</th></tr></thead>
        <tbody>${rows}</tbody>
      </table>
    </div>

    <script>
      const pass = new URLSearchParams(window.location.search).get('pass');
      async function approve(id) {
        await fetch('/admin/approve?id='+id+'&pass='+pass, {method:'POST'});
        location.reload();
      }
      async function saveGlobal() {
        const data = { version: document.getElementById('gv').value, binary_url: document.getElementById('bu').value };
        await fetch('/admin/save-global?pass='+pass, {method:'POST', body: JSON.stringify(data)});
        alert('Global Config Saved');
      }
    </script>
  </body></html>`;
}
