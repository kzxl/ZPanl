/// Embedded single-page application for the ZPanl dashboard.
pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ZPanl ⚡ Sovereign Web Panel</title>
  <style>
    :root {
      --bg: #0b0f19;
      --card-bg: #111827;
      --border: #1f2937;
      --text: #f3f4f6;
      --text-muted: #9ca3af;
      --accent: #0284c7;
      --accent-glow: #38bdf8;
      --success: #10b981;
      --warning: #f59e0b;
      --danger: #ef4444;
      --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: var(--bg);
      color: var(--text);
      font-family: var(--font-sans);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
    }
    header {
      background: var(--card-bg);
      border-bottom: 1px solid var(--border);
      padding: 0.75rem 2rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
    }
    .logo {
      display: flex;
      align-items: center;
      gap: 0.75rem;
      font-weight: 700;
      font-size: 1.25rem;
      letter-spacing: -0.025em;
    }
    .logo-badge {
      font-size: 0.7rem;
      background: rgba(56, 189, 248, 0.15);
      color: var(--accent-glow);
      border: 1px solid var(--accent);
      padding: 0.15rem 0.45rem;
      border-radius: 9999px;
      font-weight: 600;
    }
    nav {
      display: flex;
      gap: 0.5rem;
    }
    nav button {
      background: transparent;
      border: 1px solid transparent;
      color: var(--text-muted);
      padding: 0.45rem 1rem;
      border-radius: 0.375rem;
      font-size: 0.875rem;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.15s ease;
    }
    nav button:hover {
      color: var(--text);
      background: rgba(255, 255, 255, 0.05);
    }
    nav button.active {
      color: var(--accent-glow);
      background: rgba(2, 132, 199, 0.15);
      border-color: rgba(2, 132, 199, 0.4);
    }
    main {
      flex: 1;
      padding: 2rem;
      max-width: 1400px;
      margin: 0 auto;
      width: 100%;
    }
    .tab-content { display: none; }
    .tab-content.active { display: block; }
    .grid-4 {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
      gap: 1.25rem;
      margin-bottom: 2rem;
    }
    .card {
      background: var(--card-bg);
      border: 1px solid var(--border);
      border-radius: 0.5rem;
      padding: 1.25rem;
    }
    .card-title {
      font-size: 0.8rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--text-muted);
      margin-bottom: 0.5rem;
    }
    .card-value {
      font-size: 1.75rem;
      font-weight: 700;
      margin-bottom: 0.5rem;
      font-family: var(--font-mono);
    }
    .progress-bar {
      height: 6px;
      background: var(--border);
      border-radius: 3px;
      overflow: hidden;
    }
    .progress-fill {
      height: 100%;
      background: var(--accent-glow);
      transition: width 0.3s ease;
    }
    table {
      width: 100%;
      border-collapse: collapse;
      text-align: left;
      font-size: 0.875rem;
    }
    th {
      background: rgba(0, 0, 0, 0.2);
      color: var(--text-muted);
      padding: 0.75rem 1rem;
      font-weight: 600;
      border-bottom: 1px solid var(--border);
    }
    td {
      padding: 0.75rem 1rem;
      border-bottom: 1px solid var(--border);
    }
    tr:hover td {
      background: rgba(255, 255, 255, 0.02);
    }
    .badge {
      display: inline-block;
      padding: 0.2rem 0.5rem;
      border-radius: 0.25rem;
      font-size: 0.75rem;
      font-weight: 600;
      font-family: var(--font-mono);
    }
    .badge-success { background: rgba(16, 185, 129, 0.15); color: var(--success); border: 1px solid rgba(16, 185, 129, 0.3); }
    .badge-warning { background: rgba(245, 158, 11, 0.15); color: var(--warning); border: 1px solid rgba(245, 158, 11, 0.3); }
    .badge-info { background: rgba(56, 189, 248, 0.15); color: var(--accent-glow); border: 1px solid rgba(56, 189, 248, 0.3); }
    .badge-danger { background: rgba(239, 68, 68, 0.15); color: var(--danger); border: 1px solid rgba(239, 68, 68, 0.3); }
    .btn {
      background: var(--accent);
      color: #fff;
      border: none;
      padding: 0.45rem 0.9rem;
      border-radius: 0.375rem;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      transition: background 0.15s ease;
    }
    .btn:hover { background: #0369a1; }
    .btn-danger { background: rgba(239, 68, 68, 0.2); color: var(--danger); border: 1px solid rgba(239, 68, 68, 0.4); }
    .btn-danger:hover { background: var(--danger); color: #fff; }
    .btn-secondary { background: var(--border); color: var(--text); }
    .btn-secondary:hover { background: #374151; }
    .toolbar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 1.25rem;
    }
    .modal {
      display: none;
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.7);
      backdrop-filter: blur(4px);
      align-items: center;
      justify-content: center;
      z-index: 1000;
    }
    .modal.active { display: flex; }
    .modal-box {
      background: var(--card-bg);
      border: 1px solid var(--border);
      border-radius: 0.5rem;
      width: 90%;
      max-width: 600px;
      padding: 1.75rem;
    }
    .form-group {
      margin-bottom: 1rem;
    }
    .form-group label {
      display: block;
      font-size: 0.8rem;
      color: var(--text-muted);
      margin-bottom: 0.35rem;
    }
    .form-input, .form-select {
      width: 100%;
      background: var(--bg);
      border: 1px solid var(--border);
      color: var(--text);
      padding: 0.5rem 0.75rem;
      border-radius: 0.375rem;
      font-size: 0.9rem;
    }
    .form-input:focus, .form-select:focus {
      outline: none;
      border-color: var(--accent-glow);
    }
    pre {
      background: #000;
      color: #38bdf8;
      font-family: var(--font-mono);
      padding: 1rem;
      border-radius: 0.375rem;
      overflow-x: auto;
      font-size: 0.85rem;
      border: 1px solid var(--border);
    }
    .editor-textarea {
      width: 100%;
      height: 450px;
      background: #05070d;
      color: #f1f5f9;
      font-family: var(--font-mono);
      padding: 1rem;
      border: 1px solid var(--border);
      border-radius: 0.375rem;
      font-size: 0.9rem;
      resize: vertical;
    }
  </style>
</head>
<body>
  <header>
    <div class="logo">
      <span>⚡ ZPanl</span>
      <span class="logo-badge">SOVEREIGN LINUX</span>
    </div>
    <nav>
      <button class="active" onclick="switchTab('overview')">Overview</button>
      <button onclick="switchTab('sites')">Websites</button>
      <button onclick="switchTab('files')">Files</button>
      <button onclick="switchTab('services')">Services</button>
      <button onclick="switchTab('caddy')">Caddyfile</button>
    </nav>
    <div id="uptimeBadge" style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--text-muted);">Uptime: 0s</div>
  </header>

  <main>
    <!-- OVERVIEW TAB -->
    <div id="tab-overview" class="tab-content active">
      <div class="grid-4">
        <div class="card">
          <div class="card-title">CPU Utilization</div>
          <div class="card-value" id="cpuVal">0.0%</div>
          <div class="progress-bar"><div class="progress-fill" id="cpuBar" style="width: 0%;"></div></div>
        </div>
        <div class="card">
          <div class="card-title">RAM Usage</div>
          <div class="card-value" id="ramVal">0 / 0 MB</div>
          <div class="progress-bar"><div class="progress-fill" id="ramBar" style="width: 0%;"></div></div>
        </div>
        <div class="card">
          <div class="card-title">Network In / Out</div>
          <div class="card-value" id="netVal">0 / 0 KB/s</div>
          <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.25rem;">Real-time throughput</div>
        </div>
        <div class="card">
          <div class="card-title">Engine Architecture</div>
          <div class="card-value" style="font-size: 1.25rem; color: var(--accent-glow);">Pure Rust + Caddy</div>
          <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.25rem;">Sub-millisecond Edge VFS</div>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-bottom: 1rem;">Hosted Websites Summary</h3>
        <div id="overviewSitesList">Loading sites...</div>
      </div>
    </div>

    <!-- SITES TAB -->
    <div id="tab-sites" class="tab-content">
      <div class="toolbar">
        <h2>Hosted Websites</h2>
        <button class="btn" onclick="openAddSiteModal()">+ Create Site</button>
      </div>
      <div class="card">
        <table>
          <thead>
            <tr>
              <th>Domain</th>
              <th>Type</th>
              <th>PHP Version</th>
              <th>Web Root</th>
              <th>SSL</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="sitesTableBody">
            <tr><td colspan="6" style="text-align: center;">Loading websites...</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- FILES TAB -->
    <div id="tab-files" class="tab-content">
      <div class="toolbar">
        <div style="display: flex; gap: 1rem; align-items: center;">
          <select id="fileSiteSelect" class="form-select" style="width: 250px;" onchange="loadSiteFiles()"></select>
          <span id="fileBreadcrumb" style="font-family: var(--font-mono); font-size: 0.85rem; color: var(--accent-glow);">/</span>
        </div>
        <div style="display: flex; gap: 0.5rem;">
          <button class="btn btn-secondary" onclick="openNewEntryModal(false)">+ New File</button>
          <button class="btn btn-secondary" onclick="openNewEntryModal(true)">+ New Folder</button>
          <button class="btn" onclick="loadSiteFiles()">Refresh</button>
        </div>
      </div>
      <div class="card">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Type</th>
              <th>Size</th>
              <th>Permissions</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="filesTableBody">
            <tr><td colspan="5" style="text-align: center;">Select a site to explore web files.</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- SERVICES TAB -->
    <div id="tab-services" class="tab-content">
      <div class="toolbar">
        <h2>Linux Daemons & Services</h2>
        <button class="btn" onclick="loadServices()">Refresh</button>
      </div>
      <div class="grid-4" id="servicesGrid">
        Loading services...
      </div>
    </div>

    <!-- CADDYFILE TAB -->
    <div id="tab-caddy" class="tab-content">
      <div class="toolbar">
        <h2>Live Generated Caddyfile (/etc/caddy/Caddyfile)</h2>
        <button class="btn" onclick="loadCaddyfile()">Reload Preview</button>
      </div>
      <pre id="caddyfileContent">Loading Caddyfile...</pre>
    </div>
  </main>

  <!-- MODAL: ADD SITE -->
  <div id="addSiteModal" class="modal">
    <div class="modal-box">
      <h3 style="margin-bottom: 1.25rem;">Create Virtual Host</h3>
      <div class="form-group">
        <label>Domain Name</label>
        <input id="newDomain" class="form-input" placeholder="e.g. blog.mydomain.com">
      </div>
      <div class="form-group">
        <label>Website Type</label>
        <select id="newKind" class="form-select" onchange="togglePhpField()">
          <option value="static">Static HTML/CSS/JS</option>
          <option value="spa_fallback">Single Page Application (SPA Fallback)</option>
          <option value="php_fpm">Dynamic PHP (PHP-FPM)</option>
        </select>
      </div>
      <div class="form-group" id="phpVersionGroup" style="display: none;">
        <label>PHP Version</label>
        <select id="newPhpVer" class="form-select">
          <option value="8.3">PHP 8.3-FPM</option>
          <option value="8.2" selected>PHP 8.2-FPM</option>
          <option value="8.1">PHP 8.1-FPM</option>
        </select>
      </div>
      <div class="form-group">
        <label>Web Root Directory</label>
        <input id="newRoot" class="form-input" placeholder="e.g. /var/www/blog.mydomain.com">
      </div>
      <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1.5rem;">
        <button class="btn btn-secondary" onclick="closeAddSiteModal()">Cancel</button>
        <button class="btn" onclick="submitCreateSite()">Create Virtual Host</button>
      </div>
    </div>
  </div>

  <!-- MODAL: FILE EDITOR -->
  <div id="fileEditorModal" class="modal">
    <div class="modal-box" style="max-width: 900px; width: 95%;">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
        <h3 id="editorFileName" style="font-family: var(--font-mono); font-size: 1rem;">Editing</h3>
        <span id="editorMime" class="badge badge-info">text/plain</span>
      </div>
      <textarea id="editorTextarea" class="editor-textarea" spellcheck="false"></textarea>
      <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;">
        <button class="btn btn-secondary" onclick="closeEditorModal()">Cancel</button>
        <button class="btn" onclick="saveFileContent()">Save Changes</button>
      </div>
    </div>
  </div>

  <!-- MODAL: NEW ENTRY -->
  <div id="newEntryModal" class="modal">
    <div class="modal-box" style="max-width: 450px;">
      <h3 id="newEntryTitle" style="margin-bottom: 1rem;">New Item</h3>
      <div class="form-group">
        <label>Item Name</label>
        <input id="newEntryName" class="form-input" placeholder="e.g. index.php">
      </div>
      <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;">
        <button class="btn btn-secondary" onclick="closeNewEntryModal()">Cancel</button>
        <button class="btn" onclick="submitCreateEntry()">Create</button>
      </div>
    </div>
  </div>

  <script>
    let currentTab = 'overview';
    let currentSubpath = '';
    let isCreatingDir = false;
    let editingRelPath = '';

    function switchTab(tab) {
      currentTab = tab;
      document.querySelectorAll('nav button').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      event.target.classList.add('active');
      document.getElementById('tab-' + tab).classList.add('active');

      if (tab === 'sites') loadSites();
      if (tab === 'files') initFilesTab();
      if (tab === 'services') loadServices();
      if (tab === 'caddy') loadCaddyfile();
    }

    async function pollTelemetry() {
      try {
        const res = await fetch('/api/v1/telemetry');
        if (!res.ok) return;
        const d = await res.json();

        document.getElementById('cpuVal').textContent = d.cpu_usage_percent.toFixed(1) + '%';
        document.getElementById('cpuBar').style.width = Math.min(d.cpu_usage_percent, 100) + '%';

        document.getElementById('ramVal').textContent = `${d.ram_used_mb} / ${d.ram_total_mb} MB (${d.ram_usage_percent.toFixed(1)}%)`;
        document.getElementById('ramBar').style.width = Math.min(d.ram_usage_percent, 100) + '%';

        document.getElementById('netVal').textContent = `↓ ${d.net_rx_kbps} KB/s  ↑ ${d.net_tx_kbps} KB/s`;
        document.getElementById('uptimeBadge').textContent = `Uptime: ${d.uptime_seconds}s`;
      } catch (e) {
        console.error('Telemetry error:', e);
      }
    }
    setInterval(pollTelemetry, 2000);
    pollTelemetry();

    async function loadSites() {
      try {
        const res = await fetch('/api/v1/sites');
        const sites = await res.json();
        const tbody = document.getElementById('sitesTableBody');
        const overviewList = document.getElementById('overviewSitesList');

        if (!sites.length) {
          tbody.innerHTML = '<tr><td colspan="6" style="text-align: center; color: var(--text-muted);">No websites hosted yet. Click "+ Create Site" to deploy one!</td></tr>';
          overviewList.innerHTML = '<div style="color: var(--text-muted);">No websites registered.</div>';
          return;
        }

        tbody.innerHTML = sites.map(s => `
          <tr>
            <td style="font-weight: 600; color: var(--accent-glow);">${s.domain}</td>
            <td><span class="badge badge-info">${s.kind}</span></td>
            <td>${s.php_version ? 'PHP ' + s.php_version : '-'}</td>
            <td style="font-family: var(--font-mono); font-size: 0.8rem;">${s.root_path}</td>
            <td><span class="badge badge-success">Active</span></td>
            <td>
              <button class="btn btn-secondary" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="openSiteInFiles('${s.domain}')">Files</button>
              <button class="btn btn-danger" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="deleteSite('${s.domain}')">Delete</button>
            </td>
          </tr>
        `).join('');

        overviewList.innerHTML = `<div><strong>${sites.length} Active Virtual Hosts</strong> managed under Sovereign Caddy v2 reverse proxy.</div>`;
      } catch (e) {
        console.error(e);
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
        alert('Domain name is required');
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
          loadSites();
        } else {
          const err = await res.text();
          alert('Failed: ' + err);
        }
      } catch (e) {
        alert('Network error: ' + e);
      }
    }

    async function deleteSite(domain) {
      if (!confirm(`Are you sure you want to remove domain '${domain}' from ZPanl?`)) return;
      try {
        const res = await fetch(`/api/v1/sites?domain=${encodeURIComponent(domain)}`, { method: 'DELETE' });
        if (res.ok) {
          loadSites();
        } else {
          alert('Failed: ' + await res.text());
        }
      } catch (e) {
        alert('Error: ' + e);
      }
    }

    async function initFilesTab() {
      const res = await fetch('/api/v1/sites');
      const sites = await res.json();
      const sel = document.getElementById('fileSiteSelect');
      sel.innerHTML = sites.map(s => `<option value="${s.domain}">${s.domain}</option>`).join('');
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
        if (!res.ok) {
          document.getElementById('filesTableBody').innerHTML = `<tr><td colspan="5" style="color: var(--danger); text-align: center;">Error: ${await res.text()}</td></tr>`;
          return;
        }
        const entries = await res.json();
        const tbody = document.getElementById('filesTableBody');

        let rows = '';
        if (currentSubpath) {
          rows += `<tr>
            <td colspan="5" style="cursor: pointer; color: var(--accent-glow);" onclick="navigateUp()">📁 .. (Parent Directory)</td>
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
              <td style="cursor: pointer; font-weight: 500;" ${clickAction}>${icon} ${e.name}</td>
              <td><span class="badge ${isDir ? 'badge-warning' : 'badge-info'}">${e.file_type}</span></td>
              <td style="font-family: var(--font-mono);">${formatBytes(e.size_bytes)}</td>
              <td style="font-family: var(--font-mono); font-size: 0.8rem;">0o${e.posix_mode.toString(8)}</td>
              <td>
                ${!isDir ? `<button class="btn btn-secondary" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="openEditor('${e.rel_path}')">Edit</button>` : ''}
                <button class="btn btn-danger" style="padding: 0.2rem 0.5rem; font-size: 0.75rem;" onclick="deleteFileItem('${e.rel_path}')">Delete</button>
              </td>
            </tr>
          `;
        }).join('');

        tbody.innerHTML = rows || '<tr><td colspan="5" style="text-align: center; color: var(--text-muted);">Directory is empty.</td></tr>';
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

    async function openEditor(relPath) {
      const domain = document.getElementById('fileSiteSelect').value;
      editingRelPath = relPath;
      try {
        const res = await fetch(`/api/v1/files/read?site=${encodeURIComponent(domain)}&path=${encodeURIComponent(relPath)}`);
        if (!res.ok) {
          alert('Cannot open: ' + await res.text());
          return;
        }
        const data = await res.json();
        document.getElementById('editorFileName').textContent = relPath;
        document.getElementById('editorMime').textContent = data.mime;
        document.getElementById('editorTextarea').value = data.content;
        document.getElementById('fileEditorModal').classList.add('active');
      } catch (e) {
        alert('Error: ' + e);
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
          closeEditorModal();
          loadSiteFiles();
        } else {
          alert('Save failed: ' + await res.text());
        }
      } catch (e) {
        alert('Error saving: ' + e);
      }
    }

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
          loadSiteFiles();
        } else {
          alert('Delete failed: ' + await res.text());
        }
      } catch (e) {
        alert('Error: ' + e);
      }
    }

    function openNewEntryModal(isDir) {
      isCreatingDir = isDir;
      document.getElementById('newEntryTitle').textContent = isDir ? 'New Folder' : 'New File';
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
          loadSiteFiles();
        } else {
          alert('Create failed: ' + await res.text());
        }
      } catch (e) {
        alert('Error: ' + e);
      }
    }

    async function loadServices() {
      try {
        const res = await fetch('/api/v1/services');
        const services = await res.json();
        const grid = document.getElementById('servicesGrid');
        grid.innerHTML = services.map(s => `
          <div class="card">
            <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
              <div>
                <strong style="font-size: 1rem;">${s.display_name}</strong>
                <div style="font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-muted);">${s.name}.service</div>
              </div>
              <span class="badge ${s.is_active ? 'badge-success' : 'badge-danger'}">${s.state}</span>
            </div>
            <div style="display: flex; gap: 0.5rem; margin-top: 1rem;">
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.75rem;" onclick="actionService('${s.name}', 'restart')">Restart</button>
              <button class="btn btn-secondary" style="flex: 1; font-size: 0.75rem;" onclick="actionService('${s.name}', 'reload')">Reload</button>
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
          alert(`Service ${name} ${action} commanded`);
          loadServices();
        } else {
          alert('Action failed: ' + await res.text());
        }
      } catch (e) {
        alert('Error: ' + e);
      }
    }

    async function loadCaddyfile() {
      try {
        const res = await fetch('/api/v1/caddy/caddyfile');
        const text = await res.text();
        document.getElementById('caddyfileContent').textContent = text;
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
  </script>
</body>
</html>
"#;
