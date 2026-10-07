/// Embedded professional single-page application for the ZPanl web control panel.
pub const INDEX_HTML: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ZPanl ⚡ Sovereign Web Panel</title>
  <style>
    :root {
      --bg: #090d16;
      --sidebar-bg: #0d121f;
      --surface: #121829;
      --surface-elevated: #18223a;
      --surface-hover: #1e2a47;
      --border: rgba(255, 255, 255, 0.08);
      --border-light: rgba(255, 255, 255, 0.12);
      --text: #f8fafc;
      --text-muted: #94a3b8;
      --text-dim: #64748b;
      --cyan: #06b6d4;
      --cyan-glow: #22d3ee;
      --blue: #3b82f6;
      --green: #10b981;
      --green-glow: #34d399;
      --yellow: #f59e0b;
      --red: #f43f5e;
      --purple: #a855f7;
      --font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Inter, Helvetica, Arial, sans-serif;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: var(--bg);
      color: var(--text);
      font-family: var(--font-sans);
      min-height: 100vh;
      display: flex;
      overflow-x: hidden;
      -webkit-font-smoothing: antialiased;
    }

    /* APP LAYOUT: SIDEBAR + CONTENT */
    #sidebar {
      width: 260px;
      background: var(--sidebar-bg);
      border-right: 1px solid var(--border);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
      position: sticky;
      top: 0;
      height: 100vh;
      z-index: 50;
    }
    .sidebar-brand {
      padding: 1.25rem 1.5rem;
      display: flex;
      align-items: center;
      gap: 0.85rem;
      border-bottom: 1px solid var(--border);
      text-decoration: none;
      color: inherit;
    }
    .brand-logo-icon {
      width: 36px;
      height: 36px;
      background: linear-gradient(135deg, var(--cyan), var(--blue));
      border-radius: 9px;
      display: flex;
      align-items: center;
      justify-content: center;
      box-shadow: 0 0 16px rgba(6, 182, 212, 0.4);
    }
    .brand-logo-icon svg { width: 20px; height: 20px; fill: #fff; }
    .brand-names {
      display: flex;
      flex-direction: column;
    }
    .brand-title {
      font-weight: 800;
      font-size: 1.2rem;
      letter-spacing: -0.03em;
      display: flex;
      align-items: center;
      gap: 0.4rem;
    }
    .brand-tag {
      font-size: 0.65rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--cyan);
    }

    .sidebar-nav {
      flex: 1;
      padding: 1.25rem 0.75rem;
      overflow-y: auto;
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
    }
    .nav-group-title {
      font-size: 0.7rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--text-dim);
      padding: 0 0.75rem;
      margin-bottom: 0.4rem;
    }
    .nav-list {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
    }
    .nav-item button {
      width: 100%;
      background: transparent;
      border: 1px solid transparent;
      color: var(--text-muted);
      padding: 0.55rem 0.85rem;
      border-radius: 0.5rem;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 0.75rem;
      transition: all 0.15s ease;
      text-align: left;
    }
    .nav-item button svg {
      width: 18px;
      height: 18px;
      fill: currentColor;
      opacity: 0.8;
      transition: all 0.15s ease;
    }
    .nav-item button:hover {
      color: var(--text);
      background: rgba(255, 255, 255, 0.04);
    }
    .nav-item button.active {
      color: #fff;
      background: linear-gradient(90deg, rgba(6, 182, 212, 0.2), rgba(59, 130, 246, 0.1));
      border-color: rgba(6, 182, 212, 0.4);
      box-shadow: 0 0 15px rgba(6, 182, 212, 0.15);
    }
    .nav-item button.active svg {
      fill: var(--cyan-glow);
      opacity: 1;
    }
    .nav-badge {
      margin-left: auto;
      font-size: 0.7rem;
      font-family: var(--font-mono);
      background: rgba(255, 255, 255, 0.06);
      padding: 0.1rem 0.45rem;
      border-radius: 999px;
      color: var(--text-muted);
    }

    .sidebar-footer {
      padding: 1rem 1.25rem;
      border-top: 1px solid var(--border);
      background: rgba(0, 0, 0, 0.2);
    }
    .server-status-pill {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      font-size: 0.75rem;
      color: var(--text-muted);
      font-family: var(--font-mono);
    }
    .pulse-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--green);
      box-shadow: 0 0 8px var(--green);
      animation: pulse 2s infinite;
      display: inline-block;
    }
    @keyframes pulse {
      0%, 100% { opacity: 1; transform: scale(1); }
      50% { opacity: 0.4; transform: scale(0.85); }
    }

    /* MAIN CONTENT WRAPPER */
    #content-wrapper {
      flex: 1;
      display: flex;
      flex-direction: column;
      min-width: 0;
      background: var(--bg);
    }

    /* TOP HEADER BAR */
    #topbar {
      height: 64px;
      background: rgba(13, 18, 31, 0.75);
      backdrop-filter: blur(16px);
      border-bottom: 1px solid var(--border);
      padding: 0 2rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      position: sticky;
      top: 0;
      z-index: 40;
    }
    .topbar-breadcrumb {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      font-size: 0.85rem;
      color: var(--text-muted);
    }
    .topbar-breadcrumb strong {
      color: var(--text);
      font-weight: 700;
    }
    .topbar-quick-stats {
      display: flex;
      align-items: center;
      gap: 1.25rem;
    }
    .stat-pill {
      display: flex;
      align-items: center;
      gap: 0.45rem;
      font-size: 0.78rem;
      font-family: var(--font-mono);
      background: rgba(255, 255, 255, 0.03);
      border: 1px solid var(--border);
      padding: 0.35rem 0.75rem;
      border-radius: 0.4rem;
      color: var(--text-muted);
    }
    .stat-pill strong { color: var(--cyan-glow); font-weight: 700; }

    /* CONTENT VIEW */
    main {
      padding: 2rem;
      max-width: 1440px;
      width: 100%;
      margin: 0 auto;
    }
    .tab-content { display: none; animation: fadeIn 0.2s ease-out; }
    .tab-content.active { display: block; }
    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(4px); }
      to { opacity: 1; transform: translateY(0); }
    }

    /* CARDS & RADIAL GAUGES */
    .grid-4 {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
      gap: 1.25rem;
      margin-bottom: 2rem;
    }
    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 0.75rem;
      padding: 1.5rem;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
      position: relative;
      overflow: hidden;
      transition: border-color 0.2s, transform 0.15s;
    }
    .card:hover {
      border-color: var(--border-light);
    }
    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 1rem;
    }
    .card-title {
      font-size: 0.75rem;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--text-dim);
      font-weight: 700;
    }
    .card-icon {
      color: var(--cyan);
      opacity: 0.8;
    }
    .card-icon svg { width: 20px; height: 20px; fill: currentColor; }

    /* RADIAL METER WRAPPER */
    .radial-gauge-container {
      display: flex;
      align-items: center;
      gap: 1.5rem;
    }
    .radial-circle {
      position: relative;
      width: 80px;
      height: 80px;
      flex-shrink: 0;
    }
    .radial-circle svg {
      width: 80px;
      height: 80px;
      transform: rotate(-90deg);
    }
    .radial-bg {
      fill: none;
      stroke: rgba(255, 255, 255, 0.06);
      stroke-width: 7;
    }
    .radial-progress {
      fill: none;
      stroke: var(--cyan);
      stroke-width: 7;
      stroke-linecap: round;
      stroke-dasharray: 226;
      stroke-dashoffset: 226;
      transition: stroke-dashoffset 0.6s cubic-bezier(0.4, 0, 0.2, 1);
    }
    .radial-label {
      position: absolute;
      inset: 0;
      display: flex;
      align-items: center;
      justify-content: center;
      font-family: var(--font-mono);
      font-weight: 800;
      font-size: 0.95rem;
      color: var(--text);
    }
    .radial-details {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
    }
    .radial-main-val {
      font-size: 1.5rem;
      font-weight: 800;
      font-family: var(--font-mono);
      letter-spacing: -0.03em;
    }
    .radial-sub-val {
      font-size: 0.75rem;
      color: var(--text-muted);
    }

    /* TOOLBARS */
    .toolbar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 1.25rem;
      gap: 1rem;
      flex-wrap: wrap;
    }
    .toolbar-title {
      font-size: 1.25rem;
      font-weight: 800;
      letter-spacing: -0.02em;
    }
    .toolbar-actions {
      display: flex;
      gap: 0.6rem;
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
      background: var(--surface-hover);
      border-color: var(--border-light);
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
      background: var(--surface);
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
      background: rgba(0, 0, 0, 0.25);
      color: var(--text-dim);
      padding: 0.85rem 1.25rem;
      font-size: 0.72rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      border-bottom: 1px solid var(--border);
    }
    td {
      padding: 0.9rem 1.25rem;
      border-bottom: 1px solid var(--border);
      vertical-align: middle;
    }
    tr:last-child td { border-bottom: none; }
    tr:hover td { background: rgba(255, 255, 255, 0.015); }

    /* BADGES */
    .badge {
      display: inline-flex;
      align-items: center;
      gap: 0.35rem;
      padding: 0.25rem 0.6rem;
      border-radius: 0.35rem;
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

    /* SEARCH INPUTS */
    .search-input {
      background: var(--surface-elevated);
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
      background: rgba(0, 0, 0, 0.8);
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
      max-width: 650px;
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
      font-size: 0.75rem;
      font-weight: 700;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.06em;
      margin-bottom: 0.4rem;
    }
    .form-input, .form-select {
      width: 100%;
      background: var(--bg);
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

    /* AAPANEL STYLE SITE MODIFICATION MODAL */
    .btn-success {
      background: #10b981;
      color: #fff;
      border: 1px solid rgba(16, 185, 129, 0.4);
      box-shadow: 0 2px 8px rgba(16, 185, 129, 0.25);
    }
    .btn-success:hover {
      background: #059669;
      transform: translateY(-1px);
    }
    .mod-sidebar {
      width: 190px;
      background: #0a0e1a;
      border-right: 1px solid var(--border);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
      padding: 0.5rem 0;
    }
    .mod-tab-item {
      padding: 0.8rem 1.25rem;
      font-size: 0.84rem;
      color: var(--text-muted);
      cursor: pointer;
      transition: all 0.15s ease;
      display: flex;
      align-items: center;
      border-left: 3px solid transparent;
      user-select: none;
    }
    .mod-tab-item:hover {
      background: var(--surface-hover);
      color: var(--text);
    }
    .mod-tab-item.active {
      background: #141c2e;
      color: var(--cyan-glow);
      border-left-color: var(--cyan);
      font-weight: 600;
    }
    .mod-content {
      flex: 1;
      padding: 1.5rem;
      overflow-y: auto;
      background: var(--surface);
    }
    .mod-tab-content {
      display: none;
    }
    .mod-tab-content.active {
      display: block;
    }
    .mod-hint-box {
      background: #090e1b;
      border: 1px solid rgba(56, 189, 248, 0.2);
      border-radius: 0.5rem;
      padding: 0.85rem 1.15rem;
      font-size: 0.8rem;
      color: #94a3b8;
      line-height: 1.6;
    }
    .toggle-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0.85rem 1rem;
      background: var(--surface-elevated);
      border: 1px solid var(--border);
      border-radius: 0.5rem;
      margin-bottom: 0.85rem;
    }
    .toggle-row label {
      cursor: pointer;
      font-weight: 600;
      font-size: 0.85rem;
      display: flex;
      flex-direction: column;
      gap: 0.2rem;
    }
    .toggle-row label span.sub {
      font-size: 0.75rem;
      color: var(--text-muted);
      font-weight: normal;
    }
  </style>
</head>
<body>
  <!-- LEFT SIDEBAR -->
  <aside id="sidebar">
    <a href="#" class="sidebar-brand" onclick="switchTab('overview')">
      <div class="brand-logo-icon">
        <svg viewBox="0 0 24 24"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
      </div>
      <div class="brand-names">
        <div class="brand-title">ZPanl</div>
        <div class="brand-tag">SOVEREIGN EDGE</div>
      </div>
    </a>

    <div class="sidebar-nav">
      <!-- CORE SECTION -->
      <div>
        <div class="nav-group-title">Core Management</div>
        <ul class="nav-list">
          <li class="nav-item">
            <button class="active" onclick="switchTab('overview')">
              <svg viewBox="0 0 24 24"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>
              Dashboard
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('sites')">
              <svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
              Websites
              <span class="nav-badge" id="navSitesCount">0</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('files')">
              <svg viewBox="0 0 24 24"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg>
              File Manager
            </button>
          </li>
        </ul>
      </div>

      <!-- SYSTEM & SERVERS -->
      <div>
        <div class="nav-group-title">Services & Engines</div>
        <ul class="nav-list">
          <li class="nav-item">
            <button onclick="switchTab('services')">
              <svg viewBox="0 0 24 24"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
              Services
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('caddy')">
              <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
              Caddyfile
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('php')">
              <svg viewBox="0 0 24 24"><path d="M4 4h16v16H4V4zm2 4v8h2v-3h2c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2H6zm2 2h2v2H8v-2zm7-2v8h2v-3h2c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2h-4zm2 2h2v2h-2v-2z"/></svg>
              PHP-FPM Pools
            </button>
          </li>
        </ul>
      </div>
    </div>

    <!-- SIDEBAR FOOTER -->
    <div class="sidebar-footer">
      <div class="server-status-pill">
        <span class="pulse-dot"></span>
        <span id="sidebarProcMode">Linux /proc Active</span>
      </div>
      <div style="font-size: 0.7rem; color: var(--text-dim); margin-top: 0.25rem;">
        Footprint: &lt; 10 MB RAM
      </div>
    </div>
  </aside>

  <!-- CONTENT WRAPPER -->
  <div id="content-wrapper">
    <!-- TOPBAR -->
    <header id="topbar">
      <div class="topbar-breadcrumb">
        <span>ZPanl</span>
        <span>/</span>
        <strong id="breadcrumbTitle">Dashboard</strong>
      </div>

      <div class="topbar-quick-stats">
        <div class="stat-pill">
          <span>CPU:</span>
          <strong id="topbarCpu">0.0%</strong>
        </div>
        <div class="stat-pill">
          <span>RAM:</span>
          <strong id="topbarRam">0 / 0 MB</strong>
        </div>
        <div class="stat-pill">
          <span>NET:</span>
          <strong id="topbarNet">↓ 0 KB/s</strong>
        </div>
        <button class="btn" style="padding: 0.35rem 0.8rem; font-size: 0.78rem;" onclick="openAddSiteModal()">
          <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
          Deploy Site
        </button>
      </div>
    </header>

    <main>
      <!-- TAB 1: DASHBOARD -->
      <div id="tab-overview" class="tab-content active">
        <!-- 4 RADIAL GAUGES (AAPANEL STYLE) -->
        <div class="grid-4">
          <!-- CPU GAUGE -->
          <div class="card">
            <div class="card-header">
              <span class="card-title">CPU Utilization</span>
              <span class="card-icon">
                <svg viewBox="0 0 24 24"><path d="M17 17H7V7h10v10zm2-14v2h2v2h-2v2h2v2h-2v2h2v2h-2v2h-2v-2h-2v2h-2v-2h-2v2H7v-2H5v-2H3v-2h2v-2H3v-2h2V9H3V7h2V5h2V3h2v2h2V3h2v2h2V3h2zm-4 12V9H9v6h6z"/></svg>
              </span>
            </div>
            <div class="radial-gauge-container">
              <div class="radial-circle">
                <svg viewBox="0 0 80 80">
                  <circle class="radial-bg" cx="40" cy="40" r="36"/>
                  <circle class="radial-progress" id="cpuRadial" cx="40" cy="40" r="36"/>
                </svg>
                <div class="radial-label" id="cpuRadialText">0%</div>
              </div>
              <div class="radial-details">
                <div class="radial-main-val" id="cpuDetailVal">0.0%</div>
                <div class="radial-sub-val">Linux /proc/stat delta</div>
                <div class="radial-sub-val" style="color: var(--cyan);">Non-blocking parser</div>
              </div>
            </div>
          </div>

          <!-- RAM GAUGE -->
          <div class="card">
            <div class="card-header">
              <span class="card-title">Memory Allocation</span>
              <span class="card-icon">
                <svg viewBox="0 0 24 24"><path d="M4 6h16v12H4zM2 4v16h20V4H2zm3 4h2v8H5V8zm4 0h2v8H9V8zm4 0h2v8h-2V8zm4 0h2v8h-2V8z"/></svg>
              </span>
            </div>
            <div class="radial-gauge-container">
              <div class="radial-circle">
                <svg viewBox="0 0 80 80">
                  <circle class="radial-bg" cx="40" cy="40" r="36"/>
                  <circle class="radial-progress" id="ramRadial" cx="40" cy="40" r="36" style="stroke: var(--purple);"/>
                </svg>
                <div class="radial-label" id="ramRadialText">0%</div>
              </div>
              <div class="radial-details">
                <div class="radial-main-val" id="ramDetailVal">0 / 0 MB</div>
                <div class="radial-sub-val" id="ramAvailText">Available: 0 MB</div>
                <div class="radial-sub-val" style="color: var(--purple);">/proc/meminfo</div>
              </div>
            </div>
          </div>

          <!-- NETWORK THROUGHPUT -->
          <div class="card">
            <div class="card-header">
              <span class="card-title">Network I/O</span>
              <span class="card-icon">
                <svg viewBox="0 0 24 24"><path d="M4.5 11h-2V9H1v6h1.5v-2h2v2H6V9H4.5v2zm15 0h-2V9H16v6h1.5v-2h2v2H21V9h-1.5v2zm-7.5-6h-1V2H8v5h3v2h2V7h3V2h-3v3h-1zM11 17h2v2h-2v-2zm-3 2h2v2H8v-2zm6 0h2v2h-2v-2zm-5 2h4v1h-4v-1z"/></svg>
              </span>
            </div>
            <div style="display: flex; flex-direction: column; justify-content: center; height: 80px;">
              <div class="radial-main-val" id="netDetailRx" style="font-size: 1.6rem; color: var(--green-glow);">↓ 0 KB/s</div>
              <div class="radial-sub-val" id="netDetailTx" style="font-size: 0.85rem; margin-top: 0.35rem;">↑ 0 KB/s outbound</div>
            </div>
            <div style="font-size: 0.75rem; color: var(--text-dim); margin-top: 0.5rem; display: flex; justify-content: space-between;">
              <span>Interfaces: eth0, lo</span>
              <span>/proc/net/dev</span>
            </div>
          </div>

          <!-- SYSTEM ARCHITECTURE -->
          <div class="card">
            <div class="card-header">
              <span class="card-title">Panel Sovereign Stack</span>
              <span class="card-icon">
                <svg viewBox="0 0 24 24"><path d="M12 2L1 21h22L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 16h2v2h-2zm0-6h2v4h-2z"/></svg>
              </span>
            </div>
            <div style="display: flex; flex-direction: column; justify-content: center; height: 80px;">
              <div style="font-weight: 800; font-size: 1.25rem; color: var(--cyan-glow);">Pure Rust + Caddy</div>
              <div class="radial-sub-val" style="margin-top: 0.35rem;">Single binary (< 1 MB)</div>
            </div>
            <div style="font-size: 0.75rem; color: var(--green); display: flex; justify-content: space-between; margin-top: 0.5rem;">
              <span>Zero external deps</span>
              <span id="uptimeQuickText">Uptime: 0s</span>
            </div>
          </div>
        </div>

        <!-- RECENT SITES -->
        <div class="toolbar">
          <h3 class="toolbar-title">Active Virtual Hosts</h3>
          <button class="btn btn-secondary" onclick="switchTab('sites')">View All Websites &rarr;</button>
        </div>
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th>Domain Name</th>
                <th>Type</th>
                <th>PHP Version</th>
                <th>Web Root</th>
                <th>SSL Security</th>
                <th style="text-align: right;">Action</th>
              </tr>
            </thead>
            <tbody id="overviewSitesTable">
              <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2rem;">Loading websites...</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 2: WEBSITES -->
      <div id="tab-sites" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title">Websites & Virtual Hosts</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;">Declarative Caddy v2 reverse proxy routing with automatic Let's Encrypt HTTPS</div>
          </div>
          <div class="toolbar-actions">
            <input type="text" id="siteSearchInput" class="search-input" placeholder="Search domain or path..." oninput="filterSitesTable()">
            <button class="btn" onclick="openAddSiteModal()">
              <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
              Add Virtual Host
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
                <th style="text-align: right;">Actions</th>
              </tr>
            </thead>
            <tbody id="sitesTableBody">
              <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">Loading websites...</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 3: FILE MANAGER -->
      <div id="tab-files" class="tab-content">
        <div class="toolbar">
          <div style="display: flex; gap: 1rem; align-items: center; flex-wrap: wrap;">
            <select id="fileSiteSelect" class="form-select" style="width: 300px;" onchange="loadSiteFiles()"></select>
            <div style="display: flex; align-items: center; gap: 0.35rem; font-family: var(--font-mono); font-size: 0.85rem; background: var(--surface-elevated); padding: 0.45rem 0.85rem; border-radius: 0.45rem; border: 1px solid var(--border);">
              <svg viewBox="0 0 24 24" style="width: 16px; height: 16px; fill: var(--cyan);"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg>
              <span id="fileBreadcrumb" style="color: var(--cyan-glow);">/</span>
            </div>
          </div>
          <div class="toolbar-actions">
            <button class="btn btn-secondary" onclick="openNewEntryModal(false)">
              <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
              New File
            </button>
            <button class="btn btn-secondary" onclick="openNewEntryModal(true)">
              <svg viewBox="0 0 24 24"><path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-1 8h-3v3h-2v-3h-3v-2h3V9h2v3h3v2z"/></svg>
              New Folder
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
              <tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">Select a site to explore files.</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 4: SERVICES -->
      <div id="tab-services" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title">Linux Systemd Services</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;">Daemon process supervision via <code>zero-sys</code></div>
          </div>
          <button class="btn btn-secondary" onclick="loadServices()">
            <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
            Refresh Daemons
          </button>
        </div>
        <div class="grid-4" id="servicesGrid">
          Loading services...
        </div>
      </div>

      <!-- TAB 5: CADDYFILE -->
      <div id="tab-caddy" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title">Active Reverse Proxy Configuration</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;">Live synchronized from <code>/etc/caddy/Caddyfile</code></div>
          </div>
          <div class="toolbar-actions">
            <button class="btn btn-secondary" onclick="copyCaddyfile()">Copy Caddyfile</button>
            <button class="btn" onclick="loadCaddyfile()">Reload Preview</button>
          </div>
        </div>
        <pre class="code-block" id="caddyfileContent">Loading Caddyfile...</pre>
      </div>

      <!-- TAB 6: PHP-FPM POOLS -->
      <div id="tab-php" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title">PHP-FPM Worker Pools</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;">Isolated on-demand fastcgi worker pools generated by <code>zero-fastcgi</code></div>
          </div>
        </div>
        <div class="card" style="margin-bottom: 1.5rem;">
          <h3 style="font-size: 1.05rem; margin-bottom: 0.5rem;">Select Site to Inspect PHP Pool Configuration</h3>
          <div style="display: flex; gap: 1rem; align-items: center; margin-top: 1rem;">
            <select id="phpSiteSelect" class="form-select" style="width: 320px;" onchange="loadPhpPoolConfig()"></select>
            <button class="btn btn-secondary" onclick="loadPhpPoolConfig()">View Pool INI</button>
          </div>
        </div>
        <pre class="code-block" id="phpPoolConfigContent">Select a PHP website above to inspect its pool.d/*.conf configuration.</pre>
      </div>
    </main>
  </div>

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
          <input id="newDomain" class="form-input" placeholder="e.g. blog.mydomain.com" oninput="autoSuggestRoot()">
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
          <input id="newRoot" class="form-input" placeholder="/var/www/blog.mydomain.com">
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

  <!-- MODAL: SITE MODIFICATION (aaPanel Style) -->
  <div id="siteModModal" class="modal">
    <div class="modal-box" style="max-width: 860px; width: 95%; height: 580px; max-height: 90vh; display: flex; flex-direction: column;">
      <div class="modal-header" style="padding: 0.85rem 1.25rem;">
        <div style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.95rem; font-weight: 600;">
          <svg viewBox="0 0 24 24" style="width: 17px; height: 17px; fill: var(--cyan);"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
          <span>Site modification [<span id="modSiteDomainTitle" style="color: var(--cyan-glow);">domain.com</span>] -- Time added [<span id="modSiteTimeTitle" style="color: var(--text-dim); font-size: 0.8rem;">2026-10-07</span>]</span>
        </div>
        <button class="modal-close" onclick="closeSiteModModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 0; display: flex; flex: 1; overflow: hidden;">
        <!-- LEFT SUB-SIDEBAR -->
        <div class="mod-sidebar">
          <div class="mod-tab-item active" id="btn-modtab-domain" onclick="switchModTab('domain')">Domain Manager</div>
          <div class="mod-tab-item" id="btn-modtab-directory" onclick="switchModTab('directory')">Directory</div>
          <div class="mod-tab-item" id="btn-modtab-limit" onclick="switchModTab('limit')">Limit access</div>
          <div class="mod-tab-item" id="btn-modtab-rewrite" onclick="switchModTab('rewrite')">URL rewrite</div>
          <div class="mod-tab-item" id="btn-modtab-php" onclick="switchModTab('php')">PHP version</div>
          <div class="mod-tab-item" id="btn-modtab-proxy" onclick="switchModTab('proxy')">Reverse proxy</div>
          <div class="mod-tab-item" id="btn-modtab-ssl" onclick="switchModTab('ssl')">SSL</div>
          <div class="mod-tab-item" id="btn-modtab-redirect" onclick="switchModTab('redirect')">Redirect</div>
          <div class="mod-tab-item" id="btn-modtab-hotlink" onclick="switchModTab('hotlink')">Hotlink Protection</div>
          <div class="mod-tab-item" id="btn-modtab-maintenance" onclick="switchModTab('maintenance')">Maintenance Mode</div>
          <div class="mod-tab-item" id="btn-modtab-log" onclick="switchModTab('log')">Response log</div>
          <div class="mod-tab-item" id="btn-modtab-config" onclick="switchModTab('config')">Config (Caddy)</div>
        </div>
        <!-- RIGHT SUB-CONTENT -->
        <div class="mod-content">
          <!-- SUB-TAB 1: DOMAIN MANAGER -->
          <div id="modtab-domain" class="mod-tab-content active">
            <div class="mod-hint-box">
              A domain per line, the default port is 80.<br>
              Wildcard domain format: *.domain.com<br>
              To add another port, the format is www.domain.com:88
            </div>
            <div style="display: flex; gap: 0.75rem; margin-top: 1rem;">
              <textarea id="modNewAliases" class="form-input" style="flex: 1; height: 75px; font-family: var(--font-mono); font-size: 0.85rem;" placeholder="alias1.domain.com&#10;alias2.domain.com:8080"></textarea>
              <button class="btn btn-success" style="align-self: flex-start; padding: 0.6rem 1.25rem;" onclick="addDomainAliases()">Add</button>
            </div>
            <div style="margin-top: 1.25rem; border: 1px solid var(--border); border-radius: 0.4rem; overflow: hidden;">
              <table style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                <thead>
                  <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.2); color: var(--text-dim); text-align: left;">
                    <th style="padding: 0.5rem 0.75rem;">Domain name</th>
                    <th style="padding: 0.5rem 0.75rem; width: 80px;">Port</th>
                    <th style="padding: 0.5rem 0.75rem; text-align: right; width: 100px;">Operate</th>
                  </tr>
                </thead>
                <tbody id="modDomainTableBody"></tbody>
              </table>
            </div>
          </div>

          <!-- SUB-TAB 2: DIRECTORY -->
          <div id="modtab-directory" class="mod-tab-content">
            <div class="form-group">
              <label>Site Base Directory</label>
              <div style="display: flex; gap: 0.5rem;">
                <input id="modRootPath" class="form-input" style="font-family: var(--font-mono);" placeholder="/var/www/domain.com">
                <button class="btn btn-secondary" onclick="openModSiteInFileManager()">Files &rarr;</button>
              </div>
            </div>
            <div class="form-group">
              <label>Running Directory (Sub-path / Web Root)</label>
              <select id="modRunningDir" class="form-select">
                <option value="">/ (Root Directory - Standard)</option>
                <option value="/public">/public (Laravel, Symfony, ThinkPHP)</option>
                <option value="/dist">/dist (Vite, Vue, React Production Build)</option>
                <option value="/build">/build (Webpack, Next.js Static Export)</option>
              </select>
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                Point to framework public folder to keep vendor / .env secure.
              </div>
            </div>
            <div class="form-group">
              <label>Directory Ownership &amp; Permission</label>
              <div style="background: var(--surface-elevated); padding: 0.75rem 1rem; border-radius: 0.45rem; font-family: var(--font-mono); font-size: 0.85rem; border: 1px solid var(--border); color: var(--cyan-glow);">
                User: www-data:www-data | Permissions: 755 (Directories) / 644 (Files)
              </div>
            </div>
          </div>

          <!-- SUB-TAB 3: LIMIT ACCESS -->
          <div id="modtab-limit" class="mod-tab-content">
            <div class="form-group">
              <label>IP Address Blacklist (CIDR notation supported)</label>
              <textarea id="modIpBlacklist" class="form-input" style="height: 100px; font-family: var(--font-mono); font-size: 0.85rem;" placeholder="192.168.1.50&#10;10.0.0.0/8&#10;172.16.0.0/12"></textarea>
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                One IP or CIDR per line. Any connection matching will immediately receive HTTP 403 Forbidden.
              </div>
            </div>
            <div class="toggle-row" style="margin-top: 1rem;">
              <label for="modBasicAuthToggle">
                HTTP Basic Authentication
                <span class="sub">Require username and password before granting access to website</span>
              </label>
              <input type="checkbox" id="modBasicAuthToggle" style="accent-color: var(--cyan); transform: scale(1.3);" onchange="toggleBasicAuthFields()">
            </div>
            <div id="basicAuthFields" style="display: none; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.5rem;">
              <div class="form-group">
                <label>Auth Username</label>
                <input id="modAuthUser" class="form-input" placeholder="admin">
              </div>
              <div class="form-group">
                <label>Auth Password</label>
                <input id="modAuthPass" type="password" class="form-input" placeholder="••••••••">
              </div>
            </div>
          </div>

          <!-- SUB-TAB 4: URL REWRITE -->
          <div id="modtab-rewrite" class="mod-tab-content">
            <div class="form-group">
              <label>Framework URL Rewrite Preset</label>
              <select id="modRewritePreset" class="form-select" onchange="updateRewriteSnippetPreview()">
                <option value="">Default (Static File Server)</option>
                <option value="laravel">Laravel / Symfony (try_files {path} {path}/ /index.php?{query})</option>
                <option value="wordpress">WordPress / WooCommerce (FastCGI + Security Deny rules)</option>
                <option value="spa">Single Page App (try_files {path} /index.html)</option>
              </select>
            </div>
            <div class="form-group">
              <label>Caddyfile Rewrite Snippet Preview</label>
              <pre class="code-block" id="modRewritePreview" style="height: 160px;"></pre>
            </div>
          </div>

          <!-- SUB-TAB 5: PHP VERSION -->
          <div id="modtab-php" class="mod-tab-content">
            <div class="form-group">
              <label>PHP-FPM Worker Runtime</label>
              <select id="modPhpVersion" class="form-select" onchange="updatePhpSocketPreview()">
                <option value="none">Static (No PHP Processing)</option>
                <option value="8.4">PHP 8.4-FPM (Bleeding Edge)</option>
                <option value="8.3">PHP 8.3-FPM (High Performance)</option>
                <option value="8.2">PHP 8.2-FPM (Recommended LTS)</option>
                <option value="8.1">PHP 8.1-FPM</option>
                <option value="7.4">PHP 7.4-FPM (Legacy)</option>
              </select>
            </div>
            <div class="form-group">
              <label>FastCGI Socket Endpoint</label>
              <input id="modPhpSocketPreview" class="form-input" readonly style="font-family: var(--font-mono); color: var(--cyan-glow);">
            </div>
            <div class="mod-hint-box">
              ZPanl configures FastCGI with <code>pm = ondemand</code>, dynamically spinning up worker processes when HTTP requests arrive and terminating idle workers after 10s to keep RAM footprint &lt; 10 MB.
            </div>
          </div>

          <!-- SUB-TAB 6: REVERSE PROXY -->
          <div id="modtab-proxy" class="mod-tab-content">
            <div class="toggle-row">
              <label for="modProxyToggle">
                Enable Reverse Proxy
                <span class="sub">Forward all traffic to internal application server (Node, Go, Python, Docker)</span>
              </label>
              <input type="checkbox" id="modProxyToggle" style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group" style="margin-top: 1rem;">
              <label>Upstream Target (Host:Port or Unix Socket)</label>
              <input id="modProxyUpstream" class="form-input" style="font-family: var(--font-mono);" placeholder="127.0.0.1:3000">
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                Example: <code>127.0.0.1:3000</code> for Next.js, <code>127.0.0.1:8000</code> for Python FastAPI/Django.
              </div>
            </div>
            <div class="mod-hint-box" style="margin-top: 0.75rem;">
              Caddy v2 handles reverse proxying natively with automated WebSocket upgrading (<code>Connection: Upgrade</code>), streaming buffering, and standard proxy headers (<code>X-Forwarded-For</code>, <code>X-Real-IP</code>).
            </div>
          </div>

          <!-- SUB-TAB 7: SSL -->
          <div id="modtab-ssl" class="mod-tab-content">
            <div class="toggle-row">
              <label for="modSslToggle">
                Automatic HTTPS (Let's Encrypt / ZeroSSL)
                <span class="sub">Zero-configuration automated ACME certificate issuance and renewal</span>
              </label>
              <input type="checkbox" id="modSslToggle" checked style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group">
              <label>SSL / TLS Protocol Strictness</label>
              <div style="background: var(--surface-elevated); padding: 0.75rem 1rem; border-radius: 0.45rem; border: 1px solid var(--border); font-size: 0.85rem;">
                <div style="color: var(--green-glow); font-weight: 600; display: flex; align-items: center; gap: 0.5rem;">
                  <span style="display:inline-block;width:8px;height:8px;background:var(--green);border-radius:50%;"></span>
                  TLS 1.2 &amp; TLS 1.3 Modern Cipher Suite Active
                </div>
                <div style="color: var(--text-dim); font-size: 0.75rem; margin-top: 0.35rem;">
                  Automated OCSP stapling &amp; HTTP/2, HTTP/3 (QUIC) enabled by Caddy v2.
                </div>
              </div>
            </div>
          </div>

          <!-- SUB-TAB 8: REDIRECT -->
          <div id="modtab-redirect" class="mod-tab-content">
            <div class="mod-hint-box">
              Configure 301 (Permanent) or 302 (Temporary) redirects. Great for migrating old URLs, campaign links, or forwarding external domains.
            </div>
            <div style="display: flex; gap: 0.5rem; margin-top: 1rem; align-items: flex-end;">
              <div style="flex: 1;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;">Source Path</label>
                <input id="newRedirSource" class="form-input" placeholder="/old-path" style="font-family: var(--font-mono); font-size: 0.85rem;">
              </div>
              <div style="flex: 1.5;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;">Target URL</label>
                <input id="newRedirTarget" class="form-input" placeholder="https://example.com/new" style="font-family: var(--font-mono); font-size: 0.85rem;">
              </div>
              <div style="width: 100px;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;">HTTP Code</label>
                <select id="newRedirCode" class="form-select" style="font-size: 0.85rem;">
                  <option value="301">301 (Perm)</option>
                  <option value="302">302 (Temp)</option>
                </select>
              </div>
              <button class="btn btn-success" style="padding: 0.55rem 1rem;" onclick="addRedirectRule()">Add</button>
            </div>
            <div style="margin-top: 1.25rem; border: 1px solid var(--border); border-radius: 0.4rem; overflow: hidden;">
              <table style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                <thead>
                  <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.2); color: var(--text-dim); text-align: left;">
                    <th style="padding: 0.5rem 0.75rem;">Source</th>
                    <th style="padding: 0.5rem 0.75rem;">Target</th>
                    <th style="padding: 0.5rem 0.75rem; width: 60px;">Code</th>
                    <th style="padding: 0.5rem 0.75rem; text-align: right; width: 70px;">Operate</th>
                  </tr>
                </thead>
                <tbody id="modRedirectsTableBody"></tbody>
              </table>
            </div>
          </div>

          <!-- SUB-TAB 9: HOTLINK PROTECTION -->
          <div id="modtab-hotlink" class="mod-tab-content">
            <div class="toggle-row">
              <label for="modHotlinkToggle">
                Enable Anti-Leech / Hotlink Protection
                <span class="sub">Block other domains from embedding and stealing your images, media, and bandwidth</span>
              </label>
              <input type="checkbox" id="modHotlinkToggle" style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group" style="margin-top: 1rem;">
              <label>Protected Media Extensions</label>
              <input id="modHotlinkExts" class="form-input" style="font-family: var(--font-mono);" value="*.jpg *.jpeg *.png *.webp *.gif *.svg *.mp4 *.zip">
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                Space-separated glob patterns.
              </div>
            </div>
            <div class="mod-hint-box" style="margin-top: 0.75rem;">
              Caddy checks the HTTP <code>Referer</code> header against the virtual host domain. Direct requests without referrers and internal links remain permitted.
            </div>
          </div>

          <!-- SUB-TAB 10: MAINTENANCE MODE -->
          <div id="modtab-maintenance" class="mod-tab-content">
            <div class="toggle-row" style="border-left: 4px solid var(--yellow);">
              <label for="modMaintToggle">
                Site Maintenance Mode (HTTP 503)
                <span class="sub">Immediately returns 503 Service Unavailable for maintenance without removing vhost</span>
              </label>
              <input type="checkbox" id="modMaintToggle" style="accent-color: var(--yellow); transform: scale(1.3);">
            </div>
            <div class="mod-hint-box" style="border-color: rgba(245, 158, 11, 0.3); background: rgba(245, 158, 11, 0.05); color: #fde68a;">
              When maintenance mode is activated, Caddy intercepts all incoming traffic for this virtual host and cleanly returns an HTTP 503 response. Safe for software upgrades, database migrations, and emergencies.
            </div>
          </div>

          <!-- SUB-TAB 11: RESPONSE LOG -->
          <div id="modtab-log" class="mod-tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
              <div style="display: flex; gap: 0.5rem; align-items: center;">
                <select id="modLogType" class="form-select" style="width: 140px; padding: 0.3rem 0.6rem; font-size: 0.8rem;" onchange="loadModSiteLogs()">
                  <option value="access">Access Log</option>
                  <option value="error">Error Log</option>
                </select>
                <button class="btn btn-secondary" style="padding: 0.3rem 0.7rem; font-size: 0.75rem;" onclick="loadModSiteLogs()">Refresh</button>
              </div>
              <span style="font-size: 0.75rem; color: var(--text-dim); font-family: var(--font-mono);">/var/log/zpanl/&lt;domain&gt;.log</span>
            </div>
            <pre class="code-block" id="modLogViewer" style="height: 250px; margin: 0; line-height: 1.5; font-size: 0.78rem; overflow-y: auto;"></pre>
          </div>

          <!-- SUB-TAB 12: CONFIG (CADDY) -->
          <div id="modtab-config" class="mod-tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.6rem;">
              <span style="font-size: 0.8rem; color: var(--text-muted); font-weight: 600;">Active Virtual Host Caddyfile Block</span>
              <button class="btn btn-secondary" style="padding: 0.2rem 0.6rem; font-size: 0.75rem;" onclick="copyModCaddyfile()">Copy Config</button>
            </div>
            <pre class="code-block" id="modCaddyfilePreview" style="height: 240px; margin: 0;"></pre>
          </div>
        </div>
      </div>
      <div class="modal-footer" style="padding: 0.75rem 1.25rem;">
        <button class="btn btn-secondary" onclick="closeSiteModModal()">Close</button>
        <button class="btn" onclick="saveSiteModChanges()">Save &amp; Apply Changes</button>
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

    const tabTitles = {
      overview: 'Dashboard',
      sites: 'Websites & Virtual Hosts',
      files: 'File Manager',
      services: 'Systemd Services',
      caddy: 'Reverse Proxy Caddyfile',
      php: 'PHP-FPM Worker Pools'
    };

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
      document.querySelectorAll('#sidebar button').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      
      const navBtn = Array.from(document.querySelectorAll('#sidebar button')).find(b => b.textContent.trim().toLowerCase().includes(tab));
      if (navBtn) navBtn.classList.add('active');
      
      const targetContent = document.getElementById('tab-' + tab);
      if (targetContent) targetContent.classList.add('active');

      document.getElementById('breadcrumbTitle').textContent = tabTitles[tab] || 'Overview';

      if (tab === 'sites') loadSites();
      if (tab === 'files') initFilesTab();
      if (tab === 'services') loadServices();
      if (tab === 'caddy') loadCaddyfile();
      if (tab === 'php') initPhpTab();
    }

    // Telemetry Polling & Radial Gauges
    async function pollTelemetry() {
      try {
        const res = await fetch('/api/v1/telemetry');
        if (!res.ok) return;
        const d = await res.json();

        // Topbar
        document.getElementById('topbarCpu').textContent = d.cpu_usage_percent.toFixed(1) + '%';
        document.getElementById('topbarRam').textContent = `${d.ram_used_mb} / ${d.ram_total_mb} MB`;
        document.getElementById('topbarNet').textContent = `↓ ${d.net_rx_kbps} KB/s`;

        // CPU Radial Dial (circumference = 2 * pi * 36 ≈ 226)
        const cpuOffset = 226 - (226 * Math.min(d.cpu_usage_percent, 100)) / 100;
        document.getElementById('cpuRadial').style.strokeDashoffset = cpuOffset;
        document.getElementById('cpuRadialText').textContent = Math.round(d.cpu_usage_percent) + '%';
        document.getElementById('cpuDetailVal').textContent = d.cpu_usage_percent.toFixed(1) + '%';

        // RAM Radial Dial
        const ramOffset = 226 - (226 * Math.min(d.ram_usage_percent, 100)) / 100;
        document.getElementById('ramRadial').style.strokeDashoffset = ramOffset;
        document.getElementById('ramRadialText').textContent = Math.round(d.ram_usage_percent) + '%';
        document.getElementById('ramDetailVal').textContent = `${d.ram_used_mb} / ${d.ram_total_mb} MB`;
        document.getElementById('ramAvailText').textContent = `Available: ${d.ram_free_mb} MB`;

        // Network
        document.getElementById('netDetailRx').textContent = `↓ ${d.net_rx_kbps} KB/s`;
        document.getElementById('netDetailTx').textContent = `↑ ${d.net_tx_kbps} KB/s outbound`;

        // Sidebar & Uptime
        document.getElementById('uptimeQuickText').textContent = `Uptime: ${d.uptime_seconds}s`;
        document.getElementById('sidebarProcMode').textContent = d.is_linux_proc ? 'Linux /proc Native' : 'Local Dev Fallback';
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
        document.getElementById('navSitesCount').textContent = allSites.length;
        renderSitesTable(allSites);
        renderOverviewTable(allSites);
      } catch (e) {
        console.error(e);
      }
    }

    function renderSitesTable(sites) {
      const tbody = document.getElementById('sitesTableBody');
      if (!sites.length) {
        tbody.innerHTML = '<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">No virtual hosts registered yet. Click "+ Add Virtual Host" to start!</td></tr>';
        return;
      }

      tbody.innerHTML = sites.map(s => {
        let kindBadge = '<span class="badge badge-cyan">Static</span>';
        if (s.kind === 'reverse_proxy' || s.proxy_upstream) kindBadge = '<span class="badge badge-purple">Proxy</span>';
        else if (s.kind === 'spa_fallback') kindBadge = '<span class="badge badge-purple">SPA</span>';
        else if (s.kind === 'php_fpm') kindBadge = '<span class="badge badge-blue">PHP-FPM</span>';

        const maintBadge = s.maintenance ? '<span class="badge badge-yellow" style="margin-left: 0.35rem;">Maint (503)</span>' : '';
        const aliasCount = s.aliases && s.aliases.length ? `<span style="font-size: 0.72rem; color: var(--text-dim); display: block;">+${s.aliases.length} alias</span>` : '';
        const portStr = s.port && s.port !== 80 && s.port !== 443 ? `:${s.port}` : '';

        return `
          <tr>
            <td>
              <div style="font-weight: 700; color: var(--cyan-glow); font-size: 0.95rem; display: flex; align-items: center;">
                ${s.domain}${portStr}
                ${maintBadge}
              </div>
              ${aliasCount}
            </td>
            <td>${kindBadge}</td>
            <td style="font-family: var(--font-mono);">${s.php_version ? 'PHP ' + s.php_version : '<span style="color: var(--text-dim);">-</span>'}</td>
            <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-muted);">
              ${s.root_path}${s.running_dir ? '<span style="color: var(--cyan);">' + s.running_dir + '</span>' : ''}
            </td>
            <td><span class="badge badge-green">${s.ssl_enabled ? 'Auto HTTPS' : 'HTTP Only'}</span></td>
            <td style="text-align: right;">
              <div style="display: inline-flex; gap: 0.35rem;">
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem; border-color: rgba(56, 189, 248, 0.4); color: var(--cyan-glow);" onclick="openSiteModModal('${s.domain}')">⚙️ Config</button>
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="openSiteInFiles('${s.domain}')">Files</button>
                <button class="btn btn-danger" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="deleteSite('${s.domain}')">Del</button>
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
      tbody.innerHTML = sites.slice(0, 5).map(s => {
        const maintBadge = s.maintenance ? '<span class="badge badge-yellow" style="margin-left: 0.35rem;">Maint</span>' : '';
        return `
          <tr>
            <td style="font-weight: 700; color: var(--cyan-glow);">${s.domain}${maintBadge}</td>
            <td><span class="badge badge-cyan">${s.kind}</span></td>
            <td style="font-family: var(--font-mono);">${s.php_version ? 'PHP ' + s.php_version : '-'}</td>
            <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-muted);">${s.root_path}</td>
            <td><span class="badge badge-green">Active</span></td>
            <td style="text-align: right;">
              <button class="btn btn-secondary" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="openSiteModModal('${s.domain}')">⚙️ Config</button>
            </td>
          </tr>
        `;
      }).join('');
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

    // PHP Pools Tab
    async function initPhpTab() {
      const res = await fetch('/api/v1/sites');
      const sites = await res.json();
      const phpSites = sites.filter(s => s.kind === 'php_fpm');
      const sel = document.getElementById('phpSiteSelect');
      if (!phpSites.length) {
        sel.innerHTML = '<option value="">No PHP-FPM sites hosted</option>';
        document.getElementById('phpPoolConfigContent').textContent = 'No PHP-FPM sites configured. Create one in the Websites tab!';
        return;
      }
      sel.innerHTML = phpSites.map(s => `<option value="${s.domain}">🐘 ${s.domain} (PHP ${s.php_version || '8.2'})</option>`).join('');
      loadPhpPoolConfig();
    }

    async function loadPhpPoolConfig() {
      const domain = document.getElementById('phpSiteSelect').value;
      if (!domain) return;
      try {
        const res = await fetch(`/api/v1/php/pool?domain=${encodeURIComponent(domain)}`);
        if (res.ok) {
          document.getElementById('phpPoolConfigContent').textContent = await res.text();
        } else {
          document.getElementById('phpPoolConfigContent').textContent = 'Could not load pool: ' + await res.text();
        }
      } catch (e) {
        console.error(e);
      }
    }

    function formatBytes(bytes) {
      if (bytes === 0) return '-';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }

    // ============================================
    // AAPANEL-STYLE SITE MODIFICATION MODAL LOGIC
    // ============================================
    let currentModSite = null;

    function switchModTab(tabName) {
      document.querySelectorAll('.mod-tab-item').forEach(el => el.classList.remove('active'));
      document.querySelectorAll('.mod-tab-content').forEach(el => el.classList.remove('active'));
      
      const btn = document.getElementById(`btn-modtab-${tabName}`);
      if (btn) btn.classList.add('active');
      const content = document.getElementById(`modtab-${tabName}`);
      if (content) content.classList.add('active');

      if (tabName === 'config' && currentModSite) {
        loadModCaddyfile(currentModSite.domain);
      }
      if (tabName === 'log' && currentModSite) {
        loadModSiteLogs();
      }
    }

    async function openSiteModModal(domain) {
      try {
        const res = await fetch(`/api/v1/sites/detail?domain=${encodeURIComponent(domain)}`);
        if (!res.ok) {
          showToast('Failed to load site details', 'error');
          return;
        }
        currentModSite = await res.json();
        if (!currentModSite.aliases) currentModSite.aliases = [];
        if (!currentModSite.ip_blacklist) currentModSite.ip_blacklist = [];
        if (!currentModSite.redirects) currentModSite.redirects = [];

        document.getElementById('modSiteDomainTitle').textContent = currentModSite.domain;
        const d = new Date((currentModSite.created_at || 0) * 1000);
        document.getElementById('modSiteTimeTitle').textContent = d.toISOString().replace('T', ' ').substring(0, 19);

        // Tab 1: Domain Manager
        document.getElementById('modNewAliases').value = '';
        renderModDomainTable();

        // Tab 2: Directory
        document.getElementById('modRootPath').value = currentModSite.root_path || '';
        document.getElementById('modRunningDir').value = currentModSite.running_dir || '';

        // Tab 3: Limit access
        document.getElementById('modIpBlacklist').value = currentModSite.ip_blacklist.join('\n');
        const hasAuth = !!(currentModSite.basic_auth_user && currentModSite.basic_auth_pass);
        document.getElementById('modBasicAuthToggle').checked = hasAuth;
        document.getElementById('modAuthUser').value = currentModSite.basic_auth_user || '';
        document.getElementById('modAuthPass').value = currentModSite.basic_auth_pass || '';
        toggleBasicAuthFields();

        // Tab 4: URL rewrite
        document.getElementById('modRewritePreset').value = currentModSite.rewrite_preset || '';
        updateRewriteSnippetPreview();

        // Tab 5: PHP version
        if (currentModSite.kind === 'php_fpm') {
          document.getElementById('modPhpVersion').value = currentModSite.php_version || '8.2';
        } else {
          document.getElementById('modPhpVersion').value = 'none';
        }
        updatePhpSocketPreview();

        // Tab 6: Reverse proxy
        const isProxy = currentModSite.kind === 'reverse_proxy' || !!currentModSite.proxy_upstream;
        document.getElementById('modProxyToggle').checked = isProxy;
        document.getElementById('modProxyUpstream').value = currentModSite.proxy_upstream || '';

        // Tab 7: SSL
        document.getElementById('modSslToggle').checked = currentModSite.ssl_enabled !== false;

        // Tab 8: Redirects
        renderModRedirectsTable();
        document.getElementById('newRedirSource').value = '';
        document.getElementById('newRedirTarget').value = '';

        // Tab 9: Hotlink Protection
        document.getElementById('modHotlinkToggle').checked = !!currentModSite.hotlink_protection;
        document.getElementById('modHotlinkExts').value = currentModSite.hotlink_extensions || '*.jpg *.jpeg *.png *.webp *.gif *.svg *.mp4 *.zip';

        // Tab 10: Maintenance Mode
        document.getElementById('modMaintToggle').checked = !!currentModSite.maintenance;

        // Tab 11: Response log
        loadModSiteLogs();

        // Tab 12: Config
        loadModCaddyfile(currentModSite.domain);

        switchModTab('domain');
        document.getElementById('siteModModal').classList.add('active');
      } catch (e) {
        showToast(e.message, 'error');
      }
    }

    function closeSiteModModal() {
      document.getElementById('siteModModal').classList.remove('active');
    }

    function toggleBasicAuthFields() {
      const checked = document.getElementById('modBasicAuthToggle').checked;
      document.getElementById('basicAuthFields').style.display = checked ? 'grid' : 'none';
    }

    function renderModDomainTable() {
      if (!currentModSite) return;
      const tbody = document.getElementById('modDomainTableBody');
      let html = `
        <tr style="border-bottom: 1px solid var(--border);">
          <td style="padding: 0.6rem 0.75rem; font-weight: 600; color: var(--cyan-glow);">
            ${currentModSite.domain}
            <span class="badge badge-cyan" style="margin-left: 0.35rem; font-size: 0.65rem;">Primary</span>
          </td>
          <td style="padding: 0.6rem 0.75rem; font-family: var(--font-mono); color: var(--text-muted);">${currentModSite.port || 80}</td>
          <td style="padding: 0.6rem 0.75rem; text-align: right; color: var(--text-dim); font-size: 0.75rem;">Inoperable</td>
        </tr>
      `;

      (currentModSite.aliases || []).forEach((alias, idx) => {
        let port = currentModSite.port || 80;
        let name = alias;
        if (alias.includes(':')) {
          const parts = alias.split(':');
          name = parts[0];
          port = parts[1];
        }
        html += `
          <tr style="border-bottom: 1px solid var(--border);">
            <td style="padding: 0.6rem 0.75rem; font-family: var(--font-mono);">${name}</td>
            <td style="padding: 0.6rem 0.75rem; font-family: var(--font-mono); color: var(--text-muted);">${port}</td>
            <td style="padding: 0.6rem 0.75rem; text-align: right;">
              <button class="btn btn-danger" style="padding: 0.15rem 0.45rem; font-size: 0.7rem;" onclick="removeDomainAlias(${idx})">Del</button>
            </td>
          </tr>
        `;
      });

      tbody.innerHTML = html;
    }

    function addDomainAliases() {
      if (!currentModSite) return;
      const raw = document.getElementById('modNewAliases').value.trim();
      if (!raw) return;
      const lines = raw.split('\n').map(l => l.trim()).filter(l => l.length > 0);
      lines.forEach(line => {
        if (!currentModSite.aliases.includes(line) && line !== currentModSite.domain) {
          currentModSite.aliases.push(line);
        }
      });
      document.getElementById('modNewAliases').value = '';
      renderModDomainTable();
      showToast(`Added ${lines.length} domain alias(es). Remember to click Save & Apply!`, 'info');
    }

    function removeDomainAlias(index) {
      if (!currentModSite || !currentModSite.aliases) return;
      currentModSite.aliases.splice(index, 1);
      renderModDomainTable();
    }

    function renderModRedirectsTable() {
      if (!currentModSite) return;
      const tbody = document.getElementById('modRedirectsTableBody');
      if (!currentModSite.redirects || !currentModSite.redirects.length) {
        tbody.innerHTML = '<tr><td colspan="4" style="text-align: center; color: var(--text-muted); padding: 1rem;">No redirects configured.</td></tr>';
        return;
      }
      tbody.innerHTML = currentModSite.redirects.map((r, idx) => `
        <tr style="border-bottom: 1px solid var(--border);">
          <td style="padding: 0.5rem 0.75rem; font-family: var(--font-mono);">${r.source_path}</td>
          <td style="padding: 0.5rem 0.75rem; font-family: var(--font-mono); color: var(--cyan);">${r.target_url}</td>
          <td style="padding: 0.5rem 0.75rem; font-family: var(--font-mono); font-size: 0.8rem;"><span class="badge badge-purple">${r.code}</span></td>
          <td style="padding: 0.5rem 0.75rem; text-align: right;">
            <button class="btn btn-danger" style="padding: 0.15rem 0.45rem; font-size: 0.7rem;" onclick="removeRedirectRule(${idx})">Del</button>
          </td>
        </tr>
      `).join('');
    }

    function addRedirectRule() {
      if (!currentModSite) return;
      const src = document.getElementById('newRedirSource').value.trim();
      const tgt = document.getElementById('newRedirTarget').value.trim();
      const code = parseInt(document.getElementById('newRedirCode').value) || 301;
      if (!src || !tgt) {
        showToast('Please specify source path and target URL', 'error');
        return;
      }
      currentModSite.redirects.push({ source_path: src, target_url: tgt, code });
      document.getElementById('newRedirSource').value = '';
      document.getElementById('newRedirTarget').value = '';
      renderModRedirectsTable();
    }

    function removeRedirectRule(idx) {
      if (!currentModSite || !currentModSite.redirects) return;
      currentModSite.redirects.splice(idx, 1);
      renderModRedirectsTable();
    }

    async function loadModSiteLogs() {
      if (!currentModSite) return;
      const logType = document.getElementById('modLogType').value;
      const pre = document.getElementById('modLogViewer');
      pre.textContent = 'Loading logs...';
      try {
        const res = await fetch(`/api/v1/sites/logs?domain=${encodeURIComponent(currentModSite.domain)}&type=${logType}`);
        if (res.ok) {
          pre.textContent = await res.text();
          pre.scrollTop = pre.scrollHeight;
        } else {
          pre.textContent = 'Could not load log: ' + await res.text();
        }
      } catch (e) {
        pre.textContent = 'Error: ' + e.message;
      }
    }

    function updateRewriteSnippetPreview() {
      const preset = document.getElementById('modRewritePreset').value;
      const pre = document.getElementById('modRewritePreview');
      if (preset === 'laravel') {
        pre.textContent = `# Laravel / Symfony Clean URLs\nphp_fastcgi unix//run/php/php8.2-fpm.sock\nfile_server\ntry_files {path} {path}/ /index.php?{query}`;
      } else if (preset === 'wordpress') {
        pre.textContent = `# WordPress Core & Security Hardening\nphp_fastcgi unix//run/php/php8.2-fpm.sock\nfile_server\n@blocked {\n    path /wp-config.php /xmlrpc.php\n}\nrespond @blocked 403`;
      } else if (preset === 'spa') {
        pre.textContent = `# Single Page App (React, Vue, Vite)\ntry_files {path} /index.html\nfile_server`;
      } else {
        pre.textContent = `# Static File Server\nfile_server`;
      }
    }

    function updatePhpSocketPreview() {
      const ver = document.getElementById('modPhpVersion').value;
      const input = document.getElementById('modPhpSocketPreview');
      if (ver === 'none') {
        input.value = 'Static Mode - No PHP Worker Socket';
      } else {
        input.value = `unix//run/php/php${ver}-fpm.sock`;
      }
    }

    function openModSiteInFileManager() {
      if (!currentModSite) return;
      closeSiteModModal();
      openSiteInFiles(currentModSite.domain);
    }

    async function loadModCaddyfile(domain) {
      try {
        const res = await fetch(`/api/v1/sites/vhost?domain=${encodeURIComponent(domain)}`);
        if (res.ok) {
          document.getElementById('modCaddyfilePreview').textContent = await res.text();
        } else {
          document.getElementById('modCaddyfilePreview').textContent = '# Could not load vhost block: ' + await res.text();
        }
      } catch (e) {
        document.getElementById('modCaddyfilePreview').textContent = '# Error: ' + e.message;
      }
    }

    function copyModCaddyfile() {
      const text = document.getElementById('modCaddyfilePreview').textContent;
      navigator.clipboard.writeText(text);
      showToast('Virtual Host Caddyfile block copied to clipboard!', 'info');
    }

    async function saveSiteModChanges() {
      if (!currentModSite) return;
      try {
        const isProxy = document.getElementById('modProxyToggle').checked;
        const phpVer = document.getElementById('modPhpVersion').value;
        const rewrite = document.getElementById('modRewritePreset').value;

        let kind = 'static';
        if (isProxy) {
          kind = 'reverse_proxy';
        } else if (phpVer !== 'none') {
          kind = 'php_fpm';
        } else if (rewrite === 'spa') {
          kind = 'spa_fallback';
        }

        const ipList = document.getElementById('modIpBlacklist').value
          .split('\n')
          .map(s => s.trim())
          .filter(s => s.length > 0);

        const hasBasicAuth = document.getElementById('modBasicAuthToggle').checked;

        const payload = {
          domain: currentModSite.domain,
          aliases: currentModSite.aliases,
          port: currentModSite.port || 80,
          root_path: document.getElementById('modRootPath').value.trim() || currentModSite.root_path,
          running_dir: document.getElementById('modRunningDir').value || null,
          kind: kind,
          php_version: phpVer !== 'none' ? phpVer : null,
          proxy_upstream: isProxy ? document.getElementById('modProxyUpstream').value.trim() : null,
          rewrite_preset: rewrite || null,
          maintenance: document.getElementById('modMaintToggle').checked,
          ssl_enabled: document.getElementById('modSslToggle').checked,
          ip_blacklist: ipList,
          basic_auth_user: hasBasicAuth ? document.getElementById('modAuthUser').value.trim() || null : null,
          basic_auth_pass: hasBasicAuth ? document.getElementById('modAuthPass').value.trim() || null : null,
          hotlink_protection: document.getElementById('modHotlinkToggle').checked,
          hotlink_extensions: document.getElementById('modHotlinkExts').value.trim() || null,
          redirects: currentModSite.redirects || []
        };

        const res = await fetch('/api/v1/sites/update', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });

        if (!res.ok) {
          const err = await res.text();
          showToast('Update failed: ' + err, 'error');
          return;
        }

        showToast('✨ Site settings applied & Caddyfile reloaded in < 1ms!', 'success');
        closeSiteModModal();
        loadSites();
      } catch (e) {
        showToast(e.message, 'error');
      }
    }

    // Initial load
    loadSites();
  </script>
</body>
</html>
"###;
