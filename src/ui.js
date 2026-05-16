// UI Management

// Navigation
export function initNavigation() {
  const navItems = document.querySelectorAll(".nav-item");
  const views = document.querySelectorAll(".view");

  navItems.forEach((item) => {
    item.addEventListener("click", () => {
      const viewId = item.dataset.view;
      navItems.forEach((i) => i.classList.remove("active"));
      views.forEach((v) => v.classList.remove("active"));
      item.classList.add("active");
      document.getElementById(`view-${viewId}`).classList.add("active");
    });
  });
}

// Sidebar toggle
export function initSidebar() {
  const toggleBtn = document.getElementById("toggle-sidebar");
  const sidebar = document.getElementById("sidebar");
  if (toggleBtn && sidebar) {
    toggleBtn.addEventListener("click", () => {
      sidebar.classList.toggle("collapsed");
    });
  }
}

// Toast notifications
export function showToast(message, duration = 3000) {
  const container = document.getElementById("toast-container");
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.textContent = message;
  container.appendChild(toast);
  setTimeout(() => {
    toast.classList.add("fade-out");
    setTimeout(() => toast.remove(), 300);
  }, duration);
}

export function showError(message) {
  showToast(`Error: ${message}`, 5000);
}

export function showSuccess(message) {
  showToast(message, 3000);
}

// Theme presets
export function initThemePresets(config) {
  const presets = document.querySelectorAll(".preset-btn");
  const colorInput = document.getElementById("setting-theme-color");

  presets.forEach((btn) => {
    btn.addEventListener("click", () => {
      presets.forEach((b) => b.classList.remove("active"));
      btn.classList.add("active");
      colorInput.value = btn.dataset.color;
    });
  });

  // Sync color picker with presets
  colorInput.addEventListener("input", () => {
    presets.forEach((b) => {
      b.classList.toggle("active", b.dataset.color.toLowerCase() === colorInput.value.toLowerCase());
    });
  });
}

// Custom FFlags rendering
export function renderCustomFflags(flags) {
  const container = document.getElementById("custom-fflags-list");
  container.innerHTML = "";
  Object.entries(flags).forEach(([key, value]) => {
    const row = document.createElement("div");
    row.className = "fflag-row";
    row.innerHTML = `
      <input type="text" class="fflag-key" value="${key}" placeholder="Flag name" style="flex: 1; padding: 4px 8px; background: var(--bg-input); border: 1px solid var(--border-color); color: var(--text-primary); border-radius: 4px;">
      <input type="text" class="fflag-value" value="${value}" placeholder="Value" style="flex: 1; padding: 4px 8px; background: var(--bg-input); border: 1px solid var(--border-color); color: var(--text-primary); border-radius: 4px; margin-left: 8px;">
      <button class="btn-remove-fflag" style="background: none; border: none; color: var(--text-secondary); cursor: pointer; padding: 4px 8px; margin-left: 4px;">✕</button>
    `;
    row.querySelector(".btn-remove-fflag").addEventListener("click", () => row.remove());
    container.appendChild(row);
  });
}

// Add FFlag row
export function addFflagRow() {
  const container = document.getElementById("custom-fflags-list");
  const row = document.createElement("div");
  row.className = "fflag-row";
  row.innerHTML = `
    <input type="text" class="fflag-key" placeholder="Flag name" style="flex: 1; padding: 4px 8px; background: var(--bg-input); border: 1px solid var(--border-color); color: var(--text-primary); border-radius: 4px;">
    <input type="text" class="fflag-value" placeholder="Value" style="flex: 1; padding: 4px 8px; background: var(--bg-input); border: 1px solid var(--border-color); color: var(--text-primary); border-radius: 4px; margin-left: 8px;">
    <button class="btn-remove-fflag" style="background: none; border: none; color: var(--text-secondary); cursor: pointer; padding: 4px 8px; margin-left: 4px;">✕</button>
  `;
  row.querySelector(".btn-remove-fflag").addEventListener("click", () => row.remove());
  container.appendChild(row);
}

// Collect FFlags from UI
export function collectFflags() {
  const rows = document.querySelectorAll(".fflag-row");
  const flags = {};
  rows.forEach((row) => {
    const key = row.querySelector(".fflag-key").value.trim();
    const value = row.querySelector(".fflag-value").value.trim();
    if (key) flags[key] = value;
  });
  return flags;
}

// GPU loading
export async function loadGpus(gpuSelect, getGpusApi) {
  try {
    const gpus = await getGpusApi();
    gpuSelect.innerHTML = '<option value="default">Default</option>';
    gpus.forEach((gpu) => {
      const option = document.createElement("option");
      option.value = gpu.id;
      option.textContent = gpu.name;
      if (gpu.isNvidia) option.textContent += " (NVIDIA)";
      gpuSelect.appendChild(option);
    });
  } catch (e) {
    gpuSelect.innerHTML = '<option value="default">Default (failed to load)</option>';
  }
}

// System checks
export async function runSystemChecks() {
  const sse42El = document.getElementById("check-sse42");
  const soberStatusEl = document.getElementById("check-sober-running");
  const soberMsgEl = document.getElementById("check-sober-msg");

  try {
    const sse42 = await window.__TAURI__.core.invoke("check_sse42");
    sse42El.textContent = sse42 ? "✓" : "✕";
    sse42El.style.color = sse42 ? "#2ecc71" : "#e74c3c";
  } catch {
    sse42El.textContent = "?";
  }

  try {
    const status = await window.__TAURI__.core.invoke("check_sober_status");
    soberStatusEl.textContent = status.running ? "●" : "○";
    soberStatusEl.style.color = status.running ? "#e67e22" : "#2ecc71";
    soberMsgEl.textContent = status.running ? `Running (PID: ${status.pid})` : "Not running";
  } catch (e) {
    soberStatusEl.textContent = "?";
    soberMsgEl.textContent = "Check failed";
  }
}

// Theme progress
export function showThemeProgress(show) {
  const container = document.getElementById("theme-progress-container");
  container.style.display = show ? "block" : "none";
}

export function updateThemeProgress(status, progress, message) {
  const bar = document.getElementById("theme-progress-bar");
  const text = document.getElementById("theme-progress-text");
  const percent = document.getElementById("theme-progress-percent");
  bar.style.width = `${progress}%`;
  text.textContent = message;
  percent.textContent = `${progress}%`;
}