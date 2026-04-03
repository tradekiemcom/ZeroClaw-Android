#!/data/data/com.termux/files/usr/bin/node
# ============================================================
# ZeroClaw-Android — Admin Dashboard (Termux Optimized)
# Zero-Dependency Node.js Server
# ============================================================

const http = require('http');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

const PORT = 7643;
const HOST = '0.0.0.0';

const serveStatic = (res, filePath, contentType) => {
    fs.readFile(filePath, (err, data) => {
        if (err) {
            res.writeHead(500);
            res.end(`Error: ${err.code}`);
        } else {
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(data);
        }
    });
};

const server = http.createServer((req, res) => {
    if (req.url === '/') {
        serveStatic(res, path.join(__dirname, 'public', 'index.html'), 'text/html');
    } else if (req.url === '/api/status') {
        const status = {
            uptime: os.uptime(),
            totalMem: os.totalmem(),
            freeMem: os.freemem(),
            loadAvg: os.loadavg(),
            // Check services
            sshd: !!execSync('pgrep sshd || true').toString().trim(),
            cloudflared: !!execSync('pgrep cloudflared || true').toString().trim(),
        };
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(status));
    } else {
        res.writeHead(404);
        res.end('Not Found');
    }
});

server.listen(PORT, HOST, () => {
    console.log(`🐾 ZeroClaw-Android Dashboard: http://${HOST}:${PORT}`);
});
