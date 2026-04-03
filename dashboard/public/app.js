// ============================================================
// ZeroClaw Dashboard — Frontend Logic
// ============================================================

(function () {
    'use strict';

    // ── State ──────────────────────────────────────────────
    let pollInterval = null;
    let isAuthenticated = false;

    // ── Elements ───────────────────────────────────────────
    const $ = (sel) => document.querySelector(sel);
    const loginScreen = $('#login-screen');
    const dashboardScreen = $('#dashboard-screen');
    const loginForm = $('#login-form');
    const loginError = $('#login-error');
    const loginBtn = $('#login-btn');
    const logoutBtn = $('#logout-btn');
    const settingsBtn = $('#settings-btn');
    const settingsModal = $('#settings-modal');
    const passwordForm = $('#password-form');
    const connectionBadge = $('#connection-badge');

    // ── API Helpers ────────────────────────────────────────
    async function api(endpoint, options) {
        options = options || {};
        try {
            const res = await fetch(`/api/${endpoint}`, {
                method: options.method || 'GET',
                headers: options.body ? { 'Content-Type': 'application/json' } : {},
                body: options.body ? JSON.stringify(options.body) : undefined,
                credentials: 'same-origin'
            });
            const data = await res.json();
            if (!res.ok) throw new Error(data.error || `HTTP ${res.status}`);
            return data;
        } catch (err) {
            if (err.message === 'Unauthorized') {
                showLogin();
            }
            throw err;
        }
    }

    // ── Auth ───────────────────────────────────────────────
    async function checkAuth() {
        try {
            await api('auth-check');
            showDashboard();
        } catch (e) {
            showLogin();
        }
    }

    function showLogin() {
        isAuthenticated = false;
        loginScreen.classList.add('active');
        dashboardScreen.classList.remove('active');
        stopPolling();
    }

    function showDashboard() {
        isAuthenticated = true;
        loginScreen.classList.remove('active');
        dashboardScreen.classList.add('active');
        startPolling();
        refreshStatus();
    }

    loginForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        loginError.classList.add('hidden');
        loginBtn.querySelector('.btn-text').classList.add('hidden');
        loginBtn.querySelector('.btn-loader').classList.remove('hidden');
        loginBtn.disabled = true;

        try {
            await api('login', {
                method: 'POST',
                body: {
                    username: $('#username').value,
                    password: $('#password').value
                }
            });
            $('#password').value = '';
            showDashboard();
        } catch (err) {
            loginError.textContent = err.message || 'Login failed';
            loginError.classList.remove('hidden');
            shakeElement(loginForm);
        } finally {
            loginBtn.querySelector('.btn-text').classList.remove('hidden');
            loginBtn.querySelector('.btn-loader').classList.add('hidden');
            loginBtn.disabled = false;
        }
    });

    logoutBtn.addEventListener('click', async () => {
        try { await api('logout', { method: 'POST' }); } catch (e) { /* ignore */ }
        showLogin();
    });

    // ── Shake Animation ────────────────────────────────────
    function shakeElement(el) {
        el.style.animation = 'none';
        el.offsetHeight; // reflow
        el.style.animation = 'shake 0.4s ease';
        setTimeout(() => { el.style.animation = ''; }, 400);
    }

    // Add shake keyframes dynamically
    const shakeStyle = document.createElement('style');
    shakeStyle.textContent = `
        @keyframes shake {
            0%, 100% { transform: translateX(0); }
            20% { transform: translateX(-8px); }
            40% { transform: translateX(8px); }
            60% { transform: translateX(-4px); }
            80% { transform: translateX(4px); }
        }
    `;
    document.head.appendChild(shakeStyle);

    // ── Status Polling ─────────────────────────────────────
    function startPolling() {
        stopPolling();
        pollInterval = setInterval(refreshStatus, 5000);
    }

    function stopPolling() {
        if (pollInterval) {
            clearInterval(pollInterval);
            pollInterval = null;
        }
    }

    async function refreshStatus() {
        try {
            const status = await api('status');
            updateStatusCards(status);
            updateServices(status.services);
            connectionBadge.textContent = '● Connected';
            connectionBadge.classList.remove('disconnected');
        } catch (err) {
            connectionBadge.textContent = '● Disconnected';
            connectionBadge.classList.add('disconnected');
        }
    }

    function updateStatusCards(status) {
        $('#stat-uptime').textContent = status.uptime;
        $('#stat-memory').textContent = `${status.memory.used} / ${status.memory.total}`;
        $('#memory-bar').style.width = `${status.memory.percent}%`;
        // Color the memory bar based on usage
        if (status.memory.percent > 85) {
            $('#memory-bar').style.background = 'linear-gradient(90deg, var(--danger), #ef4444)';
        } else if (status.memory.percent > 60) {
            $('#memory-bar').style.background = 'linear-gradient(90deg, var(--warning), #f59e0b)';
        } else {
            $('#memory-bar').style.background = 'linear-gradient(90deg, var(--accent), var(--success))';
        }
        $('#stat-cpu').textContent = `${status.cpu.load} (${status.cpu.cores} cores)`;
        $('#stat-platform').textContent = `${status.platform} ${status.arch}`;
    }

    function updateServices(services) {
        Object.entries(services).forEach(([name, status]) => {
            const indicator = $(`#svc-${name}-indicator`);
            const statusEl = $(`#svc-${name}-status`);
            if (indicator) {
                indicator.className = `service-indicator ${status}`;
            }
            if (statusEl) {
                statusEl.textContent = status;
            }
        });
    }

    // ── Service Control ────────────────────────────────────
    window.serviceAction = async function (service, action) {
        try {
            const result = await api('service', {
                method: 'POST',
                body: { service, action }
            });
            // Brief delay then refresh
            setTimeout(refreshStatus, 1500);
        } catch (err) {
            alert(`Failed: ${err.message}`);
        }
    };

    // ── Logs ───────────────────────────────────────────────
    window.loadLogs = async function () {
        const logName = $('#log-select').value;
        const logContent = $('#log-content');
        logContent.textContent = 'Loading...';

        try {
            const data = await api(`logs?name=${logName}&lines=100`);
            if (data.error) {
                logContent.textContent = `Error: ${data.error}\nFile: ${data.file || 'unknown'}`;
            } else {
                logContent.textContent = data.lines.join('\n') || 'Empty log file';
                // Auto-scroll to bottom
                const viewer = $('#log-viewer');
                viewer.scrollTop = viewer.scrollHeight;
            }
        } catch (err) {
            logContent.textContent = `Failed to load logs: ${err.message}`;
        }
    };

    // ── Settings Modal ─────────────────────────────────────
    settingsBtn.addEventListener('click', () => {
        settingsModal.classList.remove('hidden');
    });

    window.closeSettings = function () {
        settingsModal.classList.add('hidden');
        $('#pw-error').classList.add('hidden');
        $('#pw-success').classList.add('hidden');
        passwordForm.reset();
    };

    // Close modal on Escape
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && !settingsModal.classList.contains('hidden')) {
            closeSettings();
        }
    });

    passwordForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const pwError = $('#pw-error');
        const pwSuccess = $('#pw-success');
        pwError.classList.add('hidden');
        pwSuccess.classList.add('hidden');

        const newPw = $('#new-pw').value;
        const confirmPw = $('#confirm-pw').value;

        if (newPw !== confirmPw) {
            pwError.textContent = 'Passwords do not match';
            pwError.classList.remove('hidden');
            return;
        }

        if (newPw.length < 8) {
            pwError.textContent = 'Password must be at least 8 characters';
            pwError.classList.remove('hidden');
            return;
        }

        try {
            const result = await api('change-password', {
                method: 'POST',
                body: {
                    currentPassword: $('#current-pw').value,
                    newPassword: newPw
                }
            });
            pwSuccess.textContent = result.message || 'Password changed successfully!';
            pwSuccess.classList.remove('hidden');
            passwordForm.reset();
            // Auto redirect to login after 2s
            setTimeout(() => {
                closeSettings();
                showLogin();
            }, 2000);
        } catch (err) {
            pwError.textContent = err.message || 'Failed to change password';
            pwError.classList.remove('hidden');
        }
    });

    // ── Init ───────────────────────────────────────────────
    checkAuth();
})();
