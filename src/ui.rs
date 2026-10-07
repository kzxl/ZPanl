/// Embedded modern single-page application for the ZPanl dashboard.
pub const INDEX_HTML: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ZPanl ⚡ Sovereign Web Panel</title>
  <style>
    :root {
      --bg: #07090e;
      --surface: #0e131f;
      --surface-elevated: #141c2e;
      --surface-card: rgba(18, 26, 43, 0.7);
      --border: rgba(255, 255, 255, 0.08);
      --border-focus: rgba(6, 182, 212, 0.6);
      --text: #f8fafc;
      --text-muted: #94a3b8;
      --text-dim: #64748b;
      --cyan: #06b6d4;
      --cyan-glow: #22d3ee;
      --blue: #3b82f6;
      --green: #10b981;
      --yellow: #f59e0b;
      --red: #f43f5e;
      --purple: #a855f7;
      --font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Inter, Helvetica, Arial, sans-serif;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: var(--bg);
      background-image: 
        radial-gradient(circle at 15% 10%, rgba(6, 182, 212, 0.05) 0%, transparent 40%),
        radial-gradient(circle at 85% 90%, rgba(59, 130, 246, 0.04) 0%, transparent 40%);
      color: var(--text);
      font-family: var(--font-sans);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      overflow-x: hidden;
      -webkit-font-smoothing: antialiased;
    }

    /* HEADER */
    header {
      background: rgba(14, 19, 31, 0.85);
      backdrop-filter: blur(12px);
      border-bottom: 1px solid var(--border);
      padding: 0.75rem 2rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      position: sticky;
      top: 0;
      z-index: 100;
    }
    .brand {
      display: flex;
      align-items: center;
      gap: 0.85rem;
      text-decoration: none;
      color: inherit;
    }
    .brand-icon {
      width: 32px;
      height: 32px;
      background: linear-gradient(135deg, var(--cyan), var(--blue));
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
      box-shadow: 0 0 15px rgba(6, 182, 212, 0.4);
    }
    .brand-icon svg { width: 18px; height: 18px; fill: #fff; }
    .brand-title {
      font-weight: 800;
      font-size: 1.25rem;
      letter-spacing: -0.03em;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .brand-tag {
      font-size: 0.65rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      background: rgba(6, 182, 212, 0.12);
      color: var(--cyan-glow);
      border: 1px solid rgba(6, 182, 212, 0.3);
      padding: 0.15rem 0.5rem;
      border-radius: 9999px;
    }

    /* NAVIGATION */
    nav {
      display: flex;
      background: rgba(0, 0, 0, 0.3);
      padding: 0.25rem;
      border-radius: 0.5rem;
      border: 1px solid var(--border);
      gap: 0.25rem;
    }
    nav button {
      background: transparent;
      border: none;
      color: var(--text-muted);
      padding: 0.45rem 1rem;
      border-radius: 0.375rem;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 0.45rem;
      transition: all 0.15s ease;
    }
    nav button svg { width: 16px; height: 16px; fill: currentColor; }
    nav button:hover {
      color: var(--text);
      background: rgba(255, 255, 255, 0.04);
    }
    nav button.active {
      color: var(--cyan-glow);
      background: rgba(6, 182, 212, 0.15);
      box-shadow: 0 0 10px rgba(6, 182, 212, 0.15);
    }

    /* HEADER STATUS */
    .header-status {
      display: flex;
      align-items: center;
      gap: 1rem;
      font-size: 0.8rem;
      font-family: var(--font-mono);
      color: var(--text-muted);
    }
    .pulse-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--green);
      box-shadow: 0 0 8px var(--green);
      animation: pulse 2s infinite;
      display: inline-block;
      margin-right: 0.4rem;
    }
    @keyframes pulse {
      0%, 100% { opacity: 1; transform: scale(1); }
      50% { opacity: 0.4; transform: scale(0.85); }
    }

    /* MAIN CONTAINER */
    main {
      flex: 1;
      padding: 2rem;
      max-width: 1440px;
      margin: 0 auto;
      width: 100%;
    }
    .tab-content { display: none; animation: fadeIn 0.2s ease-out; }
    .tab-content.active { display: block; }
    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(4px); }
      to { opacity: 1; transform: translateY(0); }
    }

    /* CARDS & GRIDS */
    .grid-4 {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 1.25rem;
      margin-bottom: 2rem;
    }
    .card {
      background: var(--surface-card);
      backdrop-filter: blur(16px);
      border: 1px solid var(--border);
      border-radius: 0.75rem;
      padding: 1.5rem;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.25);
      position: relative;
      overflow: hidden;
    }
    .card::before {
      content: "";
      position: absolute;
      top: 0; left: 0; right: 0;
      height: 2px;
      background: linear-gradient(90deg, transparent, rgba(6, 182, 212, 0.4), transparent);
      opacity: 0;
      transition: opacity 0.2s;
    }
    .card:hover::before { opacity: 1; }

    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 0.75rem;
    }
    .card-title {
      font-size: 0.8rem;
      text-transform: uppercase;
      letter-spacing: 0.06em;
      color: var(--text-dim);
      font-weight: 700;
    }
    .card-icon {
      color: var(--cyan);
      opacity: 0.8;
    }
    .card-icon svg { width: 20px; height: 20px; fill: currentColor; }
    .card-value {
      font-size: 2rem;
      font-weight: 800;
      font-family: var(--font-mono);
      letter-spacing: -0.03em;
      margin-bottom: 0.5rem;
    }
    .card-subtitle {
      font-size: 0.75rem;
      color: var(--text-muted);
      display: flex;
      justify-content: space-between;
      margin-top: 0.4rem;
    }
    .progress-bar {
      height: 6px;
      background: rgba(255, 255, 255, 0.06);
      border-radius: 9999px;
      overflow: hidden;
      margin-top: 0.75rem;
    }
    .progress-fill {
      height: 100%;
      border-radius: 9999px;
      background: linear-gradient(90deg, var(--cyan), var(--blue));
      transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    }

    /* TOOLBARS */
    .toolbar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 1.5rem;
      gap: 1rem;
      flex-wrap: wrap;
    }
    .toolbar-title {
      font-size: 1.35rem;
      font-weight: 700;
      letter-spacing: -0.02em;
    }
    .toolbar-actions {
      display: flex;
      gap: 0.75rem;
      align-items: center;
    }

    /* BUTTONS */
    .btn {
      background: linear-gradient(135deg, #0284c7, #0369a1);
      color: #fff;
      border: 1px solid rgba(56, 189, 248, 0.3);
      padding: 0.5rem 1rem;
      border-radius: 0.45rem;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      gap: 0.45rem;
      transition: all 0.15s ease;
      box-shadow: 0 2px 8px rgba(2, 132, 199, 0.25);
    }
    .btn svg { width: 15px; height: 15px; fill: currentColor; }
    .btn:hover {
      background: linear-gradient(135deg, #0369a1, #075985);
      border-color: rgba(56, 189, 248, 0.5);
      transform: translateY(-1px);
    }
    .btn-secondary {
      background: var(--surface-elevated);
      color: var(--text);
      border: 1px solid var(--border);
      box-shadow: none;
    }
    .btn-secondary:hover {
      background: rgba(255, 255, 255, 0.08);
      border-color: rgba(255, 255, 255, 0.15);
      transform: translateY(-1px);
    }
    .btn-danger {
      background: rgba(244, 63, 94, 0.12);
      color: var(--red);
      border: 1px solid rgba(244, 63, 94, 0.3);
      box-shadow: none;
    }
    .btn-danger:hover {
      background: var(--red);
      color: #fff;
      transform: translateY(-1px);
    }

    /* TABLES */
    .table-container {
      background: var(--surface-card);
      border: 1px solid var(--border);
      border-radius: 0.75rem;
      overflow: hidden;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
    }
    table {
      width: 100%;
      border-collapse: collapse;
      text-align: left;
      font-size: 0.875rem;
    }
    th {
      background: rgba(0, 0, 0, 0.3);
      color: var(--text-dim);
      padding: 0.85rem 1.25rem;
      font-size: 0.75rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.06em;
      border-bottom: 1px solid var(--border);
    }
    td {
      padding: 0.9rem 1.25rem;
      border-bottom: 1px solid var(--border);
      vertical-align: middle;
    }
    tr:last-child td { border-bottom: none; }
    tr:hover td { background: rgba(255, 255, 255, 0.02); }

    /* BADGES */
    .badge {
      display: inline-flex;
      align-items: center;
      gap: 0.3rem;
      padding: 0.2rem 0.55rem;
      border-radius: 0.3rem;
      font-size: 0.75rem;
      font-weight: 600;
      font-family: var(--font-mono);
    }
    .badge-cyan { background: rgba(6, 182, 212, 0.12); color: var(--cyan-glow); border: 1px solid rgba(6, 182, 212, 0.3); }
    .badge-purple { background: rgba(168, 85, 247, 0.12); color: #c084fc; border: 1px solid rgba(168, 85, 247, 0.3); }
    .badge-blue { background: rgba(59, 130, 246, 0.12); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3); }
    .badge-green { background: rgba(16, 185, 129, 0.12); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
    .badge-red { background: rgba(244, 63, 94, 0.12); color: #fb7185; border: 1px solid rgba(244, 63, 94, 0.3); }
    .badge-yellow { background: rgba(245, 158, 11, 0.12); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }

    /* SEARCH & INPUTS */
    .search-input {
      background: var(--surface);
      border: 1px solid var(--border);
      color: var(--text);
      padding: 0.5rem 0.85rem;
      border-radius: 0.45rem;
      font-size: 0.85rem;
      width: 260px;
      transition: all 0.15s ease;
    }
    .search-input:focus {
      outline: none;
      border-color: var(--cyan);
      box-shadow: 0 0 10px rgba(6, 182, 212, 0.2);
    }

    /* MODAL */
    .modal {
      display: none;
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.75);
      backdrop-filter: blur(8px);
      align-items: center;
      justify-content: center;
      z-index: 1000;
      padding: 1rem;
    }
    .modal.active { display: flex; animation: modalIn 0.15s ease-out; }
    @keyframes modalIn {
      from { opacity: 0; transform: scale(0.96); }
      to { opacity: 1; transform: scale(1); }
    }
    .modal-box {
      background: #0f1626;
      border: 1px solid rgba(255, 255, 255, 0.1);
      border-radius: 0.75rem;
      width: 100%;
      max-width: 600px;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
      overflow: hidden;
    }
    .modal-header {
      padding: 1.25rem 1.5rem;
      border-bottom: 1px solid var(--border);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .modal-header h3 { font-size: 1.15rem; font-weight: 700; }
    .modal-close {
      background: transparent;
      border: none;
      color: var(--text-muted);
      cursor: pointer;
      font-size: 1.25rem;
      line-height: 1;
    }
    .modal-close:hover { color: var(--text); }
    .modal-body { padding: 1.5rem; }
    .modal-footer {
      padding: 1rem 1.5rem;
      background: rgba(0, 0, 0, 0.25);
      border-top: 1px solid var(--border);
      display: flex;
      justify-content: flex-end;
      gap: 0.75rem;
    }

    .form-group { margin-bottom: 1.25rem; }
    .form-group label {
      display: block;
      font-size: 0.78rem;
      font-weight: 600;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.05em;
      margin-bottom: 0.4rem;
    }
    .form-input, .form-select {
      width: 100%;
      background: #07090e;
      border: 1px solid var(--border);
      color: var(--text);
      padding: 0.6rem 0.85rem;
      border-radius: 0.45rem;
      font-size: 0.9rem;
      font-family: inherit;
    }
    .form-input:focus, .form-select:focus {
      outline: none;
      border-color: var(--cyan);
      box-shadow: 0 0 10px rgba(6, 182, 212, 0.2);
    }

    /* CODE EDITOR & PREVIEWS */
    .editor-wrapper {
      position: relative;
      background: #05070d;
      border: 1px solid var(--border);
      border-radius: 0.5rem;
      overflow: hidden;
    }
    .editor-textarea {
      width: 100%;
      height: 480px;
      background: transparent;
      color: #e2e8f0;
      font-family: var(--font-mono);
      padding: 1.25rem;
      border: none;
      font-size: 0.9rem;
      line-height: 1.6;
      resize: vertical;
      tab-size: 4;
      outline: none;
    }
    pre.code-block {
      background: #05070d;
      color: #38bdf8;
      font-family: var(--font-mono);
      padding: 1.25rem;
      border-radius: 0.5rem;
      overflow-x: auto;
      font-size: 0.85rem;
      line-height: 1.6;
      border: 1px solid var(--border);
    }

    /* TOAST NOTIFICATION */
    .toast-container {
      position: fixed;
      bottom: 2rem;
      right: 2rem;
      display: flex;
      flex-direction: column;
      gap: 0.75rem;
      z-index: 9999;
    }
    .toast {
      background: #0f1626;
      border: 1px solid var(--border);
      border-left: 4px solid var(--cyan);
      padding: 0.85rem 1.25rem;
      border-radius: 0.45rem;
      font-size: 0.85rem;
      box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
      animation: toastIn 0.2s ease-out;
      display: flex;
      align-items: center;
      gap: 0.75rem;
      min-width: 280px;
    }
    .toast-success { border-left-color: var(--green); }
    .toast-error { border-left-color: var(--red); }
    @keyframes toastIn {
      from { transform: translateX(50px); opacity: 0; }
      to { transform: translateX(0); opacity: 1; }
    }
  </style>
</head>
<body>
  <!-- HEADER -->
  <header>
    <a href="#" class="brand" onclick="switchTab('overview')">
      <div class="brand-icon">
        <svg viewBox="0 0 24 24"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
      </div>
      <div class="brand-title">
        <span>ZPanl</span>
        <span class="brand-tag">SOVEREIGN LINUX</span>
      </div>
    </a>

    <nav>
      <button class="active" onclick="switchTab('overview')">
        <svg viewBox="0 0 24 24"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>
        Overview
      </button>
      <button onclick="switchTab('sites')">
        <svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
        Websites
      </button>
      <button onclick="switchTab('files')">
        <svg viewBox="0 0 24 24"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg>
        Files
      </button>
      <button onclick="switchTab('services')">
        <svg viewBox="0 0 24 24"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
        Services
      </button>
      <button onclick="switchTab('caddy')">
        <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
        Caddyfile
      </button>
    </nav>

    <div class="header-status">
      <div><span class="pulse-dot"></span><span id="procMode">Linux /proc Engine</span></div>
      <div id="uptimeDisplay">Uptime: 0s</div>
    </div>
  </header>

  <main>
    <!-- TAB: OVERVIEW -->
    <div id="tab-overview" class="tab-content active">
      <div class="grid-4">
        <!-- CPU CARD -->
        <div class="card">
          <div class="card-header">
            <span class="card-title">CPU Utilization</span>
            <span class="card-icon">
              <svg viewBox="0 0 24 24"><path d="M17 17H7V7h10v10zm2-14v2h2v2h-2v2h2v2h-2v2h2v2h-2v2h-2v-2h-2v2h-2v-2h-2v2H7v-2H5v-2H3v-2h2v-2H3v-2h2V9H3V7h2V5h2V3h2v2h2V3h2v2h2V3h2zm-4 12V9H9v6h6z"/></svg>
            </span>
          </div>
          <div class="card-value" id="cpuVal">0.0%</div>
          <div class="progress-bar">
            <div class="progress-fill" id="cpuBar" style="width: 0%;"></div>
          </div>
          <div class="card-subtitle">
            <span>Zero-alloc /proc parser</span>
            <span id="cpuDeltaText">Sub-microsecond</span>
          </div>
        </div>

        <!-- RAM CARD -->
        <div class="card">
          <div class="card-header">
            <span class="card-title">Memory Allocation</span>
            <span class="card-icon">
              <svg viewBox="0 0 24 24"><path d="M4 6h16v12H4zM2 4v16h20V4H2zm3 4h2v8H5V8zm4 0h2v8H9V8zm4 0h2v8h-2V8zm4 0h2v8h-2V8z"/></svg>
            </span>
          </div>
          <div class="card-value" id="ramVal">0 / 0 MB</div>
          <div class="progress-bar">
            <div class="progress-fill" id="ramBar" style="width: 0%;"></div>
          </div>
          <div class="card-subtitle">
            <span id="ramFreeText">Available: 0 MB</span>
            <span id="ramPercentText">0.0%</span>
          </div>
        </div>

        <!-- NETWORK CARD -->
        <div class="card">
          <div class="card-header">
            <span class="card-title">Network Throughput</span>
            <span class="card-icon">
              <svg viewBox="0 0 24 24"><path d="M4.5 11h-2V9H1v6h1.5v-2h2v2H6V9H4.5v2zm15 0h-2V9H16v6h1.5v-2h2v2H21V9h-1.5v2zm-7.5-6h-1V2H8v5h3v2h2V7h3V2h-3v3h-1zM11 17h2v2h-2v-2zm-3 2h2v2H8v-2zm6 0h2v2h-2v-2zm-5 2h4v1h-4v-1z"/></svg>
            </span>
          </div>
          <div class="card-value" id="netVal" style="font-size: 1.6rem;">↓ 0 KB/s</div>
          <div class="card-subtitle" style="margin-top: 1rem;">
            <span id="netTxText">↑ 0 KB/s outbound</span>
            <span>/proc/net/dev</span>
          </div>
        </div>

        <!-- STACK INFO CARD -->
        <div class="card">
          <div class="card-header">
            <span class="card-title">Engine Topology</span>
            <span class="card-icon">
              <svg viewBox="0 0 24 24"><path d="M12 2L1 21h22L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 16h2v2h-2zm0-6h2v4h-2z"/></svg>
            </span>
          </div>
          <div class="card-value" style="font-size: 1.35rem; color: var(--cyan-glow);">Pure Rust + Caddy v2</div>
          <div class="card-subtitle" style="margin-top: 1.25rem;">
            <span>Daemon RAM: &lt; 10 MB</span>
            <span style="color: var(--green);">Zero External Deps</span>
          </div>
        </div>
      </div>

      <!-- SITES SUMMARY TABLE -->
      <div class="toolbar">
        <h3 class="toolbar-title">Active Virtual Hosts</h3>
        <button class="btn" onclick="openAddSiteModal()">
          <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
          New Site
        </button>
      </div>
      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Domain</th>
              <th>Type</th>
              <th>PHP Pool</th>
              <th>Web Root</th>
              <th>HTTPS</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="overviewSitesTable">
            <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2rem;">Loading websites...</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- TAB: SITES -->
    <div id="tab-sites" class="tab-content">
      <div class="toolbar">
        <h2 class="toolbar-title">Virtual Host Management</h2>
        <div class="toolbar-actions">
          <input type="text" id="siteSearchInput" class="search-input" placeholder="Search domain..." oninput="filterSitesTable()">
          <button class="btn" onclick="openAddSiteModal()">
            <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
            Create Virtual Host
          </button>
        </div>
      </div>
      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Domain</th>
              <th>Type</th>
              <th>PHP Engine</th>
              <th>Document Root</th>
              <th>Security / SSL</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="sitesTableBody">
            <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2rem;">Loading hosted sites...</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- TAB: FILES -->
    <div id="tab-files" class="tab-content">
      <div class="toolbar">
        <div style="display: flex; gap: 1rem; align-items: center; flex-wrap: wrap;">
          <select id="fileSiteSelect" class="form-select" style="width: 280px;" onchange="loadSiteFiles()"></select>
          <div style="display: flex; align-items: center; gap: 0.35rem; font-family: var(--font-mono); font-size: 0.85rem; background: var(--surface); padding: 0.4rem 0.8rem; border-radius: 0.4rem; border: 1px solid var(--border);">
            <svg viewBox="0 0 24 24" style="width: 16px; height: 16px; fill: var(--cyan);"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg>
            <span id="fileBreadcrumb" style="color: var(--cyan-glow);">/</span>
          </div>
        </div>
        <div class="toolbar-actions">
          <button class="btn btn-secondary" onclick="openNewEntryModal(false)">
            <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
            + File
          </button>
          <button class="btn btn-secondary" onclick="openNewEntryModal(true)">
            <svg viewBox="0 0 24 24"><path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-1 8h-3v3h-2v-3h-3v-2h3V9h2v3h3v2z"/></svg>
            + Folder
          </button>
          <button class="btn btn-secondary" onclick="loadSiteFiles()">
            <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
            Refresh
          </button>
        </div>
      </div>
      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>File Name</th>
              <th>Type</th>
              <th>Size</th>
              <th>POSIX Permissions</th>
              <th style="text-align: right;">Actions</th>
            </tr>
          </thead>
          <tbody id="filesTableBody">
            <tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">Select a virtual host to inspect files.</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- TAB: SERVICES -->
    <div id="tab-services" class="tab-content">
      <div class="toolbar">
        <h2 class="toolbar-title">Systemd Web & Database Services</h2>
        <button class="btn btn-secondary" onclick="loadServices()">
          <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
          Refresh Status
        </button>
      </div>
      <div class="grid-4" id="servicesGrid">
        Loading services...
      </div>
    </div>

    <!-- TAB: CADDYFILE -->
    <div id="tab-caddy" class="tab-content">
      <div class="toolbar">
        <div>
          <h2 class="toolbar-title">Active Reverse Proxy Configuration</h2>
          <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;">Automatically synchronized to <code>/etc/caddy/Caddyfile</code></div>
        </div>
        <div class="toolbar-actions">
          <button class="btn btn-secondary" onclick="copyCaddyfile()">Copy Config</button>
          <button class="btn" onclick="loadCaddyfile()">Reload Preview</button>
        </div>
      </div>
      <pre class="code-block" id="caddyfileContent">Loading Caddyfile...</pre>
    </div>
  </main>

  <!-- MODAL: ADD SITE -->
  <div id="addSiteModal" class="modal">
    <div class="modal-box">
      <div class="modal-header">
        <h3>Deploy New Virtual Host</h3>
        <button class="modal-close" onclick="closeAddSiteModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label>Fully Qualified Domain Name</label>
          <input id="newDomain" class="form-input" placeholder="e.g. blog.company.com" oninput="autoSuggestRoot()">
        </div>
        <div class="form-group">
          <label>Application Type</label>
          <select id="newKind" class="form-select" onchange="togglePhpField()">
            <option value="static">Static HTML / CSS / JS / Assets</option>
            <option value="spa_fallback">Single Page Application (SPA Fallback /index.html)</option>
            <option value="php_fpm">Dynamic PHP (PHP-FPM Unix Socket)</option>
          </select>
        </div>
        <div class="form-group" id="phpVersionGroup" style="display: none;">
          <label>PHP-FPM Worker Version</label>
          <select id="newPhpVer" class="form-select">
            <option value="8.3">PHP 8.3-FPM (Latest)</option>
            <option value="8.2" selected>PHP 8.2-FPM (Recommended LTS)</option>
            <option value="8.1">PHP 8.1-FPM</option>
          </select>
        </div>
        <div class="form-group">
          <label>Document Web Root Path</label>
          <input id="newRoot" class="form-input" placeholder="/var/www/blog.company.com">
        </div>
        <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.75rem;">
          <input type="checkbox" id="newSsl" checked disabled style="accent-color: var(--cyan);">
          <label for="newSsl" style="font-size: 0.85rem; color: var(--text-muted); cursor: default;">
            Automatic HTTPS via Caddy (Let's Encrypt / ZeroSSL)
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeAddSiteModal()">Cancel</button>
        <button class="btn" onclick="submitCreateSite()">Create Virtual Host</button>
      </div>
    </div>
  </div>

  <!-- MODAL: FILE EDITOR -->
  <div id="fileEditorModal" class="modal">
    <div class="modal-box" style="max-width: 960px; width: 95%;">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 0.75rem;">
          <svg viewBox="0 0 24 24" style="width: 18px; height: 18px; fill: var(--cyan);"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
          <span id="editorFileName" style="font-family: var(--font-mono); font-size: 0.95rem; font-weight: 700;">file.php</span>
          <span id="editorMime" class="badge badge-cyan">text/plain</span>
        </div>
        <button class="modal-close" onclick="closeEditorModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 0;">
        <div class="editor-wrapper">
          <textarea id="editorTextarea" class="editor-textarea" spellcheck="false"></textarea>
        </div>
      </div>
      <div class="modal-footer" style="display: flex; justify-content: space-between; align-items: center;">
        <div style="font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-dim);">
          Shortcut: <kbd style="background: rgba(255,255,255,0.1); padding: 0.15rem 0.4rem; border-radius: 3px;">Ctrl+S</kbd> to save
        </div>
        <div style="display: flex; gap: 0.75rem;">
          <button class="btn btn-secondary" onclick="closeEditorModal()">Cancel</button>
          <button class="btn" onclick="saveFileContent()">Save Changes (Atomic)</button>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: NEW ENTRY -->
  <div id="newEntryModal" class="modal">
    <div class="modal-box" style="max-width: 440px;">
      <div class="modal-header">
        <h3 id="newEntryTitle">Create New Item</h3>
        <button class="modal-close" onclick="closeNewEntryModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label id="newEntryLabel">Item Name</label>
          <input id="newEntryName" class="form-input" placeholder="e.g. index.php">
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeNewEntryModal()">Cancel</button>
        <button class="btn" onclick="submitCreateEntry()">Create</button>
      </div>
    </div>
  </div>

  <!-- TOAST CONTAINER -->
  <div class="toast-container" id="toastContainer"></div>

  <script>
    let currentTab = 'overview';
    let currentSubpath = '';
    let isCreatingDir = false;
    let editingRelPath = '';
    let allSites = [];

    function showToast(message, type = 'info') {
      const c = document.getElementById('toastContainer');
      const t = document.createElement('div');
      t.className = `toast toast-${type}`;
      t.textContent = message;
      c.appendChild(t);
      setTimeout(() => {
        t.style.opacity = '0';
        t.style.transition = 'opacity 0.3s';
        setTimeout(() => t.remove(), 300);
      }, 3500);
    }

    function switchTab(tab) {
      currentTab = tab;
      document.querySelectorAll('nav button').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      
      const navBtn = Array.from(document.querySelectorAll('nav button')).find(b => b.textContent.trim().toLowerCase().includes(tab));
      if (navBtn) navBtn.classList.add('active');
      
      document.getElementById('tab-' + tab).classList.add('active');

      if (tab === 'sites') loadSites();
      if (tab === 'files') initFilesTab();
      if (tab === 'services') loadServices();
      if (tab === 'caddy') loadCaddyfile();
    }

    // Telemetry Polling
    async function pollTelemetry() {
      try {
        const res = await fetch('/api/v1/telemetry');
        if (!res.ok) return;
        const d = await res.json();

        // CPU
        document.getElementById('cpuVal').textContent = d.cpu_usage_percent.toFixed(1) + '%';
        document.getElementById('cpuBar').style.width = Math.min(d.cpu_usage_percent, 100) + '%';

        // RAM
        document.getElementById('ramVal').textContent = `${d.ram_used_mb} / ${d.ram_total_mb} MB`;
        document.getElementById('ramBar').style.width = Math.min(d.ram_usage_percent, 100) + '%';
        document.getElementById('ramFreeText').textContent = `Available: ${d.ram_free_mb} MB`;
        document.getElementById('ramPercentText').textContent = `${d.ram_usage_percent.toFixed(1)}%`;

        // Network
        document.getElementById('netVal').textContent = `↓ ${d.net_rx_kbps} KB/s`;
        document.getElementById('netTxText').textContent = `↑ ${d.net_tx_kbps} KB/s outbound`;

        // Status & Uptime
        document.getElementById('uptimeDisplay').textContent = `Uptime: ${d.uptime_seconds}s`;
        document.getElementById('procMode').textContent = d.is_linux_proc ? 'Linux /proc Native' : 'Local Dev Fallback';
      } catch (e) {
        console.error('Telemetry error:', e);
      }
    }
    setInterval(pollTelemetry, 2000);
    pollTelemetry();

    // Sites Management
    async function loadSites() {
      try {
        const res = await fetch('/api/v1/sites');
        allSites = await res.json();
        renderSitesTable(allSites);
        renderOverviewTable(allSites);
      } catch (e) {
        console.error(e);
      }
    }

    function renderSitesTable(sites) {
      const tbody = document.getElementById('sitesTableBody');
      if (!sites.length) {
        tbody.innerHTML = '<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">No websites hosted yet. Click "+ Create Virtual Host" to start!</td></tr>';
        return;
      }

      tbody.innerHTML = sites.map(s => {
        let kindBadge = '<span class="badge badge-cyan">Static</span>';
        if (s.kind === 'spa_fallback') kindBadge = '<span class="badge badge-purple">SPA Fallback</span>';
        if (s.kind === 'php_fpm') kindBadge = '<span class="badge badge-blue">PHP-FPM</span>';

        return `
          <tr>
            <td style="font-weight: 700; color: var(--cyan-glow); font-size: 0.95rem;">${s.domain}</td>
            <td>${kindBadge}</td>
            <td style="font-family: var(--font-mono);">${s.php_version ? 'PHP ' + s.php_version : '<span style="color: var(--text-dim);">-</span>'}</td>
            <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-muted);">${s.root_path}</td>
            <td><span class="badge badge-green">Auto HTTPS</span></td>
            <td>
              <div style="display: flex; gap: 0.4rem;">
                <button class="btn btn-secondary" style="padding: 0.25rem 0.6rem; font-size: 0.75rem;" onclick="openSiteInFiles('${s.domain}')">Files</button>
                <button class="btn btn-danger" style="padding: 0.25rem 0.6rem; font-size: 0.75rem;" onclick="deleteSite('${s.domain}')">Delete</button>
              </div>
            </td>
          </tr>
        `;
      }).join('');
    }

    function renderOverviewTable(sites) {
      const tbody = document.getElementById('overviewSitesTable');
      if (!sites.length) {
        tbody.innerHTML = '<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 1.5rem;">No virtual hosts registered yet.</td></tr>';
        return;
      }
      tbody.innerHTML = sites.slice(0, 5).map(s => `
        <tr>
          <td style="font-weight: 700; color: var(--cyan-glow);">${s.domain}</td>
          <td><span class="badge badge-cyan">${s.kind}</span></td>
          <td style="font-family: var(--font-mono);">${s.php_version ? 'PHP ' + s.php_version : '-'}</td>
          <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-muted);">${s.root_path}</td>
          <td><span class="badge badge-green">Active</span></td>
          <td>
            <button class="btn btn-secondary" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="openSiteInFiles('${s.domain}')">Browse Files</button>
          </td>
        </tr>
      `).join('');
    }

    function filterSitesTable() {
      const q = document.getElementById('siteSearchInput').value.toLowerCase();
      const filtered = allSites.filter(s => s.domain.toLowerCase().includes(q) || s.root_path.toLowerCase().includes(q));
      renderSitesTable(filtered);
    }

    function autoSuggestRoot() {
      const dom = document.getElementById('newDomain').value.trim();
      if (dom) {
        document.getElementById('newRoot').value = '/var/www/' + dom;
      }
    }

    function openAddSiteModal() {
      document.getElementById('newDomain').value = '';
      document.getElementById('newRoot').value = '';
      document.getElementById('addSiteModal').classList.add('active');
    }
    function closeAddSiteModal() {
      document.getElementById('addSiteModal').classList.remove('active');
    }
    function togglePhpField() {
      const kind = document.getElementById('newKind').value;
      document.getElementById('phpVersionGroup').style.display = kind === 'php_fpm' ? 'block' : 'none';
    }

    async function submitCreateSite() {
      const domain = document.getElementById('newDomain').value.trim();
      const kind = document.getElementById('newKind').value;
      const phpVer = kind === 'php_fpm' ? document.getElementById('newPhpVer').value : null;
      let root = document.getElementById('newRoot').value.trim();

      if (!domain) {
        showToast('Domain name is required', 'error');
        return;
      }
      if (!root) root = '/var/www/' + domain;

      const payload = {
        domain,
        root_path: root,
        kind,
        php_version: phpVer,
        ssl_enabled: true
      };

      try {
        const res = await fetch('/api/v1/sites', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        if (res.ok) {
          closeAddSiteModal();
          showToast(`Virtual host '${domain}' created successfully!`, 'success');
          loadSites();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Network error: ' + e, 'error');
      }
    }

    async function deleteSite(domain) {
      if (!confirm(`Are you sure you want to remove domain '${domain}' from ZPanl?`)) return;
      try {
        const res = await fetch(`/api/v1/sites?domain=${encodeURIComponent(domain)}`, { method: 'DELETE' });
        if (res.ok) {
          showToast(`Site '${domain}' deleted`, 'success');
          loadSites();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }

    // File Manager
    async function initFilesTab() {
      const res = await fetch('/api/v1/sites');
      const sites = await res.json();
      const sel = document.getElementById('fileSiteSelect');
      sel.innerHTML = sites.map(s => `<option value="${s.domain}">🌐 ${s.domain} (${s.root_path})</option>`).join('');
      currentSubpath = '';
      if (sites.length) loadSiteFiles();
    }

    function openSiteInFiles(domain) {
      switchTab('files');
      document.getElementById('fileSiteSelect').value = domain;
      currentSubpath = '';
      loadSiteFiles();
    }

    async function loadSiteFiles() {
      const domain = document.getElementById('fileSiteSelect').value;
      if (!domain) return;

      document.getElementById('fileBreadcrumb').textContent = '/' + currentSubpath;

      try {
        const res = await fetch(`/api/v1/files/list?site=${encodeURIComponent(domain)}&path=${encodeURIComponent(currentSubpath)}`);
        const tbody = document.getElementById('filesTableBody');
        if (!res.ok) {
          tbody.innerHTML = `<tr><td colspan="5" style="color: var(--red); text-align: center; padding: 2rem;">Error: ${await res.text()}</td></tr>`;
          return;
        }
        const entries = await res.json();

        let rows = '';
        if (currentSubpath) {
          rows += `<tr>
            <td colspan="5" style="cursor: pointer; color: var(--cyan-glow); font-weight: 600;" onclick="navigateUp()">
              📁 .. (Back to parent directory)
            </td>
          </tr>`;
        }

        rows += entries.map(e => {
          const isDir = e.file_type === 'Directory';
          const icon = isDir ? '📁' : '📄';
          const clickAction = isDir
            ? `onclick="navigateDir('${e.name}')"`
            : `onclick="openEditor('${e.rel_path}')"`;
          return `
            <tr>
              <td style="cursor: pointer; font-weight: 500;" ${clickAction}>
                <span style="margin-right: 0.5rem;">${icon}</span>
                <span style="${isDir ? 'color: var(--cyan-glow); font-weight: 600;' : ''}">${e.name}</span>
              </td>
              <td><span class="badge ${isDir ? 'badge-yellow' : 'badge-cyan'}">${e.file_type}</span></td>
              <td style="font-family: var(--font-mono);">${formatBytes(e.size_bytes)}</td>
              <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-dim);">0o${e.posix_mode.toString(8)}</td>
              <td style="text-align: right;">
                <div style="display: inline-flex; gap: 0.4rem;">
                  ${!isDir ? `<button class="btn btn-secondary" style="padding: 0.2rem 0.55rem; font-size: 0.75rem;" onclick="openEditor('${e.rel_path}')">Edit</button>` : ''}
                  <button class="btn btn-danger" style="padding: 0.2rem 0.55rem; font-size: 0.75rem;" onclick="deleteFileItem('${e.rel_path}')">Delete</button>
                </div>
              </td>
            </tr>
          `;
        }).join('');

        tbody.innerHTML = rows || '<tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 2rem;">Directory is empty.</td></tr>';
      } catch (e) {
        console.error(e);
      }
    }

    function navigateDir(dirName) {
      currentSubpath = currentSubpath ? currentSubpath + '/' + dirName : dirName;
      loadSiteFiles();
    }
    function navigateUp() {
      const parts = currentSubpath.split('/');
      parts.pop();
      currentSubpath = parts.join('/');
      loadSiteFiles();
    }

    // File Editor
    async function openEditor(relPath) {
      const domain = document.getElementById('fileSiteSelect').value;
      editingRelPath = relPath;
      try {
        const res = await fetch(`/api/v1/files/read?site=${encodeURIComponent(domain)}&path=${encodeURIComponent(relPath)}`);
        if (!res.ok) {
          showToast('Cannot open: ' + await res.text(), 'error');
          return;
        }
        const data = await res.json();
        document.getElementById('editorFileName').textContent = relPath;
        document.getElementById('editorMime').textContent = data.mime;
        document.getElementById('editorTextarea').value = data.content;
        document.getElementById('fileEditorModal').classList.add('active');
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }
    function closeEditorModal() {
      document.getElementById('fileEditorModal').classList.remove('active');
    }
    async function saveFileContent() {
      const domain = document.getElementById('fileSiteSelect').value;
      const content = document.getElementById('editorTextarea').value;
      try {
        const res = await fetch('/api/v1/files/save', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ site: domain, path: editingRelPath, content })
        });
        if (res.ok) {
          showToast(`File '${editingRelPath}' saved atomically!`, 'success');
          loadSiteFiles();
        } else {
          showToast('Save failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }

    // Ctrl+S in editor
    document.addEventListener('keydown', e => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        if (document.getElementById('fileEditorModal').classList.contains('active')) {
          e.preventDefault();
          saveFileContent();
        }
      }
      if (e.key === 'Escape') {
        closeEditorModal();
        closeAddSiteModal();
        closeNewEntryModal();
      }
    });

    async function deleteFileItem(relPath) {
      if (!confirm(`Delete '${relPath}' permanently?`)) return;
      const domain = document.getElementById('fileSiteSelect').value;
      try {
        const res = await fetch('/api/v1/files/delete', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ site: domain, path: relPath })
        });
        if (res.ok) {
          showToast(`Deleted '${relPath}'`, 'success');
          loadSiteFiles();
        } else {
          showToast('Delete failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }

    function openNewEntryModal(isDir) {
      isCreatingDir = isDir;
      document.getElementById('newEntryTitle').textContent = isDir ? 'Create New Directory' : 'Create New File';
      document.getElementById('newEntryLabel').textContent = isDir ? 'Folder Name' : 'File Name (e.g. index.php)';
      document.getElementById('newEntryName').value = '';
      document.getElementById('newEntryModal').classList.add('active');
    }
    function closeNewEntryModal() {
      document.getElementById('newEntryModal').classList.remove('active');
    }
    async function submitCreateEntry() {
      const name = document.getElementById('newEntryName').value.trim();
      if (!name) return;
      const domain = document.getElementById('fileSiteSelect').value;
      const fullRel = currentSubpath ? currentSubpath + '/' + name : name;
      try {
        const res = await fetch('/api/v1/files/create', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ site: domain, path: fullRel, is_dir: isCreatingDir })
        });
        if (res.ok) {
          closeNewEntryModal();
          showToast(`Created ${isCreatingDir ? 'directory' : 'file'} '${name}'`, 'success');
          loadSiteFiles();
        } else {
          showToast('Create failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }

    // Services
    async function loadServices() {
      try {
        const res = await fetch('/api/v1/services');
        const services = await res.json();
        const grid = document.getElementById('servicesGrid');
        grid.innerHTML = services.map(s => `
          <div class="card">
            <div class="card-header">
              <div>
                <strong style="font-size: 1.05rem;">${s.display_name}</strong>
                <div style="font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-dim); margin-top: 0.2rem;">${s.name}.service</div>
              </div>
              <span class="badge ${s.is_active ? 'badge-green' : 'badge-red'}">
                <span class="pulse-dot" style="background: ${s.is_active ? 'var(--green)' : 'var(--red)'}; box-shadow: 0 0 6px ${s.is_active ? 'var(--green)' : 'var(--red)'}"></span>
                ${s.state}
              </span>
            </div>
            <div style="display: flex; gap: 0.5rem; margin-top: 1.25rem;">
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.8rem; justify-content: center;" onclick="actionService('${s.name}', 'restart')">Restart</button>
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.8rem; justify-content: center;" onclick="actionService('${s.name}', 'reload')">Reload</button>
            </div>
          </div>
        `).join('');
      } catch (e) {
        console.error(e);
      }
    }

    async function actionService(name, action) {
      try {
        const res = await fetch('/api/v1/services/action', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ name, action })
        });
        if (res.ok) {
          showToast(`Service '${name}' ${action} executed`, 'success');
          loadServices();
        } else {
          showToast('Action failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e, 'error');
      }
    }

    // Caddyfile
    async function loadCaddyfile() {
      try {
        const res = await fetch('/api/v1/caddy/caddyfile');
        const text = await res.text();
        document.getElementById('caddyfileContent').textContent = text;
      } catch (e) {
        console.error(e);
      }
    }

    function copyCaddyfile() {
      const text = document.getElementById('caddyfileContent').textContent;
      navigator.clipboard.writeText(text);
      showToast('Caddyfile copied to clipboard!', 'info');
    }

    function formatBytes(bytes) {
      if (bytes === 0) return '-';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }

    // Initial load
    loadSites();
  </script>
</body>
</html>
"###;
