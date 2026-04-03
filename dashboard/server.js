// ============================================================
// ZeroClaw Admin Dashboard — Zero-Dependency Node.js Server
// Port: 7643 | Auth: session-based
// ============================================================

const http = require('http');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { execSync, exec } = require('child_process');
const os = require('os');

const PORT = 7643;
const HOST = '0.0.0.0';
const ZEROCLAW_HOME = process.env.HOME + '/ZeroClaw-Android';
const LOG_DIR = process.env.HOME + '/zeroclaw/logs';
const CREDS_FILE = path.join(ZEROCLAW_HOME, 'dashboard', '.credentials.json');
const ALLOWED_ORIGIN = 'https://claw.iz.life';

// ── Credential Management ──────────────────────────────────
function hashPassword(password, salt) {
    salt = salt || crypto.randomBytes(16).toString('hex');
    const hash = crypto.pbkdf2Sync(password, salt, 100000, 64, 'sha512').toString('hex');
    return { salt, hash };
}

function verifyPassword(password, salt, storedHash) {
    const { hash } = hashPassword(password, salt);
    return crypto.timingSafeEqual(Buffer.from(hash, 'hex'), Buffer.from(storedHash, 'hex'));
}

function loadCredentials() {
    try {
        if (fs.existsSync(CREDS_FILE)) {
            return JSON.parse(fs.readFileSync(CREDS_FILE, 'utf8'));
        }
    } catch (e) { /* fall through to default */ }

    // Default credentials: admin / ZeroClaw@2026
    const { salt, hash } = hashPassword('ZeroClaw@2026');
    const creds = { username: 'admin', salt, hash };
    saveCredentials(creds);
    return creds;
}

function saveCredentials(creds) {
    try {
        fs.writeFileSync(CREDS_FILE, JSON.stringify(creds, null, 2), { mode: 0o600 });
    } catch (e) {
        console.error('[CREDS] Failed to save:', e.message);
    }
}

let credentials = loadCredentials();

// ── Session Management ─────────────────────────────────────
const sessions = new Map();
const SESSION_TTL = 24 * 60 * 60 * 1000; // 24 hours

function createSession() {
    const token = crypto.randomUUID();
    sessions.set(token, { created: Date.now(), lastAccess: Date.now() });
    return token;
}

function validateSession(token) {
    if (!token) return false;
    const session = sessions.get(token);
    if (!session) return false;
    if (Date.now() - session.created > SESSION_TTL) {
        sessions.delete(token);
        return false;
    }
    session.lastAccess = Date.now();
    return true;
}

function getSessionToken(req) {
    const cookie = req.headers.cookie || '';
    const match = cookie.match(/zeroclaw_session=([^;]+)/);
    return match ? match[1] : null;
}

// ── CORS & Security Headers ────────────────────────────────
function setCorsHeaders(res, req) {
    const origin = req.headers.origin || '';
    // Allow localhost for direct access and claw.iz.life for tunnel
    if (origin === ALLOWED_ORIGIN || origin.startsWith('http://localhost') || origin.startsWith('http://127.0.0.1')) {
        res.setHeader('Access-Control-Allow-Origin', origin);
    }
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    res.setHeader('Access-Control-Allow-Credentials', 'true');
    // Security headers
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('X-XSS-Protection', '1; mode=block');
    res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
}

// ── Host Header Validation ─────────────────────────────────
function validateHost(req) {
    const host = (req.headers.host || '').split(':')[0];
    const allowed = ['localhost', '127.0.0.1', 'claw.iz.life', '0.0.0.0'];
    return allowed.includes(host);
}

// ── System Info ────────────────────────────────────────────
function getSystemStatus() {
    const uptime = os.uptime();
    const totalMem = os.totalmem();
    const freeMem = os.freemem();
    const usedMem = totalMem - freeMem;
    const memPercent = Math.round((usedMem / totalMem) * 100);
    let cpuLoad = 0;
    try {
        const loadAvg = os.loadavg();
        cpuLoad = Math.round(loadAvg[0] * 100) / 100;
    } catch (e) { /* Termux may not support this */ }

    // Check services
    const services = {};
    try {
        execSync('pgrep -f "cloudflared tunnel"', { timeout: 2000 });
        services.tunnel = 'running';
    } catch (e) { services.tunnel = 'stopped'; }

    try {
        execSync('pgrep -f "sshd"', { timeout: 2000 });
        services.sshd = 'running';
    } catch (e) { services.sshd = 'stopped'; }

    return {
        uptime: formatUptime(uptime),
        uptimeSeconds: uptime,
        memory: {
            total: formatBytes(totalMem),
            used: formatBytes(usedMem),
            free: formatBytes(freeMem),
            percent: memPercent
        },
        cpu: { load: cpuLoad, cores: os.cpus().length },
        hostname: os.hostname(),
        platform: `${os.type()} ${os.release()}`,
        arch: os.arch(),
        nodeVersion: process.version,
        services,
        dashboardPort: PORT,
        timestamp: new Date().toISOString()
    };
}

function formatUptime(seconds) {
    const d = Math.floor(seconds / 86400);
    const h = Math.floor((seconds % 86400) / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const parts = [];
    if (d > 0) parts.push(`${d}d`);
    if (h > 0) parts.push(`${h}h`);
    parts.push(`${m}m`);
    return parts.join(' ');
}

function formatBytes(bytes) {
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    while (bytes >= 1024 && i < units.length - 1) { bytes /= 1024; i++; }
    return `${Math.round(bytes * 10) / 10} ${units[i]}`;
}

// ── Log Reader ────────────────────────────────────────────
function readLogs(logName, lines) {
    lines = lines || 50;
    const logFile = path.join(LOG_DIR, `${logName}.log`);
    try {
        if (!fs.existsSync(logFile)) return { error: 'Log file not found', file: logFile };
        const content = fs.readFileSync(logFile, 'utf8');
        const allLines = content.trim().split('\n');
        return {
            file: logFile,
            totalLines: allLines.length,
            lines: allLines.slice(-lines)
        };
    } catch (e) {
        return { error: e.message };
    }
}

// ── Service Control ────────────────────────────────────────
function controlService(service, action) {
    const commands = {
        tunnel: {
            start: `cd ${ZEROCLAW_HOME} && source tunnel/.env 2>/dev/null && nohup cloudflared tunnel --no-autoupdate run --token "$TUNNEL_TOKEN" >> ${LOG_DIR}/tunnel.log 2>&1 &`,
            stop: 'pkill -f "cloudflared tunnel"',
            restart: 'pkill -f "cloudflared tunnel"; sleep 1; ' +
                `cd ${ZEROCLAW_HOME} && source tunnel/.env 2>/dev/null && nohup cloudflared tunnel --no-autoupdate run --token "$TUNNEL_TOKEN" >> ${LOG_DIR}/tunnel.log 2>&1 &`
        },
        sshd: {
            start: 'sshd',
            stop: 'pkill sshd',
            restart: 'pkill sshd; sleep 1; sshd'
        }
    };

    if (!commands[service]) return { error: `Unknown service: ${service}` };
    if (!commands[service][action]) return { error: `Unknown action: ${action}` };

    try {
        exec(commands[service][action], { shell: '/bin/bash', timeout: 10000 });
        return { success: true, service, action, message: `${service} ${action} initiated` };
    } catch (e) {
        return { error: e.message };
    }
}

// ── Request Body Parser ────────────────────────────────────
function parseBody(req) {
    return new Promise((resolve) => {
        let body = '';
        req.on('data', chunk => {
            body += chunk;
            if (body.length > 1e6) { req.destroy(); resolve({}); }
        });
        req.on('end', () => {
            try { resolve(JSON.parse(body)); }
            catch (e) { resolve({}); }
        });
    });
}

// ── Static File Server ─────────────────────────────────────
const MIME_TYPES = {
    '.html': 'text/html',
    '.css': 'text/css',
    '.js': 'application/javascript',
    '.json': 'application/json',
    '.png': 'image/png',
    '.svg': 'image/svg+xml',
    '.ico': 'image/x-icon'
};

function serveStatic(res, filePath) {
    const ext = path.extname(filePath);
    const mime = MIME_TYPES[ext] || 'application/octet-stream';

    try {
        const content = fs.readFileSync(filePath);
        res.writeHead(200, { 'Content-Type': mime });
        res.end(content);
    } catch (e) {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Not Found');
    }
}

// ── HTTP Server ────────────────────────────────────────────
const server = http.createServer(async (req, res) => {
    // Host header validation
    if (!validateHost(req)) {
        res.writeHead(403, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Invalid Host header' }));
        return;
    }

    setCorsHeaders(res, req);

    // Handle CORS preflight
    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    const url = new URL(req.url, `http://${req.headers.host}`);
    const pathname = url.pathname;

    // ── API Routes ──────────────────────────────────────
    if (pathname === '/api/login' && req.method === 'POST') {
        const body = await parseBody(req);
        if (body.username === credentials.username &&
            verifyPassword(body.password, credentials.salt, credentials.hash)) {
            const token = createSession();
            res.writeHead(200, {
                'Content-Type': 'application/json',
                'Set-Cookie': `zeroclaw_session=${token}; Path=/; HttpOnly; SameSite=Strict; Max-Age=86400`
            });
            res.end(JSON.stringify({ success: true }));
        } else {
            res.writeHead(401, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Invalid credentials' }));
        }
        return;
    }

    if (pathname === '/api/logout' && req.method === 'POST') {
        const token = getSessionToken(req);
        if (token) sessions.delete(token);
        res.writeHead(200, {
            'Content-Type': 'application/json',
            'Set-Cookie': 'zeroclaw_session=; Path=/; HttpOnly; Max-Age=0'
        });
        res.end(JSON.stringify({ success: true }));
        return;
    }

    // ── Protected API routes below ──────────────────────
    if (pathname.startsWith('/api/')) {
        const token = getSessionToken(req);
        if (!validateSession(token)) {
            res.writeHead(401, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Unauthorized' }));
            return;
        }

        if (pathname === '/api/status' && req.method === 'GET') {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify(getSystemStatus()));
            return;
        }

        if (pathname === '/api/logs' && req.method === 'GET') {
            const logName = url.searchParams.get('name') || 'boot';
            const lines = parseInt(url.searchParams.get('lines') || '50');
            // Sanitize logName to prevent path traversal
            const safeName = logName.replace(/[^a-zA-Z0-9_-]/g, '');
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify(readLogs(safeName, lines)));
            return;
        }

        if (pathname === '/api/service' && req.method === 'POST') {
            const body = await parseBody(req);
            const result = controlService(body.service, body.action);
            const status = result.error ? 500 : 200;
            res.writeHead(status, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify(result));
            return;
        }

        if (pathname === '/api/change-password' && req.method === 'POST') {
            const body = await parseBody(req);
            if (!body.currentPassword || !body.newPassword) {
                res.writeHead(400, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'currentPassword and newPassword required' }));
                return;
            }
            if (!verifyPassword(body.currentPassword, credentials.salt, credentials.hash)) {
                res.writeHead(401, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Current password is incorrect' }));
                return;
            }
            if (body.newPassword.length < 8) {
                res.writeHead(400, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Password must be at least 8 characters' }));
                return;
            }
            const { salt, hash } = hashPassword(body.newPassword);
            credentials = { username: credentials.username, salt, hash };
            saveCredentials(credentials);
            // Invalidate all sessions (force re-login)
            sessions.clear();
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ success: true, message: 'Password changed. Please login again.' }));
            return;
        }

        if (pathname === '/api/auth-check' && req.method === 'GET') {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ authenticated: true }));
            return;
        }

        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Not found' }));
        return;
    }

    // ── Static Files ────────────────────────────────────
    const publicDir = path.join(__dirname, 'public');
    let filePath = pathname === '/' ? '/index.html' : pathname;
    // Prevent path traversal
    filePath = path.normalize(filePath).replace(/^(\.\.[\/\\])+/, '');
    serveStatic(res, path.join(publicDir, filePath));
});

server.listen(PORT, HOST, () => {
    console.log(`[ZeroClaw Dashboard] Running on http://${HOST}:${PORT}`);
    console.log(`[ZeroClaw Dashboard] Tunnel endpoint: https://claw.iz.life`);
    console.log(`[ZeroClaw Dashboard] Default login: admin / ZeroClaw@2026`);
});
