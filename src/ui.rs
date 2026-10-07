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

    .file-drop-overlay {
      position: absolute;
      inset: 0;
      background: rgba(10, 15, 29, 0.92);
      backdrop-filter: blur(4px);
      z-index: 40;
      display: flex;
      align-items: center;
      justify-content: center;
      border: 2px dashed var(--cyan);
      border-radius: 0.5rem;
      pointer-events: none;
      transition: all 0.2s ease;
    }
    .file-drop-box {
      text-align: center;
      pointer-events: none;
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
        <div class="nav-group-title" data-i18n="nav_core">Core Management</div>
        <ul class="nav-list">
          <li class="nav-item">
            <button class="active" onclick="switchTab('overview')">
              <svg viewBox="0 0 24 24"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>
              <span data-i18n="nav_dashboard">Dashboard</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('sites')">
              <svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/></svg>
              <span data-i18n="nav_websites">Websites</span>
              <span class="nav-badge" id="navSitesCount">0</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('files')">
              <svg viewBox="0 0 24 24"><path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/></svg>
              <span data-i18n="nav_files">File Manager</span>
            </button>
          </li>
        </ul>
      </div>

      <!-- SYSTEM & SERVERS -->
      <div>
        <div class="nav-group-title" data-i18n="nav_services_group">Services & Engines</div>
        <ul class="nav-list">
          <li class="nav-item">
            <button onclick="switchTab('services')">
              <svg viewBox="0 0 24 24"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
              <span data-i18n="nav_services">Services</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('caddy')">
              <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
              <span data-i18n="nav_caddyfile">Caddyfile</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('php')">
              <svg viewBox="0 0 24 24"><path d="M4 4h16v16H4V4zm2 4v8h2v-3h2c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2H6zm2 2h2v2H8v-2zm7-2v8h2v-3h2c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2h-4zm2 2h2v2h-2v-2z"/></svg>
              <span data-i18n="nav_php_pools">PHP-FPM Pools</span>
            </button>
          </li>
        </ul>
      </div>

      <!-- DATA & AUTOMATION -->
      <div>
        <div class="nav-group-title" data-i18n="nav_data_group">Data &amp; Automation</div>
        <ul class="nav-list">
          <li class="nav-item">
            <button onclick="switchTab('databases')">
              <svg viewBox="0 0 24 24"><path d="M12 3C7.58 3 4 4.79 4 7v10c0 2.21 3.58 4 8 4s8-1.79 8-4V7c0-2.21-3.58-4-8-4zm0 2c3.87 0 6 1.5 6 2s-2.13 2-6 2-6-1.5-6-2 2.13-2 6-2zm6 5.27c-.72.48-1.89.96-3.4 1.25-.8.15-1.68.23-2.6.23s-1.8-.08-2.6-.23c-1.51-.29-2.68-.77-3.4-1.25V9.4c1.19.86 3.39 1.35 6 1.35s4.81-.49 6-1.35v1.87zm0 5c-.72.48-1.89.96-3.4 1.25-.8.15-1.68.23-2.6.23s-1.8-.08-2.6-.23c-1.51-.29-2.68-.77-3.4-1.25v-1.87c1.19.86 3.39 1.35 6 1.35s4.81-.49 6-1.35v1.87z"/></svg>
              <span data-i18n="nav_databases">Databases</span>
              <span class="nav-badge" id="navDatabasesCount">0</span>
            </button>
          </li>
          <li class="nav-item">
            <button onclick="switchTab('cron')">
              <svg viewBox="0 0 24 24"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg>
              <span data-i18n="nav_cron">Cron Tasks</span>
              <span class="nav-badge" id="navCronCount">0</span>
            </button>
          </li>
        </ul>
      </div>
    </div>

    <!-- SIDEBAR FOOTER -->
    <div class="sidebar-footer">
      <div class="server-status-pill">
        <span class="pulse-dot"></span>
        <span id="sidebarProcMode" data-i18n="nav_online">Linux /proc Active</span>
      </div>
      <div style="font-size: 0.7rem; color: var(--text-dim); margin-top: 0.25rem;" data-i18n="nav_footprint">
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
        <!-- LANGUAGE SWITCHER -->
        <button class="btn btn-secondary" id="langSwitchBtn" style="padding: 0.35rem 0.65rem; font-size: 0.78rem; display: flex; align-items: center; gap: 0.35rem;" onclick="toggleLanguage()">
          <span id="langFlag">🇻🇳</span> <span id="langText">Tiếng Việt</span>
        </button>
        <button class="btn" style="padding: 0.35rem 0.8rem; font-size: 0.78rem;" onclick="openAddSiteModal()">
          <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
          <span data-i18n="deploy_site">Deploy Site</span>
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
              <span class="card-title" data-i18n="card_cpu_title">CPU Utilization</span>
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
              <span class="card-title" data-i18n="card_ram_title">Memory Allocation</span>
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
              <span class="card-title" data-i18n="card_net_title">Network I/O</span>
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
              <span class="card-title" data-i18n="card_stack_title">Panel Sovereign Stack</span>
              <span class="card-icon">
                <svg viewBox="0 0 24 24"><path d="M12 2L1 21h22L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 16h2v2h-2zm0-6h2v4h-2z"/></svg>
              </span>
            </div>
            <div style="display: flex; flex-direction: column; justify-content: center; height: 80px;">
              <div style="font-weight: 800; font-size: 1.25rem; color: var(--cyan-glow);">Pure Rust + Caddy</div>
              <div class="radial-sub-val" style="margin-top: 0.35rem;">Single binary (&lt; 1 MB)</div>
            </div>
            <div style="font-size: 0.75rem; color: var(--green); display: flex; justify-content: space-between; margin-top: 0.5rem;">
              <span>Zero external deps</span>
              <span id="uptimeQuickText">Uptime: 0s</span>
            </div>
          </div>
        </div>

        <!-- RECENT SITES -->
        <div class="toolbar">
          <h3 class="toolbar-title" data-i18n="active_vhosts">Active Virtual Hosts</h3>
          <button class="btn btn-secondary" onclick="switchTab('sites')"><span data-i18n="view_all_sites">View All Websites &rarr;</span></button>
        </div>
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th data-i18n="th_domain">Domain Name</th>
                <th data-i18n="th_type">Type</th>
                <th data-i18n="th_php">PHP Version</th>
                <th data-i18n="th_root">Web Root</th>
                <th data-i18n="th_security">SSL Security</th>
                <th style="text-align: right;" data-i18n="th_actions">Action</th>
              </tr>
            </thead>
            <tbody id="overviewSitesTable">
              <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2rem;" data-i18n="loading_sites">Loading websites...</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 2: WEBSITES -->
      <div id="tab-sites" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title" data-i18n="sites_title">Websites & Virtual Hosts</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="sites_subtitle">Declarative Caddy v2 reverse proxy routing with automatic Let's Encrypt HTTPS</div>
          </div>
          <div class="toolbar-actions">
            <input type="text" id="siteSearchInput" class="search-input" placeholder="Search domain or path..." data-i18n-placeholder="search_placeholder" oninput="filterSitesTable()">
            <button class="btn" onclick="openAddSiteModal()">
              <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
              <span data-i18n="add_vhost_btn">Add Virtual Host</span>
            </button>
          </div>
        </div>
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th data-i18n="th_domain">Domain</th>
                <th data-i18n="th_type">Type</th>
                <th data-i18n="th_php">PHP Engine</th>
                <th data-i18n="th_root">Document Root</th>
                <th data-i18n="th_security">Security / SSL</th>
                <th style="text-align: right;" data-i18n="th_actions">Actions</th>
              </tr>
            </thead>
            <tbody id="sitesTableBody">
              <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;" data-i18n="loading_sites">Loading websites...</td></tr>
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
            <button class="btn btn-secondary" onclick="openFilePicker()">
              <svg viewBox="0 0 24 24"><path d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z"/></svg>
              <span data-i18n="upload_btn">Upload</span>
            </button>
            <input type="file" id="filePickerInput" multiple style="display:none" onchange="handlePickedFiles(event)">
            <button class="btn btn-secondary" onclick="openNewEntryModal(false)">
              <svg viewBox="0 0 24 24"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
              <span data-i18n="new_file_btn">New File</span>
            </button>
            <button class="btn btn-secondary" onclick="openNewEntryModal(true)">
              <svg viewBox="0 0 24 24"><path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-1 8h-3v3h-2v-3h-3v-2h3V9h2v3h3v2z"/></svg>
              <span data-i18n="new_folder_btn">New Folder</span>
            </button>
            <button class="btn btn-secondary" onclick="loadSiteFiles()">
              <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
              <span data-i18n="refresh_btn">Refresh</span>
            </button>
          </div>
        </div>
        <div class="table-container" id="filesDropZone" style="position: relative; min-height: 280px;">
          <!-- DRAG & DROP OVERLAY -->
          <div id="fileDropOverlay" class="file-drop-overlay" style="display: none;">
            <div class="file-drop-box">
              <svg viewBox="0 0 24 24" style="width: 52px; height: 52px; fill: var(--cyan); margin-bottom: 0.6rem;"><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z"/></svg>
              <div style="font-size: 1.15rem; font-weight: 700; color: var(--cyan-glow);" data-i18n="upload_drop_hint">Drop files or folders here to upload</div>
              <div style="font-size: 0.82rem; color: var(--text-muted); margin-top: 0.35rem;" id="dropZoneTargetSubpath">Target: /</div>
            </div>
          </div>
          <table>
            <thead>
              <tr>
                <th data-i18n="th_file_name">File Name</th>
                <th data-i18n="th_file_type">Type</th>
                <th data-i18n="th_file_size">Size</th>
                <th data-i18n="th_file_perm">POSIX Permissions</th>
                <th style="text-align: right;" data-i18n="th_actions">Actions</th>
              </tr>
            </thead>
            <tbody id="filesTableBody">
              <tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 2.5rem;" data-i18n="select_site_explore">Select a site to explore files.</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 4: SERVICES -->
      <div id="tab-services" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title" data-i18n="services_title">Linux Systemd Services</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="services_subtitle">Daemon process supervision via <code>zero-sys</code></div>
          </div>
          <button class="btn btn-secondary" onclick="loadServices()">
            <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
            <span data-i18n="refresh_daemons">Refresh Daemons</span>
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
            <h2 class="toolbar-title" data-i18n="caddy_title">Active Reverse Proxy Configuration</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="caddy_subtitle">Live synchronized from <code>/etc/caddy/Caddyfile</code></div>
          </div>
          <div class="toolbar-actions">
            <button class="btn btn-secondary" onclick="copyCaddyfile()" data-i18n="copy_caddyfile">Copy Caddyfile</button>
            <button class="btn" onclick="loadCaddyfile()" data-i18n="reload_preview">Reload Preview</button>
          </div>
        </div>
        <pre class="code-block" id="caddyfileContent">Loading Caddyfile...</pre>
      </div>

      <!-- TAB 6: PHP-FPM POOLS -->
      <div id="tab-php" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title" data-i18n="php_title">PHP-FPM Worker Pools</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="php_subtitle">Isolated on-demand fastcgi worker pools generated by <code>zero-fastcgi</code></div>
          </div>
        </div>
        <div class="card" style="margin-bottom: 1.5rem;">
          <h3 style="font-size: 1.05rem; margin-bottom: 0.5rem;" data-i18n="select_site_php">Select Site to Inspect PHP Pool Configuration</h3>
          <div style="display: flex; gap: 1rem; align-items: center; margin-top: 1rem;">
            <select id="phpSiteSelect" class="form-select" style="width: 320px;" onchange="loadPhpPoolConfig()"></select>
            <button class="btn btn-secondary" onclick="loadPhpPoolConfig()" data-i18n="view_pool_ini">View Pool INI</button>
          </div>
        </div>
        <pre class="code-block" id="phpPoolConfigContent">Select a PHP website above to inspect its pool.d/*.conf configuration.</pre>
      </div>

      <!-- TAB 7: DATABASES -->
      <div id="tab-databases" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title" data-i18n="databases_title">Database Management</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="databases_subtitle">Managed relational databases with one-click SQL dumps and access credentials</div>
          </div>
          <div class="toolbar-actions">
            <input type="text" id="dbSearchInput" class="search-input" placeholder="Search database..." data-i18n-placeholder="search_db_placeholder" oninput="filterDatabasesTable()">
            <button class="btn" onclick="openAddDatabaseModal()">
              <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
              <span data-i18n="add_db_btn">Add Database</span>
            </button>
          </div>
        </div>
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th data-i18n="th_db_name">Database Name</th>
                <th data-i18n="th_db_engine">Engine</th>
                <th data-i18n="th_db_user">Username</th>
                <th data-i18n="th_db_host">Access Host</th>
                <th data-i18n="th_db_site">Linked Site</th>
                <th data-i18n="th_db_size">Size</th>
                <th style="text-align: right;" data-i18n="th_actions">Actions</th>
              </tr>
            </thead>
            <tbody id="databasesTableBody">
              <tr><td colspan="7" style="text-align: center; color: var(--text-muted); padding: 2.5rem;" data-i18n="loading_databases">Loading databases...</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- TAB 8: CRON TASKS -->
      <div id="tab-cron" class="tab-content">
        <div class="toolbar">
          <div>
            <h2 class="toolbar-title" data-i18n="cron_title">Scheduled Tasks &amp; Crontab</h2>
            <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.2rem;" data-i18n="cron_subtitle">Precision recurring background automation, shell maintenance, and framework schedules</div>
          </div>
          <div class="toolbar-actions">
            <button class="btn" onclick="openAddCronModal()">
              <svg viewBox="0 0 24 24"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
              <span data-i18n="add_cron_btn">Add Cron Job</span>
            </button>
          </div>
        </div>
        <div class="table-container">
          <table>
            <thead>
              <tr>
                <th data-i18n="th_cron_name">Task Name</th>
                <th data-i18n="th_cron_schedule">Schedule</th>
                <th data-i18n="th_cron_command">Command</th>
                <th data-i18n="th_cron_status">Last Status</th>
                <th data-i18n="th_cron_last_run">Last Executed</th>
                <th style="text-align: right;" data-i18n="th_actions">Actions</th>
              </tr>
            </thead>
            <tbody id="cronTableBody">
              <tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;" data-i18n="loading_cron">Loading scheduled tasks...</td></tr>
            </tbody>
          </table>
        </div>
      </div>
    </main>
  </div>

  <!-- MODAL: ADD SITE -->
  <div id="addSiteModal" class="modal">
    <div class="modal-box">
      <div class="modal-header">
        <h3 data-i18n="modal_add_title">Deploy New Virtual Host</h3>
        <button class="modal-close" onclick="closeAddSiteModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label data-i18n="modal_domain_label">Fully Qualified Domain Name</label>
          <input id="newDomain" class="form-input" placeholder="e.g. blog.mydomain.com" oninput="autoSuggestRoot()">
        </div>
        <div class="form-group">
          <label data-i18n="modal_app_type">Application Type</label>
          <select id="newKind" class="form-select" onchange="togglePhpField()">
            <option value="static" data-i18n="type_static">Static HTML / CSS / JS / Assets</option>
            <option value="spa_fallback" data-i18n="type_spa">Single Page Application (SPA Fallback /index.html)</option>
            <option value="php_fpm" data-i18n="type_php">Dynamic PHP (PHP-FPM Unix Socket)</option>
          </select>
        </div>
        <div class="form-group" id="phpVersionGroup" style="display: none;">
          <label data-i18n="modal_php_ver">PHP-FPM Worker Version</label>
          <select id="newPhpVer" class="form-select">
            <option value="8.3">PHP 8.3-FPM (Latest)</option>
            <option value="8.2" selected>PHP 8.2-FPM (Recommended LTS)</option>
            <option value="8.1">PHP 8.1-FPM</option>
          </select>
        </div>
        <div class="form-group">
          <label data-i18n="modal_web_root">Document Web Root Path</label>
          <input id="newRoot" class="form-input" placeholder="/var/www/blog.mydomain.com">
        </div>
        <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.75rem;">
          <input type="checkbox" id="newSsl" checked disabled style="accent-color: var(--cyan);">
          <label for="newSsl" style="font-size: 0.85rem; color: var(--text-muted); cursor: default;" data-i18n="modal_auto_https">
            Automatic HTTPS via Caddy (Let's Encrypt / ZeroSSL)
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeAddSiteModal()" data-i18n="btn_cancel">Cancel</button>
        <button class="btn" onclick="submitCreateSite()" data-i18n="btn_create_vhost">Create Virtual Host</button>
      </div>
    </div>
  </div>

  <!-- MODAL: SITE MODIFICATION (aaPanel Style) -->
  <div id="siteModModal" class="modal">
    <div class="modal-box" style="max-width: 860px; width: 95%; height: 580px; max-height: 90vh; display: flex; flex-direction: column;">
      <div class="modal-header" style="padding: 0.85rem 1.25rem;">
        <div style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.95rem; font-weight: 600;">
          <svg viewBox="0 0 24 24" style="width: 17px; height: 17px; fill: var(--cyan);"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>
          <span><span data-i18n="mod_title_prefix">Site modification</span> [<span id="modSiteDomainTitle" style="color: var(--cyan-glow);">domain.com</span>] -- <span data-i18n="mod_time_added">Time added</span> [<span id="modSiteTimeTitle" style="color: var(--text-dim); font-size: 0.8rem;">2026-10-07</span>]</span>
        </div>
        <button class="modal-close" onclick="closeSiteModModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 0; display: flex; flex: 1; overflow: hidden;">
        <!-- LEFT SUB-SIDEBAR -->
        <div class="mod-sidebar">
          <div class="mod-tab-item active" id="btn-modtab-domain" onclick="switchModTab('domain')" data-i18n="mod_tab_domain">Domain Manager</div>
          <div class="mod-tab-item" id="btn-modtab-directory" onclick="switchModTab('directory')" data-i18n="mod_tab_directory">Directory</div>
          <div class="mod-tab-item" id="btn-modtab-limit" onclick="switchModTab('limit')" data-i18n="mod_tab_limit">Limit access</div>
          <div class="mod-tab-item" id="btn-modtab-rewrite" onclick="switchModTab('rewrite')" data-i18n="mod_tab_rewrite">URL rewrite</div>
          <div class="mod-tab-item" id="btn-modtab-php" onclick="switchModTab('php')" data-i18n="mod_tab_php">PHP version</div>
          <div class="mod-tab-item" id="btn-modtab-proxy" onclick="switchModTab('proxy')" data-i18n="mod_tab_proxy">Reverse proxy</div>
          <div class="mod-tab-item" id="btn-modtab-ssl" onclick="switchModTab('ssl')" data-i18n="mod_tab_ssl">SSL</div>
          <div class="mod-tab-item" id="btn-modtab-redirect" onclick="switchModTab('redirect')" data-i18n="mod_tab_redirect">Redirect</div>
          <div class="mod-tab-item" id="btn-modtab-hotlink" onclick="switchModTab('hotlink')" data-i18n="mod_tab_hotlink">Hotlink Protection</div>
          <div class="mod-tab-item" id="btn-modtab-maintenance" onclick="switchModTab('maintenance')" data-i18n="mod_tab_maintenance">Maintenance Mode</div>
          <div class="mod-tab-item" id="btn-modtab-log" onclick="switchModTab('log')" data-i18n="mod_tab_log">Response log</div>
          <div class="mod-tab-item" id="btn-modtab-waf" onclick="switchModTab('waf')" data-i18n="mod_tab_waf">WAF &amp; Shield</div>
          <div class="mod-tab-item" id="btn-modtab-deploy" onclick="switchModTab('deploy')" data-i18n="mod_tab_deploy">Git-Ops Deploy</div>
          <div class="mod-tab-item" id="btn-modtab-config" onclick="switchModTab('config')" data-i18n="mod_tab_config">Config (Caddy)</div>
        </div>
        <!-- RIGHT SUB-CONTENT -->
        <div class="mod-content">
          <!-- SUB-TAB 1: DOMAIN MANAGER -->
          <div id="modtab-domain" class="mod-tab-content active">
            <div class="mod-hint-box" data-i18n="mod_hint_domain">
              A domain per line, the default port is 80.<br>
              Wildcard domain format: *.domain.com<br>
              To add another port, the format is www.domain.com:88
            </div>
            <div style="display: flex; gap: 0.75rem; margin-top: 1rem;">
              <textarea id="modNewAliases" class="form-input" style="flex: 1; height: 75px; font-family: var(--font-mono); font-size: 0.85rem;" placeholder="alias1.domain.com&#10;alias2.domain.com:8080"></textarea>
              <button class="btn btn-success" style="align-self: flex-start; padding: 0.6rem 1.25rem;" onclick="addDomainAliases()" data-i18n="btn_add">Add</button>
            </div>
            <div style="margin-top: 1.25rem; border: 1px solid var(--border); border-radius: 0.4rem; overflow: hidden;">
              <table style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                <thead>
                  <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.2); color: var(--text-dim); text-align: left;">
                    <th style="padding: 0.5rem 0.75rem;" data-i18n="th_domain_name">Domain name</th>
                    <th style="padding: 0.5rem 0.75rem; width: 80px;" data-i18n="th_port">Port</th>
                    <th style="padding: 0.5rem 0.75rem; text-align: right; width: 100px;" data-i18n="th_operate">Operate</th>
                  </tr>
                </thead>
                <tbody id="modDomainTableBody"></tbody>
              </table>
            </div>
          </div>

          <!-- SUB-TAB 2: DIRECTORY -->
          <div id="modtab-directory" class="mod-tab-content">
            <div class="form-group">
              <label data-i18n="mod_base_dir">Site Base Directory</label>
              <div style="display: flex; gap: 0.5rem;">
                <input id="modRootPath" class="form-input" style="font-family: var(--font-mono);" placeholder="/var/www/domain.com">
                <button class="btn btn-secondary" onclick="openModSiteInFileManager()" data-i18n="mod_btn_files">Files &rarr;</button>
              </div>
            </div>
            <div class="form-group">
              <label data-i18n="mod_running_dir">Running Directory (Sub-path / Web Root)</label>
              <select id="modRunningDir" class="form-select">
                <option value="">/ (Root Directory - Standard)</option>
                <option value="/public">/public (Laravel, Symfony, ThinkPHP)</option>
                <option value="/dist">/dist (Vite, Vue, React Production Build)</option>
                <option value="/build">/build (Webpack, Next.js Static Export)</option>
              </select>
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;" data-i18n="mod_running_dir_hint">
                Point to framework public folder to keep vendor / .env secure.
              </div>
            </div>
            <div class="form-group">
              <label data-i18n="mod_dir_ownership">Directory Ownership &amp; Permission</label>
              <div style="background: var(--surface-elevated); padding: 0.75rem 1rem; border-radius: 0.45rem; font-family: var(--font-mono); font-size: 0.85rem; border: 1px solid var(--border); color: var(--cyan-glow);">
                User: www-data:www-data | Permissions: 755 (Directories) / 644 (Files)
              </div>
            </div>
          </div>

          <!-- SUB-TAB 3: LIMIT ACCESS -->
          <div id="modtab-limit" class="mod-tab-content">
            <div class="form-group">
              <label data-i18n="mod_ip_blacklist">IP Address Blacklist (CIDR notation supported)</label>
              <textarea id="modIpBlacklist" class="form-input" style="height: 100px; font-family: var(--font-mono); font-size: 0.85rem;" placeholder="192.168.1.50&#10;10.0.0.0/8&#10;172.16.0.0/12"></textarea>
              <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;" data-i18n="mod_ip_hint">
                One IP or CIDR per line. Any connection matching will immediately receive HTTP 403 Forbidden.
              </div>
            </div>
            <div class="toggle-row" style="margin-top: 1rem;">
              <label for="modBasicAuthToggle">
                <span data-i18n="mod_basic_auth">HTTP Basic Authentication</span>
                <span class="sub" data-i18n="mod_basic_auth_desc">Require username and password before granting access to website</span>
              </label>
              <input type="checkbox" id="modBasicAuthToggle" style="accent-color: var(--cyan); transform: scale(1.3);" onchange="toggleBasicAuthFields()">
            </div>
            <div id="basicAuthFields" style="display: none; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.5rem;">
              <div class="form-group">
                <label data-i18n="mod_auth_user">Auth Username</label>
                <input id="modAuthUser" class="form-input" placeholder="admin">
              </div>
              <div class="form-group">
                <label data-i18n="mod_auth_pass">Auth Password</label>
                <input id="modAuthPass" type="password" class="form-input" placeholder="••••••••">
              </div>
            </div>
          </div>

          <!-- SUB-TAB 4: URL REWRITE -->
          <div id="modtab-rewrite" class="mod-tab-content">
            <div class="form-group">
              <label data-i18n="mod_rewrite_preset">Framework URL Rewrite Preset</label>
              <select id="modRewritePreset" class="form-select" onchange="updateRewriteSnippetPreview()">
                <option value="">Default (Static File Server)</option>
                <option value="laravel">Laravel / Symfony (try_files {path} {path}/ /index.php?{query})</option>
                <option value="wordpress">WordPress / WooCommerce (FastCGI + Security Deny rules)</option>
                <option value="spa">Single Page App (try_files {path} /index.html)</option>
              </select>
            </div>
            <div class="form-group">
              <label data-i18n="mod_rewrite_preview">Caddyfile Rewrite Snippet Preview</label>
              <pre class="code-block" id="modRewritePreview" style="height: 160px;"></pre>
            </div>
          </div>

          <!-- SUB-TAB 5: PHP VERSION -->
          <div id="modtab-php" class="mod-tab-content">
            <div class="form-group">
              <label data-i18n="mod_php_runtime">PHP-FPM Worker Runtime</label>
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
              <label data-i18n="mod_fastcgi_socket">FastCGI Socket Endpoint</label>
              <input id="modPhpSocketPreview" class="form-input" readonly style="font-family: var(--font-mono); color: var(--cyan-glow);">
            </div>
            <div class="mod-hint-box" data-i18n="mod_php_hint">
              ZPanl configures FastCGI with <code>pm = ondemand</code>, dynamically spinning up worker processes when HTTP requests arrive and terminating idle workers after 10s to keep RAM footprint &lt; 10 MB.
            </div>
          </div>

          <!-- SUB-TAB 6: REVERSE PROXY -->
          <div id="modtab-proxy" class="mod-tab-content">
            <div class="toggle-row">
              <label for="modProxyToggle">
                <span data-i18n="mod_enable_proxy">Enable Reverse Proxy</span>
                <span class="sub" data-i18n="mod_enable_proxy_desc">Forward all traffic to internal application server (Node, Go, Python, Docker)</span>
              </label>
              <input type="checkbox" id="modProxyToggle" style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group" style="margin-top: 1rem;">
              <label data-i18n="mod_upstream_target">Upstream Target (Host:Port or Unix Socket)</label>
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
                <span data-i18n="mod_auto_ssl">Automatic HTTPS (Let's Encrypt / ZeroSSL)</span>
                <span class="sub" data-i18n="mod_auto_ssl_desc">Zero-configuration automated ACME certificate issuance and renewal</span>
              </label>
              <input type="checkbox" id="modSslToggle" checked style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group">
              <label data-i18n="mod_tls_strict">SSL / TLS Protocol Strictness</label>
              <div style="background: var(--surface-elevated); padding: 0.75rem 1rem; border-radius: 0.45rem; border: 1px solid var(--border); font-size: 0.85rem;">
                <div style="color: var(--green-glow); font-weight: 600; display: flex; align-items: center; gap: 0.5rem;">
                  <span style="display:inline-block;width:8px;height:8px;background:var(--green);border-radius:50%;"></span>
                  <span data-i18n="mod_tls_status">TLS 1.2 &amp; TLS 1.3 Modern Cipher Suite Active</span>
                </div>
                <div style="color: var(--text-dim); font-size: 0.75rem; margin-top: 0.35rem;">
                  Automated OCSP stapling &amp; HTTP/2, HTTP/3 (QUIC) enabled by Caddy v2.
                </div>
              </div>
            </div>
          </div>

          <!-- SUB-TAB 8: REDIRECT -->
          <div id="modtab-redirect" class="mod-tab-content">
            <div class="mod-hint-box" data-i18n="mod_redir_hint">
              Configure 301 (Permanent) or 302 (Temporary) redirects. Great for migrating old URLs, campaign links, or forwarding external domains.
            </div>
            <div style="display: flex; gap: 0.5rem; margin-top: 1rem; align-items: flex-end;">
              <div style="flex: 1;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;" data-i18n="mod_redir_src">Source Path</label>
                <input id="newRedirSource" class="form-input" placeholder="/old-path" style="font-family: var(--font-mono); font-size: 0.85rem;">
              </div>
              <div style="flex: 1.5;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;" data-i18n="mod_redir_tgt">Target URL</label>
                <input id="newRedirTarget" class="form-input" placeholder="https://example.com/new" style="font-family: var(--font-mono); font-size: 0.85rem;">
              </div>
              <div style="width: 100px;">
                <label style="font-size: 0.75rem; color: var(--text-muted); font-weight: 700;" data-i18n="mod_redir_code">HTTP Code</label>
                <select id="newRedirCode" class="form-select" style="font-size: 0.85rem;">
                  <option value="301">301 (Perm)</option>
                  <option value="302">302 (Temp)</option>
                </select>
              </div>
              <button class="btn btn-success" style="padding: 0.55rem 1rem;" onclick="addRedirectRule()" data-i18n="btn_add">Add</button>
            </div>
            <div style="margin-top: 1.25rem; border: 1px solid var(--border); border-radius: 0.4rem; overflow: hidden;">
              <table style="width: 100%; border-collapse: collapse; font-size: 0.85rem;">
                <thead>
                  <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.2); color: var(--text-dim); text-align: left;">
                    <th style="padding: 0.5rem 0.75rem;" data-i18n="th_source">Source</th>
                    <th style="padding: 0.5rem 0.75rem;" data-i18n="th_target">Target</th>
                    <th style="padding: 0.5rem 0.75rem; width: 60px;" data-i18n="th_code">Code</th>
                    <th style="padding: 0.5rem 0.75rem; text-align: right; width: 70px;" data-i18n="th_operate">Operate</th>
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
                <span data-i18n="mod_hotlink_title">Enable Anti-Leech / Hotlink Protection</span>
                <span class="sub" data-i18n="mod_hotlink_desc">Block other domains from embedding and stealing your images, media, and bandwidth</span>
              </label>
              <input type="checkbox" id="modHotlinkToggle" style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>
            <div class="form-group" style="margin-top: 1rem;">
              <label data-i18n="mod_hotlink_exts">Protected Media Extensions</label>
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
                <span data-i18n="mod_maint_title">Site Maintenance Mode (HTTP 503)</span>
                <span class="sub" data-i18n="mod_maint_desc">Immediately returns 503 Service Unavailable for maintenance without removing vhost</span>
              </label>
              <input type="checkbox" id="modMaintToggle" style="accent-color: var(--yellow); transform: scale(1.3);">
            </div>
            <div class="mod-hint-box" style="border-color: rgba(245, 158, 11, 0.3); background: rgba(245, 158, 11, 0.05); color: #fde68a;" data-i18n="mod_maint_box">
              When maintenance mode is activated, Caddy intercepts all incoming traffic for this virtual host and cleanly returns an HTTP 503 response. Safe for software upgrades, database migrations, and emergencies.
            </div>
          </div>

          <!-- SUB-TAB 11: RESPONSE LOG -->
          <div id="modtab-log" class="mod-tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
              <div style="display: flex; gap: 0.5rem; align-items: center;">
                <select id="modLogType" class="form-select" style="width: 140px; padding: 0.3rem 0.6rem; font-size: 0.8rem;" onchange="loadModSiteLogs()">
                  <option value="access" data-i18n="mod_log_access">Access Log</option>
                  <option value="error" data-i18n="mod_log_error">Error Log</option>
                </select>
                <button class="btn btn-secondary" style="padding: 0.3rem 0.7rem; font-size: 0.75rem;" onclick="loadModSiteLogs()" data-i18n="refresh_btn">Refresh</button>
              </div>
              <span style="font-size: 0.75rem; color: var(--text-dim); font-family: var(--font-mono);">/var/log/zpanl/&lt;domain&gt;.log</span>
            </div>
            <pre class="code-block" id="modLogViewer" style="height: 250px; margin: 0; line-height: 1.5; font-size: 0.78rem; overflow-y: auto;"></pre>
          </div>

          <!-- SUB-TAB 13: WAF & SHIELD -->
          <div id="modtab-waf" class="mod-tab-content">
            <div class="toggle-row" style="border-left: 4px solid var(--purple);">
              <label for="modWafToggle">
                <span data-i18n="waf_master_title">Web Application Firewall (WAF) Master Shield</span>
                <span class="sub" data-i18n="waf_master_desc">Activate heuristic threat mitigation, layer-7 filter, and bot defense</span>
              </label>
              <input type="checkbox" id="modWafToggle" style="accent-color: var(--purple); transform: scale(1.3);">
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 1rem;">
              <div class="card" style="padding: 0.75rem; background: var(--surface-elevated);">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                  <label for="modBadBotToggle" style="font-size: 0.85rem; font-weight: 600; cursor: pointer;">
                    <span data-i18n="waf_bad_bots">Bad Bot &amp; Scraper Shield</span>
                    <span style="display: block; font-size: 0.72rem; color: var(--text-dim);" data-i18n="waf_bad_bots_desc">Block ByteSpider, Ahrefs, Semrush, MJ12bot</span>
                  </label>
                  <input type="checkbox" id="modBadBotToggle" checked style="accent-color: var(--cyan); transform: scale(1.2);">
                </div>
              </div>

              <div class="card" style="padding: 0.75rem; background: var(--surface-elevated);">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                  <label for="modSqliXssToggle" style="font-size: 0.85rem; font-weight: 600; cursor: pointer;">
                    <span data-i18n="waf_sqli_xss">Heuristic SQLi / XSS Filter</span>
                    <span style="display: block; font-size: 0.72rem; color: var(--text-dim);" data-i18n="waf_sqli_xss_desc">Block query injection, traversal, eval patterns</span>
                  </label>
                  <input type="checkbox" id="modSqliXssToggle" checked style="accent-color: var(--cyan); transform: scale(1.2);">
                </div>
              </div>
            </div>

            <div class="toggle-row" style="margin-top: 1rem;">
              <label for="modRateLimitToggle">
                <span data-i18n="waf_rate_limit_title">Client Request Velocity Limiting</span>
                <span class="sub" data-i18n="waf_rate_limit_desc">Mitigate DDoS surges and brute-force credential stuffing</span>
              </label>
              <input type="checkbox" id="modRateLimitToggle" style="accent-color: var(--cyan); transform: scale(1.3);">
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.5rem;">
              <div class="form-group">
                <label style="font-size: 0.78rem;" data-i18n="waf_max_reqs">Max Requests per Client</label>
                <input id="modRateLimitRequests" type="number" class="form-input" value="60" min="5" max="10000">
              </div>
              <div class="form-group">
                <label style="font-size: 0.78rem;" data-i18n="waf_window">Time Window</label>
                <select id="modRateLimitWindow" class="form-select">
                  <option value="10s">10 seconds</option>
                  <option value="30s">30 seconds</option>
                  <option value="1m" selected>1 minute (Standard)</option>
                  <option value="5m">5 minutes</option>
                  <option value="1h">1 hour</option>
                </select>
              </div>
            </div>

            <div class="form-group" style="margin-top: 0.75rem;">
              <label style="font-size: 0.78rem;" data-i18n="waf_custom_agents">Custom Blocked User-Agents (One per line)</label>
              <textarea id="modCustomBlockedAgents" class="form-input" style="height: 60px; font-family: var(--font-mono); font-size: 0.8rem;" placeholder="curl&#10;python-requests&#10;Go-http-client"></textarea>
            </div>
          </div>

          <!-- SUB-TAB 14: GIT-OPS DEPLOY -->
          <div id="modtab-deploy" class="mod-tab-content">
            <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 0.75rem;">
              <div class="form-group">
                <label style="font-size: 0.78rem;" data-i18n="deploy_repo_url">Git Repository URL (HTTPS or SSH)</label>
                <input id="modDeployRepoUrl" class="form-input" placeholder="https://github.com/org/repo.git" style="font-family: var(--font-mono); font-size: 0.82rem;">
              </div>
              <div class="form-group">
                <label style="font-size: 0.78rem;" data-i18n="deploy_branch">Target Branch</label>
                <input id="modDeployBranch" class="form-input" value="main" style="font-family: var(--font-mono); font-size: 0.82rem;">
              </div>
            </div>

            <div class="form-group" style="margin-top: 0.5rem;">
              <label style="font-size: 0.78rem;" data-i18n="deploy_webhook_label">Automated Webhook Endpoint (GitHub / GitLab / Gitea)</label>
              <div style="display: flex; gap: 0.5rem;">
                <input id="modDeployWebhookUrl" class="form-input" readonly style="font-family: var(--font-mono); font-size: 0.78rem; color: var(--cyan-glow); background: var(--surface-elevated);">
                <button class="btn btn-secondary" style="padding: 0.3rem 0.7rem; font-size: 0.75rem;" onclick="copyWebhookUrl()" data-i18n="copy_caddyfile">Copy</button>
              </div>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.5rem;">
              <div class="toggle-row" style="padding: 0.5rem 0.75rem;">
                <label for="modDeploySymlinkToggle">
                  <span style="font-size: 0.82rem;" data-i18n="deploy_symlink">Zero-Downtime Atomic Symlink</span>
                  <span class="sub" style="font-size: 0.7rem;" data-i18n="deploy_symlink_desc">Releases /current pattern</span>
                </label>
                <input type="checkbox" id="modDeploySymlinkToggle" checked style="accent-color: var(--cyan); transform: scale(1.15);">
              </div>
              <div class="toggle-row" style="padding: 0.5rem 0.75rem;">
                <label for="modDeployAutoToggle">
                  <span style="font-size: 0.82rem;" data-i18n="deploy_auto">Auto-Deploy on Push</span>
                  <span class="sub" style="font-size: 0.7rem;" data-i18n="deploy_auto_desc">Webhook triggers build</span>
                </label>
                <input type="checkbox" id="modDeployAutoToggle" checked style="accent-color: var(--cyan); transform: scale(1.15);">
              </div>
            </div>

            <div class="form-group" style="margin-top: 0.6rem;">
              <div style="display: flex; justify-content: space-between; align-items: center;">
                <label style="font-size: 0.78rem;" data-i18n="deploy_build_script">Post-Deploy Hook Command (POSIX Shell / PowerShell)</label>
                <div style="display: flex; gap: 0.35rem;">
                  <button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="setDeployPreset('laravel')">Laravel</button>
                  <button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="setDeployPreset('vite')">Node / Vite</button>
                  <button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="setDeployPreset('static')">Static</button>
                </div>
              </div>
              <textarea id="modDeployBuildScript" class="form-input" style="height: 65px; font-family: var(--font-mono); font-size: 0.8rem;" placeholder="composer install --no-dev --optimize-autoloader&#10;php artisan migrate --force"></textarea>
            </div>

            <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 0.75rem;">
              <button class="btn btn-secondary" style="padding: 0.4rem 0.85rem; font-size: 0.8rem;" onclick="saveSiteDeployConfig()" data-i18n="save_deploy_cfg">Save Git Config</button>
              <button class="btn" id="btnTriggerDeploy" style="padding: 0.4rem 1.1rem; font-size: 0.82rem;" onclick="triggerSiteDeploy()">
                🚀 <span data-i18n="btn_deploy_now">Deploy Now</span>
              </button>
            </div>

            <!-- HISTORY TABLE -->
            <div style="margin-top: 1rem;">
              <div style="font-size: 0.8rem; font-weight: 700; color: var(--text-muted); margin-bottom: 0.4rem;" data-i18n="deploy_recent_history">Recent Deployment Releases</div>
              <div style="border: 1px solid var(--border); border-radius: 0.4rem; overflow: hidden; max-height: 140px; overflow-y: auto;">
                <table style="width: 100%; border-collapse: collapse; font-size: 0.78rem;">
                  <thead>
                    <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.2); color: var(--text-dim); text-align: left;">
                      <th style="padding: 0.4rem 0.6rem;">Release</th>
                      <th style="padding: 0.4rem 0.6rem;">Commit</th>
                      <th style="padding: 0.4rem 0.6rem;">Status</th>
                      <th style="padding: 0.4rem 0.6rem;">Time</th>
                      <th style="padding: 0.4rem 0.6rem; text-align: right;">Action</th>
                    </tr>
                  </thead>
                  <tbody id="modDeployHistoryTableBody">
                    <tr><td colspan="5" style="text-align: center; color: var(--text-dim); padding: 1rem;">No deployments yet.</td></tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>

          <!-- SUB-TAB 12: CONFIG (CADDY) -->
          <div id="modtab-config" class="mod-tab-content">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.6rem;">
              <span style="font-size: 0.8rem; color: var(--text-muted); font-weight: 600;" data-i18n="mod_active_block">Active Virtual Host Caddyfile Block</span>
              <button class="btn btn-secondary" style="padding: 0.2rem 0.6rem; font-size: 0.75rem;" onclick="copyModCaddyfile()" data-i18n="mod_copy_config">Copy Config</button>
            </div>
            <pre class="code-block" id="modCaddyfilePreview" style="height: 240px; margin: 0;"></pre>
          </div>
        </div>
      </div>
      <div class="modal-footer" style="padding: 0.75rem 1.25rem;">
        <button class="btn btn-secondary" onclick="closeSiteModModal()" data-i18n="btn_close">Close</button>
        <button class="btn" onclick="saveSiteModChanges()" data-i18n="btn_save_apply">Save &amp; Apply Changes</button>
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
          <button class="btn btn-secondary" onclick="closeEditorModal()" data-i18n="btn_cancel">Cancel</button>
          <button class="btn" onclick="saveFileContent()" data-i18n="save_changes_btn">Save Changes (Atomic)</button>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: NEW ENTRY -->
  <div id="newEntryModal" class="modal">
    <div class="modal-box" style="max-width: 440px;">
      <div class="modal-header">
        <h3 id="newEntryTitle" data-i18n="modal_new_item">Create New Item</h3>
        <button class="modal-close" onclick="closeNewEntryModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label id="newEntryLabel" data-i18n="item_name">Item Name</label>
          <input id="newEntryName" class="form-input" placeholder="e.g. index.php">
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeNewEntryModal()" data-i18n="btn_cancel">Cancel</button>
        <button class="btn" onclick="submitCreateEntry()" data-i18n="create_btn">Create</button>
      </div>
    </div>
  </div>

  <!-- MODAL: ADD DATABASE -->
  <div id="addDatabaseModal" class="modal">
    <div class="modal-box" style="max-width: 520px;">
      <div class="modal-header">
        <h3 data-i18n="modal_add_db_title">Create Relational Database</h3>
        <button class="modal-close" onclick="closeAddDatabaseModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label data-i18n="modal_db_name">Database Name</label>
          <input id="newDbName" class="form-input" placeholder="e.g. blog_db">
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem;">
          <div class="form-group">
            <label data-i18n="modal_db_engine">Database Engine</label>
            <select id="newDbEngine" class="form-select">
              <option value="mysql">MySQL 8.0</option>
              <option value="mariadb">MariaDB 10.11 LTS</option>
              <option value="sqlite">SQLite 3 (Embedded)</option>
              <option value="postgres">PostgreSQL 16</option>
            </select>
          </div>
          <div class="form-group">
            <label data-i18n="modal_db_collation">Collation</label>
            <select id="newDbCollation" class="form-select">
              <option value="utf8mb4_unicode_ci" selected>utf8mb4_unicode_ci</option>
              <option value="utf8mb4_general_ci">utf8mb4_general_ci</option>
              <option value="utf8_general_ci">utf8_general_ci</option>
            </select>
          </div>
        </div>
        <div class="form-group">
          <label data-i18n="modal_db_user">Username</label>
          <input id="newDbUser" class="form-input" value="root">
        </div>
        <div class="form-group">
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <label data-i18n="modal_db_pass">Password</label>
            <a href="javascript:void(0)" style="font-size: 0.75rem; color: var(--cyan);" onclick="generateRandomPassword()" data-i18n="btn_generate_pass">Generate 16-char Key</a>
          </div>
          <input id="newDbPass" class="form-input" placeholder="••••••••••••••••">
        </div>
        <div class="form-group">
          <label data-i18n="modal_db_host">Access Permission</label>
          <select id="newDbHost" class="form-select">
            <option value="127.0.0.1" selected>127.0.0.1 (Localhost only - Secure)</option>
            <option value="%">% (Any remote host / External clients)</option>
          </select>
        </div>
        <div class="form-group">
          <label data-i18n="modal_db_site">Linked Website (Optional)</label>
          <select id="newDbSite" class="form-select"></select>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeAddDatabaseModal()" data-i18n="btn_cancel">Cancel</button>
        <button class="btn" onclick="submitCreateDatabase()" data-i18n="btn_create_db">Create Database</button>
      </div>
    </div>
  </div>

  <!-- MODAL: ADD CRON JOB -->
  <div id="addCronModal" class="modal">
    <div class="modal-box" style="max-width: 560px;">
      <div class="modal-header">
        <h3 data-i18n="modal_add_cron_title">Schedule Recurring Task</h3>
        <button class="modal-close" onclick="closeAddCronModal()">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label data-i18n="modal_cron_name">Task Name</label>
          <input id="newCronName" class="form-input" placeholder="e.g. Laravel Schedule Worker">
        </div>
        <div class="form-group">
          <label data-i18n="modal_cron_preset">Schedule Preset</label>
          <select id="newCronPreset" class="form-select" onchange="onCronPresetChange()">
            <option value="* * * * *">Every Minute (* * * * *)</option>
            <option value="0 * * * *">Every Hour (0 * * * *)</option>
            <option value="0 0 * * *">Daily at Midnight (0 0 * * *)</option>
            <option value="0 0 * * 0">Weekly on Sunday (0 0 * * 0)</option>
            <option value="0 0 1 * *">Monthly on the 1st (0 0 1 * *)</option>
            <option value="*/15 * * * *">WordPress Cron (Every 15 mins)</option>
            <option value="custom">Custom Expression...</option>
          </select>
        </div>
        <div class="form-group">
          <label data-i18n="modal_cron_expr">Cron Expression (Min Hour Day Month Week)</label>
          <input id="newCronSchedule" class="form-input" value="* * * * *" style="font-family: var(--font-mono); color: var(--cyan-glow);">
        </div>
        <div class="form-group">
          <label data-i18n="modal_cron_cmd">Execute Command</label>
          <textarea id="newCronCommand" class="form-input" style="height: 75px; font-family: var(--font-mono); font-size: 0.85rem;" placeholder="php /var/www/site/artisan schedule:run >> /dev/null 2>&1"></textarea>
          <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;" data-i18n="cron_cmd_hint">
            Standard POSIX shell command executed directly on host environment.
          </div>
        </div>
        <div class="form-group">
          <label data-i18n="modal_cron_site">Associated Website (Optional)</label>
          <select id="newCronSite" class="form-select"></select>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick="closeAddCronModal()" data-i18n="btn_cancel">Cancel</button>
        <button class="btn" onclick="submitCreateCronJob()" data-i18n="btn_create_cron">Add Scheduled Task</button>
      </div>
    </div>
  </div>

  <!-- MODAL: CRON LOGS -->
  <div id="cronLogsModal" class="modal">
    <div class="modal-box" style="max-width: 720px; width: 95%;">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <svg viewBox="0 0 24 24" style="width: 17px; height: 17px; fill: var(--cyan);"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
          <span data-i18n="modal_cron_log_title">Task Execution Output Log</span>
        </div>
        <button class="modal-close" onclick="closeCronLogsModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 0;">
        <pre id="cronLogContent" class="code-block" style="height: 320px; margin: 0; line-height: 1.5; font-size: 0.8rem; overflow-y: auto;">Loading logs...</pre>
      </div>
      <div class="modal-footer" style="display: flex; justify-content: space-between;">
        <button class="btn btn-secondary" onclick="refreshCronLogs()" data-i18n="refresh_btn">Refresh</button>
        <button class="btn" onclick="closeCronLogsModal()" data-i18n="btn_close">Close</button>
      </div>
    </div>
  </div>

  <!-- MODAL: DEPLOY LOGS -->
  <div id="deployLogsModal" class="modal">
    <div class="modal-box" style="max-width: 760px; width: 95%;">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <svg viewBox="0 0 24 24" style="width: 17px; height: 17px; fill: var(--cyan);"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>
          <span data-i18n="modal_deploy_log_title">Deployment Build &amp; Release Log</span>
        </div>
        <button class="modal-close" onclick="closeDeployLogsModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 0;">
        <pre id="deployLogContent" class="code-block" style="height: 340px; margin: 0; line-height: 1.5; font-size: 0.8rem; overflow-y: auto;">Loading release logs...</pre>
      </div>
      <div class="modal-footer">
        <button class="btn" onclick="closeDeployLogsModal()" data-i18n="btn_close">Close</button>
      </div>
    </div>
  </div>

  <!-- MODAL: UPLOAD FILES (AAPANEL STYLE DRAG & DROP QUEUE) -->
  <div id="uploadFilesModal" class="modal">
    <div class="modal-box" style="max-width: 820px; width: 95%;">
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <svg viewBox="0 0 24 24" style="width: 18px; height: 18px; fill: var(--cyan);"><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z"/></svg>
          <span data-i18n="upload_modal_title">Upload File</span>
        </div>
        <button class="modal-close" onclick="closeUploadFilesModal()">&times;</button>
      </div>
      <div class="modal-body" style="padding: 1rem 1.25rem;">
        <!-- Top Green Summary Banner -->
        <div style="background: rgba(34, 197, 94, 0.12); border: 1px solid rgba(34, 197, 94, 0.35); border-radius: 9999px; padding: 0.55rem 1.25rem; display: flex; justify-content: space-between; align-items: center; font-size: 0.82rem; font-weight: 600; color: #4ade80; margin-bottom: 1rem; flex-wrap: wrap; gap: 0.5rem;">
          <span><span data-i18n="upload_size">Upload Size</span>: <span id="uploadSummarySize" style="font-family: var(--font-mono);">0 B / 0 B</span></span>
          <span><span data-i18n="avg_speed">Average Speed</span>: <span id="uploadSummarySpeed" style="font-family: var(--font-mono);">0 KB/s</span></span>
          <span><span data-i18n="upload_success">Upload Success</span>: <span id="uploadSummaryCount" style="font-family: var(--font-mono);">0 / 0</span></span>
        </div>

        <!-- Files Queue Table -->
        <div style="border: 1px solid var(--border); border-radius: 0.45rem; overflow: hidden; max-height: 320px; overflow-y: auto;">
          <table style="width: 100%; border-collapse: collapse; font-size: 0.8rem;">
            <thead>
              <tr style="border-bottom: 1px solid var(--border); background: rgba(0,0,0,0.25); color: var(--text-dim); text-align: left;">
                <th style="padding: 0.5rem 0.75rem;" data-i18n="th_file_name">File Name</th>
                <th style="padding: 0.5rem 0.75rem; width: 110px;" data-i18n="th_file_size">File Size</th>
                <th style="padding: 0.5rem 0.75rem; width: 150px;" data-i18n="th_upload_status">Upload Status</th>
                <th style="padding: 0.5rem 0.75rem; width: 90px; text-align: right;" data-i18n="th_operate">Operation</th>
              </tr>
            </thead>
            <tbody id="uploadQueueTableBody">
              <tr><td colspan="4" style="text-align: center; color: var(--text-dim); padding: 2rem;" data-i18n="upload_empty_queue">No files in queue. Drag &amp; drop files here or click Add Files.</td></tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="modal-footer" style="display: flex; justify-content: space-between; align-items: center;">
        <button class="btn btn-secondary" onclick="openFilePicker()" style="font-size: 0.8rem;">
          <svg viewBox="0 0 24 24" style="width: 14px; height: 14px; fill: currentColor; margin-right: 0.3rem;"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
          <span data-i18n="btn_add_files">+ Add More Files</span>
        </button>
        <div style="display: flex; gap: 0.5rem;">
          <button class="btn btn-secondary" onclick="closeUploadFilesModal()" data-i18n="btn_close">Close</button>
          <button class="btn" id="btnContinueUpload" onclick="startUploadQueue()" style="background: #16a34a; border-color: #22c55e; color: #fff; font-weight: 600;" data-i18n="btn_continue_upload">Continue Upload</button>
        </div>
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
    let allDatabases = [];
    let allCronJobs = [];
    let currentViewingCronId = null;

    let currentLang = localStorage.getItem('zpanl_lang') || 'vi';

    const I18N = {
      en: {
        nav_core: 'Core Management',
        nav_dashboard: 'Dashboard',
        nav_websites: 'Websites',
        nav_files: 'File Manager',
        nav_services_group: 'Services & Engines',
        nav_services: 'Services',
        nav_caddyfile: 'Caddyfile',
        nav_php_pools: 'PHP-FPM Pools',
        nav_data_group: 'Data & Automation',
        nav_databases: 'Databases',
        nav_cron: 'Cron Tasks',
        nav_online: 'Linux /proc Native',
        nav_footprint: 'Footprint: &lt; 10 MB RAM',
        deploy_site: 'Deploy Site',
        card_cpu_title: 'CPU Utilization',
        card_ram_title: 'Memory Allocation',
        card_net_title: 'Network I/O',
        card_stack_title: 'Panel Sovereign Stack',
        active_vhosts: 'Active Virtual Hosts',
        view_all_sites: 'View All Websites &rarr;',
        th_domain: 'Domain Name',
        th_type: 'Type',
        th_php: 'PHP Version',
        th_root: 'Web Root',
        th_security: 'SSL Security',
        th_actions: 'Actions',
        sites_title: 'Websites & Virtual Hosts',
        sites_subtitle: "Declarative Caddy v2 reverse proxy routing with automatic Let's Encrypt HTTPS",
        search_placeholder: 'Search domain or path...',
        add_vhost_btn: 'Add Virtual Host',
        loading_sites: 'Loading websites...',
        no_sites: 'No virtual hosts registered yet. Click "+ Add Virtual Host" to start!',
        new_file_btn: 'New File',
        new_folder_btn: 'New Folder',
        refresh_btn: 'Refresh',
        th_file_name: 'File Name',
        th_file_type: 'Type',
        th_file_size: 'Size',
        th_file_perm: 'POSIX Permissions',
        select_site_explore: 'Select a site to explore files.',
        services_title: 'Linux Systemd Services',
        services_subtitle: 'Daemon process supervision via zero-sys',
        refresh_daemons: 'Refresh Daemons',
        caddy_title: 'Active Reverse Proxy Configuration',
        caddy_subtitle: 'Live synchronized from /etc/caddy/Caddyfile',
        copy_caddyfile: 'Copy Caddyfile',
        reload_preview: 'Reload Preview',
        php_title: 'PHP-FPM Worker Pools',
        php_subtitle: 'Isolated on-demand fastcgi worker pools generated by zero-fastcgi',
        select_site_php: 'Select Site to Inspect PHP Pool Configuration',
        view_pool_ini: 'View Pool INI',
        databases_title: 'Database Management',
        databases_subtitle: 'Managed relational databases with one-click SQL dumps and access credentials',
        search_db_placeholder: 'Search database...',
        add_db_btn: 'Add Database',
        loading_databases: 'Loading databases...',
        no_databases: 'No databases created yet. Click "+ Add Database" to create one!',
        th_db_name: 'Database Name',
        th_db_engine: 'Engine',
        th_db_user: 'Username',
        th_db_host: 'Access Host',
        th_db_site: 'Linked Site',
        th_db_size: 'Size',
        modal_add_db_title: 'Create Relational Database',
        modal_db_name: 'Database Name',
        modal_db_engine: 'Database Engine',
        modal_db_collation: 'Collation',
        modal_db_user: 'Username',
        modal_db_pass: 'Password',
        btn_generate_pass: 'Generate 16-char Key',
        modal_db_host: 'Access Permission',
        modal_db_site: 'Linked Website (Optional)',
        btn_create_db: 'Create Database',
        btn_backup: 'Backup SQL',
        confirm_delete_db: "Are you sure you want to drop database '{name}'? This action cannot be undone.",
        db_created: "Database '{name}' created successfully!",
        db_deleted: "Database '{name}' deleted",
        db_backup_success: "SQL dump for '{name}' downloaded successfully",
        cron_title: 'Scheduled Tasks & Crontab',
        cron_subtitle: 'Precision recurring background automation, shell maintenance, and framework schedules',
        add_cron_btn: 'Add Cron Job',
        loading_cron: 'Loading scheduled tasks...',
        no_cron: 'No scheduled tasks registered. Click "+ Add Cron Job" to create one!',
        th_cron_name: 'Task Name',
        th_cron_schedule: 'Schedule',
        th_cron_command: 'Command',
        th_cron_status: 'Last Status',
        th_cron_last_run: 'Last Executed',
        modal_add_cron_title: 'Schedule Recurring Task',
        modal_cron_name: 'Task Name',
        modal_cron_preset: 'Schedule Preset',
        modal_cron_expr: 'Cron Expression (Min Hour Day Month Week)',
        modal_cron_cmd: 'Execute Command',
        cron_cmd_hint: 'Standard POSIX shell command executed directly on host environment.',
        modal_cron_site: 'Associated Website (Optional)',
        btn_create_cron: 'Add Scheduled Task',
        modal_cron_log_title: 'Task Execution Output Log',
        btn_run_now: 'Run Now',
        btn_logs: 'Logs',
        btn_enable: 'Enable',
        btn_disable: 'Disable',
        confirm_delete_cron: "Are you sure you want to delete task '{name}'?",
        cron_created: "Task '{name}' scheduled successfully!",
        cron_deleted: "Task '{name}' deleted",
        cron_triggered: "Task triggered! Check logs for real-time output.",
        modal_add_title: 'Deploy New Virtual Host',
        modal_domain_label: 'Fully Qualified Domain Name',
        modal_app_type: 'Application Type',
        type_static: 'Static HTML / CSS / JS / Assets',
        type_spa: 'Single Page Application (SPA Fallback /index.html)',
        type_php: 'Dynamic PHP (PHP-FPM Unix Socket)',
        modal_php_ver: 'PHP-FPM Worker Version',
        modal_web_root: 'Document Web Root Path',
        modal_auto_https: "Automatic HTTPS via Caddy (Let's Encrypt / ZeroSSL)",
        btn_cancel: 'Cancel',
        btn_create_vhost: 'Create Virtual Host',
        btn_close: 'Close',
        btn_save_apply: 'Save & Apply Changes',
        mod_title_prefix: 'Site modification',
        mod_time_added: 'Time added',
        mod_tab_domain: 'Domain Manager',
        mod_tab_directory: 'Directory',
        mod_tab_limit: 'Limit access',
        mod_tab_rewrite: 'URL rewrite',
        mod_tab_php: 'PHP version',
        mod_tab_proxy: 'Reverse proxy',
        mod_tab_ssl: 'SSL',
        mod_tab_redirect: 'Redirect',
        mod_tab_hotlink: 'Hotlink Protection',
        mod_tab_maintenance: 'Maintenance Mode',
        mod_tab_log: 'Response log',
        mod_tab_config: 'Config (Caddy)',
        mod_hint_domain: 'A domain per line, the default port is 80.<br>Wildcard domain format: *.domain.com<br>To add another port, the format is www.domain.com:88',
        btn_add: 'Add',
        th_domain_name: 'Domain name',
        th_port: 'Port',
        th_operate: 'Operate',
        mod_base_dir: 'Site Base Directory',
        mod_btn_files: 'Files &rarr;',
        mod_running_dir: 'Running Directory (Sub-path / Web Root)',
        mod_running_dir_hint: 'Point to framework public folder to keep vendor / .env secure.',
        mod_dir_ownership: 'Directory Ownership & Permission',
        mod_ip_blacklist: 'IP Address Blacklist (CIDR notation supported)',
        mod_ip_hint: 'One IP or CIDR per line. Any connection matching will immediately receive HTTP 403 Forbidden.',
        mod_basic_auth: 'HTTP Basic Authentication',
        mod_basic_auth_desc: 'Require username and password before granting access to website',
        mod_auth_user: 'Auth Username',
        mod_auth_pass: 'Auth Password',
        mod_rewrite_preset: 'Framework URL Rewrite Preset',
        mod_rewrite_preview: 'Caddyfile Rewrite Snippet Preview',
        mod_php_runtime: 'PHP-FPM Worker Runtime',
        mod_fastcgi_socket: 'FastCGI Socket Endpoint',
        mod_php_hint: 'ZPanl configures FastCGI with <code>pm = ondemand</code>, dynamically spinning up worker processes when HTTP requests arrive and terminating idle workers after 10s to keep RAM footprint &lt; 10 MB.',
        mod_enable_proxy: 'Enable Reverse Proxy',
        mod_enable_proxy_desc: 'Forward all traffic to internal application server (Node, Go, Python, Docker)',
        mod_upstream_target: 'Upstream Target (Host:Port or Unix Socket)',
        mod_auto_ssl: "Automatic HTTPS (Let's Encrypt / ZeroSSL)",
        mod_auto_ssl_desc: 'Zero-configuration automated ACME certificate issuance and renewal',
        mod_tls_strict: 'SSL / TLS Protocol Strictness',
        mod_tls_status: 'TLS 1.2 & TLS 1.3 Modern Cipher Suite Active',
        mod_redir_hint: 'Configure 301 (Permanent) or 302 (Temporary) redirects. Great for migrating old URLs, campaign links, or forwarding external domains.',
        mod_redir_src: 'Source Path',
        mod_redir_tgt: 'Target URL',
        mod_redir_code: 'HTTP Code',
        th_source: 'Source',
        th_target: 'Target',
        th_code: 'Code',
        mod_hotlink_title: 'Enable Anti-Leech / Hotlink Protection',
        mod_hotlink_desc: 'Block other domains from embedding and stealing your images, media, and bandwidth',
        mod_hotlink_exts: 'Protected Media Extensions',
        mod_maint_title: 'Site Maintenance Mode (HTTP 503)',
        mod_maint_desc: 'Immediately returns 503 Service Unavailable for maintenance without removing vhost',
        mod_maint_box: 'When maintenance mode is activated, Caddy intercepts all incoming traffic for this virtual host and cleanly returns an HTTP 503 response. Safe for software upgrades, database migrations, and emergencies.',
        mod_log_access: 'Access Log',
        mod_log_error: 'Error Log',
        mod_active_block: 'Active Virtual Host Caddyfile Block',
        mod_copy_config: 'Copy Config',
        save_changes_btn: 'Save Changes (Atomic)',
        modal_new_item: 'Create New Item',
        item_name: 'Item Name',
        create_btn: 'Create',
        btn_config: '⚙️ Config',
        btn_files: 'Files',
        btn_delete: 'Del',
        btn_edit: 'Edit',
        btn_download: 'Download',
        status_active: 'Active',
        status_stopped: 'Stopped',
        confirm_delete_site: "Are you sure you want to remove domain '{domain}' from ZPanl?",
        confirm_delete_file: "Are you sure you want to delete '{name}'?",
        site_created: "Virtual host '{domain}' created successfully!",
        site_deleted: "Site '{domain}' deleted",
        site_updated: '✨ Site settings applied & Caddyfile reloaded in < 1ms!',
        mod_tab_waf: 'WAF & Limiter',
        mod_tab_deploy: 'Git-Ops & Deploy',
        waf_title: 'Web Application Firewall (WAF) & Rate Limiting',
        waf_desc: 'Active heuristic exploit filtering and client request throttling via native Caddy engine',
        waf_bad_bots: 'Block Malicious Scrapers & Aggressive Crawlers',
        waf_bad_bots_desc: 'Heuristically rejects ByteSpider, MJ12bot, PetalBot, Semrush, Ahrefs with HTTP 403',
        waf_sqli_xss: 'SQL Injection & XSS Attack Shield',
        waf_sqli_xss_desc: 'Drops suspicious payload strings (UNION SELECT, <script>, eval, base64) at edge',
        waf_rate_limit: 'Request Velocity Rate Limiter',
        waf_rate_limit_desc: 'Mitigate brute-force, credential stuffing, and layer-7 denial of service',
        waf_max_req: 'Max Requests',
        waf_window: 'Time Window',
        waf_custom_agents: 'Custom Blocked User-Agents (One per line)',
        deploy_repo_url: 'Git Repository URL (HTTPS or SSH)',
        deploy_branch: 'Target Branch',
        deploy_webhook_label: 'Automated Webhook Endpoint (GitHub / GitLab / Gitea)',
        deploy_symlink: 'Zero-Downtime Atomic Symlink',
        deploy_symlink_desc: 'Releases /current pattern',
        deploy_auto: 'Auto-Deploy on Push',
        deploy_auto_desc: 'Webhook triggers build',
        deploy_build_script: 'Post-Deploy Hook Command (POSIX Shell / PowerShell)',
        save_deploy_cfg: 'Save Git Config',
        btn_deploy_now: 'Deploy Now',
        deploy_recent_history: 'Recent Deployment Releases',
        modal_deploy_log_title: 'Deployment Build & Release Log',
        upload_btn: 'Upload',
        upload_modal_title: 'Upload File',
        upload_drop_hint: 'Drop files or folders here to upload',
        upload_size: 'Upload Size',
        avg_speed: 'Average Speed',
        upload_success: 'Upload Success',
        th_upload_status: 'Upload Status',
        btn_add_files: '+ Add More Files',
        btn_continue_upload: 'Continue Upload',
        upload_waiting: 'Waiting',
        upload_uploading: 'Uploading...',
        upload_completed: 'Completed',
        upload_failed: 'Failed',
        upload_empty_queue: 'No files in queue. Drag & drop files here or click Add Files.',
        upload_all_done: 'All file(s) uploaded successfully!'
      },
      vi: {
        nav_core: 'Quản Lý Cốt Lõi',
        nav_dashboard: 'Tổng Quan',
        nav_websites: 'Website',
        nav_files: 'Quản Lý Tệp',
        nav_services_group: 'Dịch Vụ & Máy Chủ',
        nav_services: 'Dịch Vụ',
        nav_caddyfile: 'Cấu Hình Caddy',
        nav_php_pools: 'Cụm Worker PHP',
        nav_data_group: 'Dữ Liệu & Tự Động Hóa',
        nav_databases: 'Cơ Sở Dữ Liệu',
        nav_cron: 'Tác Vụ Định Kỳ',
        nav_online: 'Linux /proc Chuẩn',
        nav_footprint: 'Dung lượng: &lt; 10 MB RAM',
        deploy_site: 'Thêm Website',
        card_cpu_title: 'Tải CPU',
        card_ram_title: 'Bộ Nhớ RAM',
        card_net_title: 'Băng Thông Mạng',
        card_stack_title: 'Nền Tảng Sovereign',
        active_vhosts: 'Website Đang Chạy',
        view_all_sites: 'Xem tất cả Website &rarr;',
        th_domain: 'Tên Miền',
        th_type: 'Thể Loại',
        th_php: 'Phiên Bản PHP',
        th_root: 'Thư Mục Gốc',
        th_security: 'Chứng Chỉ SSL',
        th_actions: 'Thao Tác',
        sites_title: 'Danh Sách Website & Virtual Host',
        sites_subtitle: "Điều hướng reverse proxy Caddy v2 tốc độ cao với HTTPS tự động Let's Encrypt",
        search_placeholder: 'Tìm kiếm tên miền hoặc đường dẫn...',
        add_vhost_btn: 'Thêm Website Mới',
        loading_sites: 'Đang tải danh sách website...',
        no_sites: 'Chưa có website nào được đăng ký. Bấm "+ Thêm Website Mới" để bắt đầu!',
        new_file_btn: 'Tệp Mới',
        new_folder_btn: 'Thư Mục Mới',
        refresh_btn: 'Làm Mới',
        th_file_name: 'Tên Tệp Tin',
        th_file_type: 'Loại',
        th_file_size: 'Dung Lượng',
        th_file_perm: 'Quyền POSIX',
        select_site_explore: 'Chọn một website để duyệt tệp tin.',
        services_title: 'Dịch Vụ Hệ Thống Linux',
        services_subtitle: 'Giám sát tiến trình daemon hệ thống qua zero-sys',
        refresh_daemons: 'Làm Mới Tiến Trình',
        caddy_title: 'Cấu Hình Reverse Proxy Đang Chạy',
        caddy_subtitle: 'Đồng bộ trực tiếp thời gian thực từ /etc/caddy/Caddyfile',
        copy_caddyfile: 'Sao Chép Caddyfile',
        reload_preview: 'Tải Lại Cấu Hình',
        php_title: 'Cụm Worker PHP-FPM',
        php_subtitle: 'Các worker FastCGI theo yêu cầu tách biệt tạo bởi zero-fastcgi',
        select_site_php: 'Chọn Website Để Xem Cấu Hình PHP Pool',
        view_pool_ini: 'Xem Cấu Hình INI',
        databases_title: 'Quản Lý Cơ Sở Dữ Liệu',
        databases_subtitle: 'Quản lý cơ sở dữ liệu quan hệ, sao lưu .SQL 1-click và thông tin truy cập',
        search_db_placeholder: 'Tìm kiếm cơ sở dữ liệu...',
        add_db_btn: 'Thêm Cơ Sở Dữ Liệu',
        loading_databases: 'Đang tải danh sách CSDL...',
        no_databases: 'Chưa có CSDL nào được tạo. Bấm "+ Thêm Cơ Sở Dữ Liệu" để bắt đầu!',
        th_db_name: 'Tên CSDL',
        th_db_engine: 'Động Cơ',
        th_db_user: 'Tài Khoản',
        th_db_host: 'Quyền Truy Cập',
        th_db_site: 'Website Liên Kết',
        th_db_size: 'Dung Lượng',
        modal_add_db_title: 'Khởi Tạo Cơ Sở Dữ Liệu',
        modal_db_name: 'Tên Cơ Sở Dữ Liệu',
        modal_db_engine: 'Loại Động Cơ CSDL',
        modal_db_collation: 'Bảng Mã (Collation)',
        modal_db_user: 'Tài Khoản Người Dùng',
        modal_db_pass: 'Mật Khẩu',
        btn_generate_pass: 'Tạo ngẫu nhiên 16 ký tự',
        modal_db_host: 'Phạm Vi Truy Cập',
        modal_db_site: 'Website Liên Kết (Tùy chọn)',
        btn_create_db: 'Tạo CSDL',
        btn_backup: 'Sao Lưu SQL',
        confirm_delete_db: "Bạn có chắc chắn muốn xóa CSDL '{name}' không? Hành động này không thể hoàn tác.",
        db_created: "Cơ sở dữ liệu '{name}' đã được tạo thành công!",
        db_deleted: "Cơ sở dữ liệu '{name}' đã bị xóa",
        db_backup_success: "Đã tải xuống bản sao lưu SQL cho '{name}'",
        cron_title: 'Lập Lịch Tác Vụ & Crontab',
        cron_subtitle: 'Tự động hóa tác vụ nền định kỳ, bảo trì hệ thống và lịch chạy framework',
        add_cron_btn: 'Thêm Tác Vụ',
        loading_cron: 'Đang tải danh sách tác vụ...',
        no_cron: 'Chưa có tác vụ định kỳ nào. Bấm "+ Thêm Tác Vụ" để bắt đầu!',
        th_cron_name: 'Tên Tác Vụ',
        th_cron_schedule: 'Lịch Chạy',
        th_cron_command: 'Lệnh Thực Thi',
        th_cron_status: 'Trạng Thái Cuối',
        th_cron_last_run: 'Lần Chạy Cuối',
        modal_add_cron_title: 'Lập Lịch Tác Vụ Mới',
        modal_cron_name: 'Tên Tác Vụ',
        modal_cron_preset: 'Mẫu Định Kỳ Thường Dùng',
        modal_cron_expr: 'Biểu Thức Cron (Phút Giờ Ngày Tháng Thứ)',
        modal_cron_cmd: 'Câu Lệnh Thực Thi',
        cron_cmd_hint: 'Câu lệnh shell chuẩn được thực thi trực tiếp trên môi trường máy chủ.',
        modal_cron_site: 'Website Liên Kết (Tùy chọn)',
        btn_create_cron: 'Thêm Tác Vụ Định Kỳ',
        modal_cron_log_title: 'Nhật Ký Thực Thi Tác Vụ',
        btn_run_now: 'Chạy Ngay',
        btn_logs: 'Nhật Ký',
        btn_enable: 'Bật',
        btn_disable: 'Tắt',
        confirm_delete_cron: "Bạn có chắc chắn muốn xóa tác vụ '{name}' không?",
        cron_created: "Tác vụ '{name}' đã được thêm vào lịch trình!",
        cron_deleted: "Tác vụ '{name}' đã bị xóa",
        cron_triggered: "Tác vụ đã được kích hoạt! Kiểm tra nhật ký để xem kết quả.",
        modal_add_title: 'Khởi Tạo Website Mới',
        modal_domain_label: 'Tên Miền Đầy Đủ (FQDN)',
        modal_app_type: 'Thể Loại Ứng Dụng',
        type_static: 'Tĩnh (HTML / CSS / JS / Assets)',
        type_spa: 'Ứng Dụng SPA (Fallback /index.html)',
        type_php: 'PHP Động (Socket Unix PHP-FPM)',
        modal_php_ver: 'Phiên Bản PHP-FPM Worker',
        modal_web_root: 'Đường Dẫn Thư Mục Web Gốc',
        modal_auto_https: "Tự động cấp SSL HTTPS qua Caddy (Let's Encrypt / ZeroSSL)",
        btn_cancel: 'Hủy Bỏ',
        btn_create_vhost: 'Tạo Website',
        btn_close: 'Đóng',
        btn_save_apply: 'Lưu & Áp Dụng Thay Đổi',
        mod_title_prefix: 'Chỉnh sửa cấu hình Website',
        mod_time_added: 'Thời gian tạo',
        mod_tab_domain: 'Quản Lý Tên Miền',
        mod_tab_directory: 'Thư Mục Web',
        mod_tab_limit: 'Giới Hạn Truy Cập',
        mod_tab_rewrite: 'Viết Lại URL',
        mod_tab_php: 'Phiên Bản PHP',
        mod_tab_proxy: 'Reverse Proxy',
        mod_tab_ssl: 'Chứng Chỉ SSL',
        mod_tab_redirect: 'Chuyển Hướng',
        mod_tab_hotlink: 'Chống Hotlink',
        mod_tab_maintenance: 'Chế Độ Bảo Trì',
        mod_tab_log: 'Nhật Ký Truy Cập & Lỗi',
        mod_tab_config: 'Cấu Hình Caddy',
        mod_hint_domain: 'Mỗi dòng một tên miền, cổng mặc định là 80.<br>Định dạng wildcard: *.domain.com<br>Thêm cổng khác theo định dạng: www.domain.com:88',
        btn_add: 'Thêm',
        th_domain_name: 'Tên miền',
        th_port: 'Cổng',
        th_operate: 'Thao tác',
        mod_base_dir: 'Thư Mục Cơ Sở Website',
        mod_btn_files: 'Quản Lý Tệp &rarr;',
        mod_running_dir: 'Thư Mục Chạy (Thư Mục Con / Web Root)',
        mod_running_dir_hint: 'Trỏ vào thư mục public của framework để bảo vệ tệp tin vendor / .env.',
        mod_dir_ownership: 'Quyền Sở Hữu & Phân Quyền Thư Mục',
        mod_ip_blacklist: 'Danh Sách Đen IP (Hỗ trợ định dạng CIDR)',
        mod_ip_hint: 'Mỗi IP hoặc dải CIDR một dòng. Mọi kết nối trùng khớp sẽ lập tức nhận mã HTTP 403 Forbidden.',
        mod_basic_auth: 'Xác Thực HTTP Basic',
        mod_basic_auth_desc: 'Yêu cầu tài khoản và mật khẩu trước khi cấp quyền truy cập website',
        mod_auth_user: 'Tên Đăng Nhập',
        mod_auth_pass: 'Mật Khẩu',
        mod_rewrite_preset: 'Bộ Viết Lại URL Theo Framework',
        mod_rewrite_preview: 'Xem Trước Cấu Hình Caddyfile',
        mod_php_runtime: 'Môi Trường Thực Thi PHP-FPM',
        mod_fastcgi_socket: 'Điểm Cuối Unix Socket FastCGI',
        mod_php_hint: 'ZPanl cấu hình FastCGI với <code>pm = ondemand</code>, tự động khởi tạo worker khi có request và giải phóng sau 10s rảnh rỗi nhằm duy trì RAM &lt; 10 MB.',
        mod_enable_proxy: 'Kích Hoạt Reverse Proxy',
        mod_enable_proxy_desc: 'Chuyển tiếp toàn bộ lưu lượng tới máy chủ backend nội bộ (Node, Go, Python, Docker)',
        mod_upstream_target: 'Địa Chỉ Upstream (Host:Port hoặc Unix Socket)',
        mod_auto_ssl: "Tự Động Cấp SSL (Let's Encrypt / ZeroSSL)",
        mod_auto_ssl_desc: 'Tự động đăng ký và gia hạn chứng chỉ ACME hoàn toàn không cần cấu hình',
        mod_tls_strict: 'Mức Độ Bảo Mật SSL / TLS',
        mod_tls_status: 'Kích Hoạt Bộ Mã Hóa Hiện Đại TLS 1.2 & TLS 1.3',
        mod_redir_hint: 'Cấu hình chuyển hướng 301 (Vĩnh viễn) hoặc 302 (Tạm thời). Phù hợp khi đổi liên kết hoặc chuyển tiếp tên miền ngoài.',
        mod_redir_src: 'Đường Dẫn Gốc',
        mod_redir_tgt: 'URL Đích',
        mod_redir_code: 'Mã HTTP',
        th_source: 'Nguồn',
        th_target: 'Đích',
        th_code: 'Mã',
        mod_hotlink_title: 'Kích Hoạt Chống Lấy Trộm Liên Kết (Hotlink)',
        mod_hotlink_desc: 'Ngăn chặn trang web khác nhúng và tiêu tốn băng thông hình ảnh, video của bạn',
        mod_hotlink_exts: 'Đuôi Tệp Đang Được Bảo Vệ',
        mod_maint_title: 'Chế Độ Bảo Trì Website (HTTP 503)',
        mod_maint_desc: 'Lập tức trả về mã HTTP 503 Service Unavailable để bảo trì mà không cần xóa vhost',
        mod_maint_box: 'Khi bật chế độ bảo trì, Caddy chặn toàn bộ truy cập tới vhost này và trả về mã HTTP 503 sạch. An toàn cho nâng cấp mã nguồn, di chuyển CSDL hoặc xử lý sự cố.',
        mod_log_access: 'Nhật Ký Truy Cập',
        mod_log_error: 'Nhật Ký Lỗi',
        mod_active_block: 'Khối Cấu Hình Caddyfile Của Website Này',
        mod_copy_config: 'Sao Chép Cấu Hình',
        save_changes_btn: 'Lưu Thay Đổi (Atomic)',
        modal_new_item: 'Tạo Mục Mới',
        item_name: 'Tên Mục',
        create_btn: 'Tạo Mới',
        btn_config: '⚙️ Cấu Hình',
        btn_files: 'Tệp Tin',
        btn_delete: 'Xóa',
        btn_edit: 'Sửa',
        btn_download: 'Tải Về',
        status_active: 'Đang chạy',
        status_stopped: 'Đã dừng',
        confirm_delete_site: "Bạn có chắc chắn muốn xóa tên miền '{domain}' khỏi ZPanl không?",
        confirm_delete_file: "Bạn có chắc muốn xóa '{name}' không?",
        site_created: "Website '{domain}' đã được tạo thành công!",
        site_deleted: "Website '{domain}' đã bị xóa",
        site_updated: '✨ Cấu hình website đã được áp dụng & nạp lại Caddyfile trong < 1ms!',
        mod_tab_waf: 'Tường Lửa WAF',
        mod_tab_deploy: 'Git-Ops & Deploy',
        waf_title: 'Tường Lửa Ứng Dụng Web (WAF) & Giới Hạn Tốc Độ',
        waf_desc: 'Lọc tấn công tự động và giới hạn tần suất truy cập qua máy chủ Caddy',
        waf_bad_bots: 'Chặn Bot Độc Hại & Công Cụ Quét Dữ Liệu',
        waf_bad_bots_desc: 'Tự động chặn ByteSpider, MJ12bot, PetalBot, Semrush, Ahrefs với mã HTTP 403',
        waf_sqli_xss: 'Lá Chắn Chống SQL Injection & XSS',
        waf_sqli_xss_desc: 'Chặn ngay các chuỗi độc hại (UNION SELECT, <script>, eval, base64) từ rìa mạng',
        waf_rate_limit: 'Bộ Giới Hạn Tần Suất Truy Cập (Rate Limit)',
        waf_rate_limit_desc: 'Ngăn chặn tấn công dò quét mật khẩu và DoS tầng ứng dụng (Layer 7)',
        waf_max_req: 'Số Yêu Cầu Tối Đa',
        waf_window: 'Khung Thời Gian',
        waf_custom_agents: 'Chặn Thêm User-Agent Tự Chọn (Mỗi dòng một chuỗi)',
        deploy_repo_url: 'Đường Dẫn Kho Chứa Git (HTTPS hoặc SSH)',
        deploy_branch: 'Nhánh Cần Triển Khai',
        deploy_webhook_label: 'Điểm Cuối Webhook Tự Động (GitHub / GitLab / Gitea)',
        deploy_symlink: 'Triển Khai Atomic Không Gián Đoạn (Symlink)',
        deploy_symlink_desc: 'Cơ chế thư mục releases /current',
        deploy_auto: 'Tự Động Triển Khai Khi Có Push',
        deploy_auto_desc: 'Webhook kích hoạt quy trình build',
        deploy_build_script: 'Lệnh Hook Sau Triển Khai (Shell / PowerShell)',
        save_deploy_cfg: 'Lưu Cấu Hình Git',
        btn_deploy_now: 'Triển Khai Ngay',
        deploy_recent_history: 'Lịch Sử Triển Khai Gần Đây',
        modal_deploy_log_title: 'Nhật Ký Triển Khai & Bản Phát Hành',
        upload_btn: 'Tải Lên',
        upload_modal_title: 'Tải Lên Tệp Tin',
        upload_drop_hint: 'Kéo thả tệp tin hoặc thư mục vào đây để tải lên',
        upload_size: 'Dung Lượng',
        avg_speed: 'Tốc Độ TB',
        upload_success: 'Thành Công',
        th_upload_status: 'Trạng Thái',
        btn_add_files: '+ Thêm Tệp Tin',
        btn_continue_upload: 'Tiếp Tục Tải Lên',
        upload_waiting: 'Đang chờ',
        upload_uploading: 'Đang tải lên...',
        upload_completed: 'Hoàn thành',
        upload_failed: 'Thất bại',
        upload_empty_queue: 'Chưa có tệp nào trong hàng đợi. Kéo thả tệp vào đây hoặc bấm Thêm Tệp.',
        upload_all_done: 'Đã tải lên tất cả tệp tin thành công!'
      }
    };

    function t(key, fallback = '') {
      if (I18N[currentLang] && I18N[currentLang][key]) {
        return I18N[currentLang][key];
      }
      if (I18N['en'] && I18N['en'][key]) {
        return I18N['en'][key];
      }
      return fallback || key;
    }

    function toggleLanguage() {
      currentLang = currentLang === 'vi' ? 'en' : 'vi';
      localStorage.setItem('zpanl_lang', currentLang);
      updateLanguageUI();
      applyTranslations();
      if (currentTab === 'sites') renderSitesTable(allSites);
      if (currentTab === 'overview') renderOverviewTable(allSites);
      if (currentTab === 'files') loadSiteFiles();
      if (currentTab === 'services') loadServices();
      if (currentTab === 'databases') renderDatabasesTable(allDatabases);
      if (currentTab === 'cron') renderCronTable(allCronJobs);
    }

    function updateLanguageUI() {
      const flag = document.getElementById('langFlag');
      const text = document.getElementById('langText');
      if (currentLang === 'vi') {
        if (flag) flag.textContent = '🇻🇳';
        if (text) text.textContent = 'Tiếng Việt';
      } else {
        if (flag) flag.textContent = '🇬🇧';
        if (text) text.textContent = 'English';
      }
      const titleObj = tabTitles[currentTab];
      if (titleObj) {
        document.getElementById('breadcrumbTitle').textContent = titleObj[currentLang] || titleObj.en;
      }
    }

    function applyTranslations() {
      document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        const val = t(key);
        if (val) {
          if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
            el.placeholder = val;
          } else {
            el.innerHTML = val;
          }
        }
      });
      document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        const val = t(key);
        if (val) el.placeholder = val;
      });
    }

    const tabTitles = {
      overview: { en: 'Dashboard', vi: 'Bảng Điều Khiển' },
      sites: { en: 'Websites & Virtual Hosts', vi: 'Website & Virtual Host' },
      files: { en: 'File Manager', vi: 'Quản Lý Tệp Tin' },
      services: { en: 'Systemd Services', vi: 'Dịch Vụ Hệ Thống' },
      caddy: { en: 'Reverse Proxy Caddyfile', vi: 'Cấu Hình Caddyfile' },
      php: { en: 'PHP-FPM Worker Pools', vi: 'Cụm Worker PHP-FPM' },
      databases: { en: 'Database Management', vi: 'Quản Lý Cơ Sở Dữ Liệu' },
      cron: { en: 'Scheduled Tasks & Crontab', vi: 'Lập Lịch Tác Vụ & Crontab' }
    };

    function showToast(message, type = 'info') {
      const c = document.getElementById('toastContainer');
      const tEl = document.createElement('div');
      tEl.className = `toast toast-${type}`;
      tEl.textContent = message;
      c.appendChild(tEl);
      setTimeout(() => {
        tEl.style.opacity = '0';
        tEl.style.transition = 'opacity 0.3s';
        setTimeout(() => tEl.remove(), 300);
      }, 3500);
    }

    function switchTab(tab) {
      currentTab = tab;
      document.querySelectorAll('#sidebar button').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      
      const navBtn = document.querySelector(`#sidebar button[onclick*="'${tab}'"]`);
      if (navBtn) navBtn.classList.add('active');
      
      const targetContent = document.getElementById('tab-' + tab);
      if (targetContent) targetContent.classList.add('active');

      const titleObj = tabTitles[tab];
      document.getElementById('breadcrumbTitle').textContent = (titleObj && titleObj[currentLang]) || 'Overview';

      if (tab === 'sites') loadSites();
      if (tab === 'files') initFilesTab();
      if (tab === 'services') loadServices();
      if (tab === 'caddy') loadCaddyfile();
      if (tab === 'php') initPhpTab();
      if (tab === 'databases') loadDatabases();
      if (tab === 'cron') loadCronJobs();
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
        document.getElementById('ramAvailText').textContent = `${currentLang === 'vi' ? 'Khả dụng' : 'Available'}: ${d.ram_free_mb} MB`;

        // Network
        document.getElementById('netDetailRx').textContent = `↓ ${d.net_rx_kbps} KB/s`;
        document.getElementById('netDetailTx').textContent = `↑ ${d.net_tx_kbps} KB/s ${currentLang === 'vi' ? 'chiều gửi' : 'outbound'}`;

        // Sidebar & Uptime
        document.getElementById('uptimeQuickText').textContent = `${currentLang === 'vi' ? 'Thời gian chạy' : 'Uptime'}: ${d.uptime_seconds}s`;
        document.getElementById('sidebarProcMode').textContent = d.is_linux_proc ? (currentLang === 'vi' ? 'Linux /proc Chuẩn' : 'Linux /proc Native') : (currentLang === 'vi' ? 'Chế độ Dev Fallback' : 'Local Dev Fallback');
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
        tbody.innerHTML = `<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">${t('no_sites')}</td></tr>`;
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
        const sslLabel = s.ssl_enabled ? 'Auto HTTPS' : (currentLang === 'vi' ? 'Chỉ HTTP' : 'HTTP Only');

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
            <td><span class="badge badge-green">${sslLabel}</span></td>
            <td style="text-align: right;">
              <div style="display: inline-flex; gap: 0.35rem;">
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem; border-color: rgba(56, 189, 248, 0.4); color: var(--cyan-glow);" onclick="openSiteModModal('${s.domain}')">${t('btn_config')}</button>
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="openSiteInFiles('${s.domain}')">${t('btn_files')}</button>
                <button class="btn btn-danger" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="deleteSite('${s.domain}')">${t('btn_delete')}</button>
              </div>
            </td>
          </tr>
        `;
      }).join('');
    }

    function renderOverviewTable(sites) {
      const tbody = document.getElementById('overviewSitesTable');
      if (!sites.length) {
        tbody.innerHTML = `<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 1.5rem;">${t('no_sites')}</td></tr>`;
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
            <td><span class="badge badge-green">${t('status_active')}</span></td>
            <td style="text-align: right;">
              <button class="btn btn-secondary" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="openSiteModModal('${s.domain}')">${t('btn_config')}</button>
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
          showToast(t('site_created').replace('{domain}', domain), 'success');
          loadSites();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Network error: ' + e, 'error');
      }
    }

    async function deleteSite(domain) {
      if (!confirm(t('confirm_delete_site').replace('{domain}', domain))) return;
      try {
        const res = await fetch(`/api/v1/sites?domain=${encodeURIComponent(domain)}`, { method: 'DELETE' });
        if (res.ok) {
          showToast(t('site_deleted').replace('{domain}', domain), 'success');
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
      initFileDragAndDrop();
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
              📁 .. (${currentLang === 'vi' ? 'Quay lại thư mục cha' : 'Back to parent directory'})
            </td>
          </tr>`;
        }

        rows += entries.map(e => {
          const isDir = e.file_type === 'Directory';
          const icon = isDir ? '📁' : '📄';
          const clickAction = isDir
            ? `onclick="navigateDir('${e.name}')"`
            : `onclick="openEditor('${e.rel_path}')"`;
          const typeBadge = isDir 
            ? (currentLang === 'vi' ? 'Thư mục' : 'Directory')
            : (currentLang === 'vi' ? 'Tệp tin' : 'File');
          return `
            <tr>
              <td style="cursor: pointer; font-weight: 500;" ${clickAction}>
                <span style="margin-right: 0.5rem;">${icon}</span>
                <span style="${isDir ? 'color: var(--cyan-glow); font-weight: 600;' : ''}">${e.name}</span>
              </td>
              <td><span class="badge ${isDir ? 'badge-yellow' : 'badge-cyan'}">${typeBadge}</span></td>
              <td style="font-family: var(--font-mono);">${formatBytes(e.size_bytes)}</td>
              <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-dim);">0o${e.posix_mode.toString(8)}</td>
              <td style="text-align: right;">
                <div style="display: inline-flex; gap: 0.4rem;">
                  ${!isDir ? `<button class="btn btn-secondary" style="padding: 0.2rem 0.55rem; font-size: 0.75rem;" onclick="openEditor('${e.rel_path}')">${t('btn_edit')}</button>` : ''}
                  <button class="btn btn-danger" style="padding: 0.2rem 0.55rem; font-size: 0.75rem;" onclick="deleteFileItem('${e.rel_path}')">${t('btn_delete')}</button>
                </div>
              </td>
            </tr>
          `;
        }).join('');

        tbody.innerHTML = rows || `<tr><td colspan="5" style="text-align: center; color: var(--text-muted); padding: 2rem;">${currentLang === 'vi' ? 'Thư mục trống.' : 'Directory is empty.'}</td></tr>`;
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
        closeUploadFilesModal();
      }
    });

    // ============================================
    // FILE MANAGER DRAG & DROP AND UPLOAD QUEUE
    // ============================================
    let uploadQueue = []; // array of { id, file, relPath, size, status: 'waiting'|'uploading'|'completed'|'failed', progress: 0, error: null }
    let isUploading = false;
    let uploadStartTime = 0;

    function openFilePicker() {
      const picker = document.getElementById('filePickerInput');
      if (picker) {
        picker.value = '';
        picker.click();
      }
    }

    async function handlePickedFiles(event) {
      const files = event.target.files;
      if (!files || !files.length) return;
      const fileList = [];
      for (let i = 0; i < files.length; i++) {
        fileList.push({ file: files[i], relPath: files[i].name });
      }
      enqueueFilesForUpload(fileList);
    }

    let dragDropInitialized = false;
    function initFileDragAndDrop() {
      if (dragDropInitialized) return;
      const dropZone = document.getElementById('filesDropZone');
      const overlay = document.getElementById('fileDropOverlay');
      if (!dropZone || !overlay) return;

      let dragCounter = 0;

      ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        window.addEventListener(eventName, preventDefaults, false);
      });

      function preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
      }

      window.addEventListener('dragenter', (e) => {
        if (currentTab !== 'files') return;
        dragCounter++;
        const targetSub = currentSubpath ? `/${currentSubpath}` : '/';
        const subLabel = document.getElementById('dropZoneTargetSubpath');
        if (subLabel) subLabel.textContent = `Target: ${targetSub}`;
        overlay.style.display = 'flex';
      });

      window.addEventListener('dragover', (e) => {
        if (currentTab !== 'files') return;
        overlay.style.display = 'flex';
      });

      window.addEventListener('dragleave', (e) => {
        if (currentTab !== 'files') return;
        dragCounter--;
        if (dragCounter <= 0) {
          dragCounter = 0;
          overlay.style.display = 'none';
        }
      });

      window.addEventListener('drop', async (e) => {
        if (currentTab !== 'files') return;
        dragCounter = 0;
        overlay.style.display = 'none';
        const fileList = await getFilesFromDataTransfer(e.dataTransfer);
        if (fileList && fileList.length) {
          enqueueFilesForUpload(fileList);
        }
      });

      dragDropInitialized = true;
    }

    async function getFilesFromDataTransfer(dataTransfer) {
      const files = [];
      const items = dataTransfer.items;
      if (items && items.length > 0 && items[0].webkitGetAsEntry) {
        const queue = [];
        for (let i = 0; i < items.length; i++) {
          const entry = items[i].webkitGetAsEntry();
          if (entry) queue.push(traverseEntry(entry, ''));
        }
        await Promise.all(queue);
      } else if (dataTransfer.files) {
        for (let i = 0; i < dataTransfer.files.length; i++) {
          const f = dataTransfer.files[i];
          files.push({ file: f, relPath: f.name });
        }
      }

      async function traverseEntry(entry, path) {
        if (entry.isFile) {
          const f = await new Promise(resolve => entry.file(resolve));
          files.push({ file: f, relPath: path ? `${path}/${f.name}` : f.name });
        } else if (entry.isDirectory) {
          const dirReader = entry.createReader();
          const entries = await new Promise(resolve => dirReader.readEntries(resolve));
          for (const child of entries) {
            await traverseEntry(child, path ? `${path}/${entry.name}` : entry.name);
          }
        }
      }

      return files;
    }

    function enqueueFilesForUpload(fileList) {
      fileList.forEach(item => {
        uploadQueue.push({
          id: 'up_' + Math.random().toString(36).substring(2, 9),
          file: item.file,
          relPath: item.relPath,
          size: item.file.size,
          status: 'waiting',
          progress: 0,
          error: null
        });
      });

      renderUploadQueue();
      openUploadFilesModal();
      if (!isUploading) {
        startUploadQueue();
      }
    }

    function openUploadFilesModal() {
      document.getElementById('uploadFilesModal').classList.add('active');
    }

    function closeUploadFilesModal() {
      document.getElementById('uploadFilesModal').classList.remove('active');
      loadSiteFiles();
    }

    function renderUploadQueue() {
      const tbody = document.getElementById('uploadQueueTableBody');
      if (!uploadQueue.length) {
        tbody.innerHTML = `<tr><td colspan="4" style="text-align: center; color: var(--text-dim); padding: 2rem;">${t('upload_empty_queue')}</td></tr>`;
        updateUploadSummary();
        return;
      }

      tbody.innerHTML = uploadQueue.map(item => {
        let statusBadge = `<span class="badge" style="background: rgba(255,255,255,0.08); color: var(--text-dim);">${t('upload_waiting')}</span>`;
        if (item.status === 'completed') {
          statusBadge = `<span class="badge badge-green">${t('upload_completed')}</span>`;
        } else if (item.status === 'uploading') {
          statusBadge = `
            <div style="width: 100%;">
              <div style="font-size: 0.72rem; color: var(--cyan); margin-bottom: 2px;">${item.progress}%</div>
              <div style="width: 100%; height: 5px; background: rgba(255,255,255,0.1); border-radius: 3px; overflow: hidden;">
                <div style="width: ${item.progress}%; height: 100%; background: var(--cyan); transition: width 0.1s;"></div>
              </div>
            </div>
          `;
        } else if (item.status === 'failed') {
          statusBadge = `<span class="badge badge-red" title="${item.error || ''}">${t('upload_failed')}</span>`;
        }

        const opBtn = item.status === 'failed' 
          ? `<button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="retryUploadItem('${item.id}')">Retry</button>`
          : (item.status === 'waiting' ? `<button class="btn btn-danger" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="removeUploadItem('${item.id}')">&times;</button>` : '-');

        return `
          <tr style="border-bottom: 1px solid var(--border);">
            <td style="padding: 0.5rem 0.75rem; font-family: var(--font-mono); font-size: 0.78rem;">
              <span style="color: var(--cyan);">${item.relPath}</span>
            </td>
            <td style="padding: 0.5rem 0.75rem; font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-muted);">${formatBytes(item.size)}</td>
            <td style="padding: 0.5rem 0.75rem;">${statusBadge}</td>
            <td style="padding: 0.5rem 0.75rem; text-align: right;">${opBtn}</td>
          </tr>
        `;
      }).join('');

      updateUploadSummary();
    }

    function updateUploadSummary() {
      const totalBytes = uploadQueue.reduce((acc, cur) => acc + cur.size, 0);
      const completedItems = uploadQueue.filter(i => i.status === 'completed');
      const completedBytes = completedItems.reduce((acc, cur) => acc + cur.size, 0);

      const sizeSpan = document.getElementById('uploadSummarySize');
      if (sizeSpan) sizeSpan.textContent = `${formatBytes(completedBytes)} / ${formatBytes(totalBytes)}`;
      const countSpan = document.getElementById('uploadSummaryCount');
      if (countSpan) countSpan.textContent = `${completedItems.length} / ${uploadQueue.length}`;

      const speedSpan = document.getElementById('uploadSummarySpeed');
      if (speedSpan) {
        if (isUploading && uploadStartTime > 0) {
          const elapsedSec = (Date.now() - uploadStartTime) / 1000;
          if (elapsedSec > 0.2) {
            const speedBps = completedBytes / elapsedSec;
            speedSpan.textContent = `${formatBytes(speedBps)}/s`;
          }
        } else {
          speedSpan.textContent = '0 KB/s';
        }
      }
    }

    async function startUploadQueue() {
      if (isUploading) return;
      const domain = document.getElementById('fileSiteSelect').value;
      if (!domain) {
        showToast('Please select a website before uploading files', 'error');
        return;
      }

      isUploading = true;
      uploadStartTime = Date.now();
      const btn = document.getElementById('btnContinueUpload');
      if (btn) btn.disabled = true;

      while (true) {
        const nextItem = uploadQueue.find(i => i.status === 'waiting');
        if (!nextItem) break;

        nextItem.status = 'uploading';
        renderUploadQueue();

        try {
          await uploadSingleFile(domain, currentSubpath, nextItem);
          nextItem.status = 'completed';
        } catch (e) {
          nextItem.status = 'failed';
          nextItem.error = e.message;
        }

        renderUploadQueue();
      }

      isUploading = false;
      if (btn) btn.disabled = false;
      updateUploadSummary();
      loadSiteFiles();
      showToast(t('upload_all_done'), 'success');
    }

    function uploadSingleFile(domain, subpath, item) {
      return new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();
        const url = `/api/v1/files/upload?site=${encodeURIComponent(domain)}&path=${encodeURIComponent(subpath)}&filename=${encodeURIComponent(item.relPath)}`;
        xhr.open('POST', url, true);

        xhr.upload.onprogress = (e) => {
          if (e.lengthComputable) {
            item.progress = Math.round((e.loaded / e.total) * 100);
            renderUploadQueue();
          }
        };

        xhr.onload = () => {
          if (xhr.status >= 200 && xhr.status < 300) {
            resolve();
          } else {
            reject(new Error(xhr.responseText || `HTTP ${xhr.status}`));
          }
        };

        xhr.onerror = () => reject(new Error('Network error during file upload'));
        xhr.send(item.file);
      });
    }

    function retryUploadItem(id) {
      const item = uploadQueue.find(i => i.id === id);
      if (item) {
        item.status = 'waiting';
        item.progress = 0;
        item.error = null;
        renderUploadQueue();
        if (!isUploading) startUploadQueue();
      }
    }

    function removeUploadItem(id) {
      uploadQueue = uploadQueue.filter(i => i.id !== id);
      renderUploadQueue();
    }

    async function deleteFileItem(relPath) {
      if (!confirm(t('confirm_delete_file').replace('{name}', relPath))) return;
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
      document.getElementById('newEntryTitle').textContent = isDir 
        ? (currentLang === 'vi' ? 'Tạo Thư Mục Mới' : 'Create New Directory')
        : (currentLang === 'vi' ? 'Tạo Tệp Tin Mới' : 'Create New File');
      document.getElementById('newEntryLabel').textContent = isDir 
        ? (currentLang === 'vi' ? 'Tên Thư Mục' : 'Folder Name')
        : (currentLang === 'vi' ? 'Tên Tệp Tin (vd: index.php)' : 'File Name (e.g. index.php)');
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
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.8rem; justify-content: center;" onclick="actionService('${s.name}', 'restart')">${currentLang === 'vi' ? 'Khởi động lại' : 'Restart'}</button>
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.8rem; justify-content: center;" onclick="actionService('${s.name}', 'reload')">${currentLang === 'vi' ? 'Nạp lại' : 'Reload'}</button>
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
      if (tabName === 'deploy' && currentModSite) {
        loadSiteDeployConfig(currentModSite.domain);
        loadSiteDeployHistory(currentModSite.domain);
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

        // Tab 13: WAF & Rate Limiting
        document.getElementById('modWafToggle').checked = currentModSite.waf_enabled !== false;
        document.getElementById('modBadBotToggle').checked = currentModSite.bad_bot_blocking !== false;
        document.getElementById('modSqliXssToggle').checked = currentModSite.sqli_xss_protection !== false;
        document.getElementById('modRateLimitToggle').checked = !!currentModSite.rate_limit_enabled;
        document.getElementById('modRateLimitRequests').value = currentModSite.rate_limit_requests || 100;
        document.getElementById('modRateLimitWindow').value = currentModSite.rate_limit_window || '1m';
        document.getElementById('modCustomBlockedAgents').value = (currentModSite.custom_blocked_agents || []).join('\n');

        // Tab 14: Git-Ops Deploy
        const webhookUrl = `${window.location.origin}/api/v1/deploy/webhook?token=${encodeURIComponent(currentModSite.domain)}`;
        document.getElementById('modDeployWebhookUrl').value = webhookUrl;
        loadSiteDeployConfig(currentModSite.domain);
        loadSiteDeployHistory(currentModSite.domain);

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
          redirects: currentModSite.redirects || [],
          waf_enabled: document.getElementById('modWafToggle').checked,
          bad_bot_blocking: document.getElementById('modBadBotToggle').checked,
          sqli_xss_protection: document.getElementById('modSqliXssToggle').checked,
          rate_limit_enabled: document.getElementById('modRateLimitToggle').checked,
          rate_limit_requests: parseInt(document.getElementById('modRateLimitRequests').value) || 100,
          rate_limit_window: document.getElementById('modRateLimitWindow').value || '1m',
          custom_blocked_agents: document.getElementById('modCustomBlockedAgents').value
            .split('\n')
            .map(s => s.trim())
            .filter(s => s.length > 0)
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

        showToast(t('site_updated'), 'success');
        closeSiteModModal();
        loadSites();
      } catch (e) {
        showToast(e.message, 'error');
      }
    }

    // ============================================
    // DATABASE MANAGEMENT CONTROLLER
    // ============================================
    async function loadDatabases() {
      try {
        const res = await fetch('/api/v1/databases');
        if (!res.ok) return;
        allDatabases = await res.json();
        const badge = document.getElementById('navDatabasesCount');
        if (badge) badge.textContent = allDatabases.length;
        renderDatabasesTable(allDatabases);
      } catch (e) {
        console.error('Error loading databases:', e);
      }
    }

    function renderDatabasesTable(dbs) {
      const tbody = document.getElementById('databasesTableBody');
      if (!tbody) return;
      if (!dbs || !dbs.length) {
        tbody.innerHTML = `<tr><td colspan="7" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">${t('no_databases')}</td></tr>`;
        return;
      }

      tbody.innerHTML = dbs.map(d => {
        let engineBadge = '<span class="badge badge-cyan">MySQL</span>';
        if (d.engine === 'mariadb') engineBadge = '<span class="badge badge-purple">MariaDB</span>';
        else if (d.engine === 'sqlite') engineBadge = '<span class="badge badge-yellow">SQLite</span>';
        else if (d.engine === 'postgres') engineBadge = '<span class="badge badge-blue">PostgreSQL</span>';

        const siteName = d.site || d.linked_site;
        const siteLabel = siteName ? `<span style="font-weight: 600; color: var(--cyan);">${siteName}</span>` : '<span style="color: var(--text-dim);">-</span>';

        return `
          <tr>
            <td>
              <div style="font-weight: 700; color: var(--cyan-glow); font-size: 0.95rem;">${d.name}</div>
              <div style="font-size: 0.72rem; color: var(--text-dim); font-family: var(--font-mono);">${d.collation || d.character_set || ''}</div>
            </td>
            <td>${engineBadge}</td>
            <td style="font-family: var(--font-mono);">${d.username}</td>
            <td style="font-family: var(--font-mono); font-size: 0.82rem; color: var(--text-muted);">${d.host || d.access_host || ''}</td>
            <td>${siteLabel}</td>
            <td style="font-family: var(--font-mono); font-size: 0.85rem;">${formatBytes(d.size_bytes)}</td>
            <td style="text-align: right;">
              <div style="display: inline-flex; gap: 0.35rem;">
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="backupDatabase('${d.name}')">
                  <svg viewBox="0 0 24 24" style="width: 12px; height: 12px; fill: currentColor; margin-right: 0.25rem;"><path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
                  ${t('btn_backup')}
                </button>
                <button class="btn btn-danger" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="deleteDatabase('${d.name}')">${t('btn_delete')}</button>
              </div>
            </td>
          </tr>
        `;
      }).join('');
    }

    function filterDatabasesTable() {
      const q = (document.getElementById('dbSearchInput').value || '').toLowerCase();
      const filtered = allDatabases.filter(d => 
        d.name.toLowerCase().includes(q) || 
        d.username.toLowerCase().includes(q) || 
        ((d.site || d.linked_site) && (d.site || d.linked_site).toLowerCase().includes(q))
      );
      renderDatabasesTable(filtered);
    }

    async function openAddDatabaseModal() {
      document.getElementById('newDbName').value = '';
      document.getElementById('newDbUser').value = 'root';
      document.getElementById('newDbPass').value = '';
      document.getElementById('newDbHost').value = '127.0.0.1';

      // Populate site options
      const sel = document.getElementById('newDbSite');
      sel.innerHTML = '<option value="">-- ' + (currentLang === 'vi' ? 'Không liên kết' : 'None') + ' --</option>';
      allSites.forEach(s => {
        sel.innerHTML += `<option value="${s.domain}">${s.domain}</option>`;
      });

      generateRandomPassword();
      document.getElementById('addDatabaseModal').classList.add('active');
    }

    function closeAddDatabaseModal() {
      document.getElementById('addDatabaseModal').classList.remove('active');
    }

    function generateRandomPassword() {
      const chars = 'abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%';
      let pass = '';
      for (let i = 0; i < 16; i++) {
        pass += chars.charAt(Math.floor(Math.random() * chars.length));
      }
      document.getElementById('newDbPass').value = pass;
    }

    async function submitCreateDatabase() {
      const name = document.getElementById('newDbName').value.trim();
      const engine = document.getElementById('newDbEngine').value;
      const character_set = document.getElementById('newDbCollation').value;
      const username = document.getElementById('newDbUser').value.trim() || 'root';
      const password = document.getElementById('newDbPass').value.trim();
      const access_host = document.getElementById('newDbHost').value;
      const linked_site = document.getElementById('newDbSite').value.trim() || null;

      if (!name) {
        showToast(currentLang === 'vi' ? 'Vui lòng nhập tên CSDL' : 'Database name is required', 'error');
        return;
      }

      const payload = {
        name,
        engine,
        collation: character_set,
        username,
        password: password || null,
        host: access_host,
        site: linked_site
      };

      try {
        const res = await fetch('/api/v1/databases', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        if (res.ok) {
          closeAddDatabaseModal();
          showToast(t('db_created').replace('{name}', name), 'success');
          loadDatabases();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Network error: ' + e.message, 'error');
      }
    }

    async function deleteDatabase(name) {
      if (!confirm(t('confirm_delete_db').replace('{name}', name))) return;
      try {
        const res = await fetch(`/api/v1/databases?name=${encodeURIComponent(name)}`, { method: 'DELETE' });
        if (res.ok) {
          showToast(t('db_deleted').replace('{name}', name), 'success');
          loadDatabases();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e.message, 'error');
      }
    }

    async function backupDatabase(name) {
      try {
        const res = await fetch(`/api/v1/databases/backup?name=${encodeURIComponent(name)}`);
        if (!res.ok) {
          showToast('Backup error: ' + await res.text(), 'error');
          return;
        }
        const blob = await res.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.style.display = 'none';
        a.href = url;
        a.download = `${name}_backup.sql`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        a.remove();
        showToast(t('db_backup_success').replace('{name}', name), 'success');
      } catch (e) {
        showToast('Backup download failed: ' + e.message, 'error');
      }
    }

    // ============================================
    // CRON JOB MANAGER CONTROLLER
    // ============================================
    async function loadCronJobs() {
      try {
        const res = await fetch('/api/v1/cron');
        if (!res.ok) return;
        allCronJobs = await res.json();
        const badge = document.getElementById('navCronCount');
        if (badge) badge.textContent = allCronJobs.length;
        renderCronTable(allCronJobs);
      } catch (e) {
        console.error('Error loading cron jobs:', e);
      }
    }

    function renderCronTable(jobs) {
      const tbody = document.getElementById('cronTableBody');
      if (!tbody) return;
      if (!jobs || !jobs.length) {
        tbody.innerHTML = `<tr><td colspan="6" style="text-align: center; color: var(--text-muted); padding: 2.5rem;">${t('no_cron')}</td></tr>`;
        return;
      }

      tbody.innerHTML = jobs.map(j => {
        let statusBadge = '<span class="badge badge-yellow">Pending</span>';
        if (j.last_status && (j.last_status.toLowerCase() === 'success' || j.last_status === '0')) {
          statusBadge = '<span class="badge badge-green">Success (0)</span>';
        } else if (j.last_status) {
          statusBadge = `<span class="badge badge-red">${j.last_status}</span>`;
        }

        const runTimestamp = j.last_run_at || j.last_run;
        const lastRunStr = runTimestamp 
          ? new Date(runTimestamp * 1000).toISOString().replace('T', ' ').substring(0, 19)
          : '<span style="color: var(--text-dim);">-</span>';

        const toggleBtnLabel = j.enabled ? t('btn_disable') : t('btn_enable');
        const toggleBtnClass = j.enabled ? 'btn-secondary' : 'btn-success';
        const siteName = j.site || j.linked_site;

        return `
          <tr>
            <td>
              <div style="font-weight: 700; color: var(--cyan-glow); font-size: 0.95rem; display: flex; align-items: center; gap: 0.4rem;">
                <span class="pulse-dot" style="background: ${j.enabled ? 'var(--green)' : 'var(--text-dim)'}; box-shadow: 0 0 6px ${j.enabled ? 'var(--green)' : 'transparent'}"></span>
                ${j.name}
              </div>
              ${siteName ? `<div style="font-size: 0.72rem; color: var(--text-dim);">Site: ${siteName}</div>` : ''}
            </td>
            <td>
              <code style="font-size: 0.82rem; color: var(--purple-glow); background: rgba(168, 85, 247, 0.1); padding: 0.2rem 0.4rem; border-radius: 4px;">
                ${j.schedule}
              </code>
            </td>
            <td style="font-family: var(--font-mono); font-size: 0.78rem; color: var(--text-muted); max-width: 280px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;" title="${j.command}">
              ${j.command}
            </td>
            <td>${statusBadge}</td>
            <td style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-dim);">${lastRunStr}</td>
            <td style="text-align: right;">
              <div style="display: inline-flex; gap: 0.35rem;">
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem; border-color: rgba(56, 189, 248, 0.4); color: var(--cyan-glow);" onclick="runCronJob('${j.id}')">
                  ▶ ${t('btn_run_now')}
                </button>
                <button class="btn btn-secondary" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="viewCronLogs('${j.id}')">
                  📜 ${t('btn_logs')}
                </button>
                <button class="btn ${toggleBtnClass}" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="toggleCronJob('${j.id}')">
                  ${toggleBtnLabel}
                </button>
                <button class="btn btn-danger" style="padding: 0.25rem 0.55rem; font-size: 0.75rem;" onclick="deleteCronJob('${j.id}', '${j.name}')">
                  ${t('btn_delete')}
                </button>
              </div>
            </td>
          </tr>
        `;
      }).join('');
    }

    function openAddCronModal() {
      document.getElementById('newCronName').value = '';
      document.getElementById('newCronPreset').value = '* * * * *';
      document.getElementById('newCronSchedule').value = '* * * * *';
      document.getElementById('newCronCommand').value = '';

      // Populate site options
      const sel = document.getElementById('newCronSite');
      sel.innerHTML = '<option value="">-- ' + (currentLang === 'vi' ? 'Không liên kết' : 'None') + ' --</option>';
      allSites.forEach(s => {
        sel.innerHTML += `<option value="${s.domain}">${s.domain}</option>`;
      });

      document.getElementById('addCronModal').classList.add('active');
    }

    function closeAddCronModal() {
      document.getElementById('addCronModal').classList.remove('active');
    }

    function onCronPresetChange() {
      const preset = document.getElementById('newCronPreset').value;
      if (preset !== 'custom') {
        document.getElementById('newCronSchedule').value = preset;
      }
    }

    async function submitCreateCronJob() {
      const name = document.getElementById('newCronName').value.trim();
      const schedule = document.getElementById('newCronSchedule').value.trim();
      const command = document.getElementById('newCronCommand').value.trim();
      const linked_site = document.getElementById('newCronSite').value.trim() || null;

      if (!name || !schedule || !command) {
        showToast(currentLang === 'vi' ? 'Vui lòng điền đủ tên, chu kỳ và câu lệnh' : 'Name, schedule and command are required', 'error');
        return;
      }

      const payload = {
        name,
        schedule,
        command,
        enabled: true,
        site: linked_site
      };

      try {
        const res = await fetch('/api/v1/cron', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        if (res.ok) {
          closeAddCronModal();
          showToast(t('cron_created').replace('{name}', name), 'success');
          loadCronJobs();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Network error: ' + e.message, 'error');
      }
    }

    async function runCronJob(id) {
      try {
        showToast(t('cron_triggered'), 'info');
        const res = await fetch(`/api/v1/cron/run?id=${encodeURIComponent(id)}`, { method: 'POST' });
        if (res.ok) {
          loadCronJobs();
          viewCronLogs(id);
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e.message, 'error');
      }
    }

    async function toggleCronJob(id) {
      try {
        const res = await fetch(`/api/v1/cron/toggle?id=${encodeURIComponent(id)}`, { method: 'POST' });
        if (res.ok) {
          loadCronJobs();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e.message, 'error');
      }
    }

    async function deleteCronJob(id, name) {
      if (!confirm(t('confirm_delete_cron').replace('{name}', name))) return;
      try {
        const res = await fetch(`/api/v1/cron?id=${encodeURIComponent(id)}`, { method: 'DELETE' });
        if (res.ok) {
          showToast(t('cron_deleted').replace('{name}', name), 'success');
          loadCronJobs();
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Error: ' + e.message, 'error');
      }
    }

    async function viewCronLogs(id) {
      currentViewingCronId = id;
      const pre = document.getElementById('cronLogContent');
      pre.textContent = 'Loading logs...';
      document.getElementById('cronLogsModal').classList.add('active');
      await refreshCronLogs();
    }

    async function refreshCronLogs() {
      if (!currentViewingCronId) return;
      const pre = document.getElementById('cronLogContent');
      try {
        const res = await fetch(`/api/v1/cron/logs?id=${encodeURIComponent(currentViewingCronId)}`);
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

    function closeCronLogsModal() {
      document.getElementById('cronLogsModal').classList.remove('active');
      currentViewingCronId = null;
    }

    // ============================================
    // GIT-OPS & AUTO-DEPLOY CONTROLLER
    // ============================================
    let cachedDeployHistory = [];

    async function loadSiteDeployConfig(domain) {
      if (!domain) return;
      try {
        const res = await fetch(`/api/v1/deploy/config?domain=${encodeURIComponent(domain)}`);
        if (res.ok) {
          const cfg = await res.json();
          document.getElementById('modDeployRepoUrl').value = cfg.repo_url || '';
          document.getElementById('modDeployBranch').value = cfg.branch || 'main';
          document.getElementById('modDeploySymlinkToggle').checked = cfg.atomic_symlink !== false;
          document.getElementById('modDeployAutoToggle').checked = cfg.auto_deploy !== false;
          document.getElementById('modDeployBuildScript').value = cfg.build_script || '';
          if (cfg.webhook_secret) {
            document.getElementById('modDeployWebhookUrl').value = `${window.location.origin}/api/v1/deploy/webhook?token=${encodeURIComponent(cfg.webhook_secret)}`;
          }
        }
      } catch (e) {
        console.error('Error loading deploy config:', e);
      }
    }

    async function saveSiteDeployConfig() {
      if (!currentModSite) return;
      const repoUrl = document.getElementById('modDeployRepoUrl').value.trim();
      const branch = document.getElementById('modDeployBranch').value.trim() || 'main';
      const atomicSymlink = document.getElementById('modDeploySymlinkToggle').checked;
      const autoDeploy = document.getElementById('modDeployAutoToggle').checked;
      const buildScript = document.getElementById('modDeployBuildScript').value.trim() || null;

      const payload = {
        domain: currentModSite.domain,
        repo_url: repoUrl,
        branch: branch,
        atomic_symlink: atomicSymlink,
        auto_deploy: autoDeploy,
        build_script: buildScript
      };

      try {
        const res = await fetch('/api/v1/deploy/config', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        if (res.ok) {
          const updated = await res.json();
          if (updated.webhook_secret) {
            document.getElementById('modDeployWebhookUrl').value = `${window.location.origin}/api/v1/deploy/webhook?token=${encodeURIComponent(updated.webhook_secret)}`;
          }
          showToast(currentLang === 'vi' ? 'Đã lưu cấu hình Git-Ops thành công!' : 'Git-Ops configuration saved successfully!', 'success');
        } else {
          showToast('Failed: ' + await res.text(), 'error');
        }
      } catch (e) {
        showToast('Network error: ' + e.message, 'error');
      }
    }

    async function triggerSiteDeploy() {
      if (!currentModSite) return;
      const btn = document.getElementById('btnTriggerDeploy');
      const oldHtml = btn.innerHTML;
      btn.disabled = true;
      btn.innerHTML = `⏳ <span>${currentLang === 'vi' ? 'Đang triển khai...' : 'Deploying...'}</span>`;

      try {
        const res = await fetch('/api/v1/deploy/trigger', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ domain: currentModSite.domain })
        });
        const result = await res.json();
        if (res.ok && result.status === 'success') {
          showToast(currentLang === 'vi' ? `Triển khai bản ${result.release_id} thành công!` : `Release ${result.release_id} deployed successfully!`, 'success');
        } else {
          showToast(currentLang === 'vi' ? `Triển khai thất bại: ${result.message || 'Lỗi quy trình'}` : `Deployment failed: ${result.message || 'Build error'}`, 'error');
        }
      } catch (e) {
        showToast('Deploy error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.innerHTML = oldHtml;
        loadSiteDeployHistory(currentModSite.domain);
      }
    }

    async function loadSiteDeployHistory(domain) {
      if (!domain) return;
      const tbody = document.getElementById('modDeployHistoryTableBody');
      try {
        const res = await fetch(`/api/v1/deploy/history?domain=${encodeURIComponent(domain)}`);
        if (!res.ok) return;
        cachedDeployHistory = await res.json();
        if (!cachedDeployHistory.length) {
          tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: var(--text-dim); padding: 1rem;">${currentLang === 'vi' ? 'Chưa có bản phát hành nào.' : 'No deployments yet.'}</td></tr>`;
          return;
        }

        tbody.innerHTML = cachedDeployHistory.map(rel => {
          let badge = '<span class="badge badge-green">Success</span>';
          if (rel.status === 'failed') badge = '<span class="badge badge-red">Failed</span>';
          else if (rel.status === 'in_progress') badge = '<span class="badge badge-yellow">Building</span>';

          const commitDisplay = rel.commit_hash ? rel.commit_hash.substring(0, 7) : '-';
          const timeDisplay = new Date((rel.created_at || 0) * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });

          const rollbackBtn = (rel.status === 'success') ?
            `<button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="rollbackSiteDeploy('${domain}', '${rel.id}')">Rollback</button>` : '';

          return `
            <tr style="border-bottom: 1px solid var(--border);">
              <td style="padding: 0.4rem 0.6rem; font-family: var(--font-mono); font-weight: 600; color: var(--cyan);">${rel.id}</td>
              <td style="padding: 0.4rem 0.6rem; font-family: var(--font-mono);">${commitDisplay}</td>
              <td style="padding: 0.4rem 0.6rem;">${badge}</td>
              <td style="padding: 0.4rem 0.6rem; color: var(--text-muted);">${timeDisplay}</td>
              <td style="padding: 0.4rem 0.6rem; text-align: right;">
                <div style="display: inline-flex; gap: 0.3rem;">
                  <button class="btn btn-secondary" style="padding: 0.15rem 0.4rem; font-size: 0.68rem;" onclick="viewDeployReleaseLog('${rel.id}')">${t('btn_logs')}</button>
                  ${rollbackBtn}
                </div>
              </td>
            </tr>
          `;
        }).join('');
      } catch (e) {
        console.error('Error loading deploy history:', e);
      }
    }

    async function rollbackSiteDeploy(domain, releaseId) {
      const msg = currentLang === 'vi' ?
        `Bạn có chắc chắn muốn hoàn tác website '${domain}' về bản phát hành ${releaseId}?` :
        `Are you sure you want to rollback website '${domain}' to release ${releaseId}?`;
      if (!confirm(msg)) return;

      try {
        const res = await fetch('/api/v1/deploy/rollback', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ domain: domain, release_id: releaseId })
        });
        const result = await res.json();
        if (res.ok && result.status === 'success') {
          showToast(currentLang === 'vi' ? `Hoàn tác về bản ${releaseId} hoàn tất trong < 1ms!` : `Rolled back to release ${releaseId} in < 1ms!`, 'success');
          loadSiteDeployHistory(domain);
        } else {
          showToast('Rollback failed: ' + (result.message || 'Unknown error'), 'error');
        }
      } catch (e) {
        showToast('Rollback network error: ' + e.message, 'error');
      }
    }

    function viewDeployReleaseLog(releaseId) {
      const rel = cachedDeployHistory.find(r => r.id === releaseId);
      const pre = document.getElementById('deployLogContent');
      if (rel && rel.output_log) {
        pre.textContent = rel.output_log;
      } else {
        pre.textContent = 'No logs available for this release.';
      }
      document.getElementById('deployLogsModal').classList.add('active');
    }

    function closeDeployLogsModal() {
      document.getElementById('deployLogsModal').classList.remove('active');
    }

    function copyWebhookUrl() {
      const input = document.getElementById('modDeployWebhookUrl');
      if (input && input.value) {
        navigator.clipboard.writeText(input.value);
        showToast(currentLang === 'vi' ? 'Đã sao chép Webhook URL vào bộ nhớ đệm!' : 'Webhook URL copied to clipboard!', 'info');
      }
    }

    function setDeployPreset(preset) {
      const area = document.getElementById('modDeployBuildScript');
      if (preset === 'laravel') {
        area.value = "composer install --no-dev --optimize-autoloader\nphp artisan config:cache\nphp artisan route:cache\nphp artisan view:cache\nphp artisan migrate --force";
      } else if (preset === 'vite') {
        area.value = "npm ci\nnpm run build";
      } else if (preset === 'static') {
        area.value = 'echo "Static assets synced."';
      }
    }

    // Initial load
    updateLanguageUI();
    applyTranslations();
    loadSites();
    loadDatabases();
    loadCronJobs();
    initFileDragAndDrop();
  </script>
</body>
</html>
"###;
