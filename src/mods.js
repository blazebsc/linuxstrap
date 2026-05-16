// Mods management

let currentModTab = "installed";

export function initMods(getModsApi, installModApi, uninstallModApi, showToast) {
  const tabInstalled = document.getElementById("tab-installed");
  const tabGamebanana = document.getElementById("tab-gamebanana");
  const tabFishstrap = document.getElementById("tab-fishstrap");

  tabInstalled.addEventListener("click", () => switchModTab("installed"));
  tabGamebanana.addEventListener("click", () => switchModTab("gamebanana"));
  tabFishstrap.addEventListener("click", () => switchModTab("fishstrap"));
}

function switchModTab(tab) {
  currentModTab = tab;
  document.getElementById("tab-installed").classList.toggle("active-tab", tab === "installed");
  document.getElementById("tab-gamebanana").classList.toggle("active-tab", tab === "gamebanana");
  document.getElementById("tab-fishstrap").classList.toggle("active-tab", tab === "fishstrap");
  renderModsContent();
}

async function renderModsContent() {
  const contentArea = document.getElementById("mods-content-area");
  contentArea.innerHTML = '<div class="empty-state"><p>Loading...</p></div>';

  if (currentModTab === "installed") {
    try {
      const mods = await window.__TAURI__.core.invoke("get_mods");
      renderInstalledMods(mods);
    } catch (e) {
      contentArea.innerHTML = `<div class="empty-state"><p>Failed to load mods: ${e}</p></div>`;
    }
  } else if (currentModTab === "gamebanana") {
    try {
      const mods = await window.__TAURI__.core.invoke("get_gamebanana_mods", { page: 1 });
      renderGameBananaMods(mods);
    } catch (e) {
      contentArea.innerHTML = `<div class="empty-state"><p>Failed to load GameBanana: ${e}</p></div>`;
    }
  } else if (currentModTab === "fishstrap") {
    try {
      const mods = await window.__TAURI__.core.invoke("get_fishstrap_mods");
      renderFishstrapMods(mods);
    } catch (e) {
      contentArea.innerHTML = `<div class="empty-state"><p>Failed to load Fishstrap: ${e}</p></div>`;
    }
  }
}

function renderInstalledMods(mods) {
  const contentArea = document.getElementById("mods-content-area");
  if (!mods || mods.length === 0) {
    contentArea.innerHTML = `
      <div class="empty-state">
        <p>No mods installed yet.</p>
        <p style="font-size: 11px; opacity: 0.6;">Browse GameBanana or Fishstrap tabs to find mods.</p>
      </div>
    `;
    return;
  }

  contentArea.innerHTML = mods.map((mod) => `
    <div class="mod-card">
      <div class="mod-info">
        <h4>${mod.name || mod.id}</h4>
        <p class="mod-type">${mod.type || "mod"}</p>
      </div>
      <button class="btn-uninstall-mod secondary-btn" data-id="${mod.id}" style="padding: 4px 10px; font-size: 11px;">Uninstall</button>
    </div>
  `).join("");

  contentArea.querySelectorAll(".btn-uninstall-mod").forEach((btn) => {
    btn.addEventListener("click", async () => {
      try {
        await window.__TAURI__.core.invoke("uninstall_mod", { id: btn.dataset.id });
        showToast("Mod uninstalled");
        renderModsContent();
      } catch (e) {
        showToast(`Failed: ${e}`);
      }
    });
  });
}

function renderGameBananaMods(mods) {
  const contentArea = document.getElementById("mods-content-area");
  if (!mods || mods.length === 0) {
    contentArea.innerHTML = '<div class="empty-state"><p>No mods found.</p></div>';
    return;
  }

  contentArea.innerHTML = mods.map((mod) => `
    <div class="mod-card">
      <div class="mod-info">
        <h4>${mod.name}</h4>
        <p class="mod-type">${mod.category || "Roblox Mod"}</p>
        <p class="mod-size">${formatFileSize(mod.filesize)}</p>
      </div>
      <button class="btn-install-mod secondary-btn" data-url="${mod.download_url}" style="padding: 4px 10px; font-size: 11px;">Install</button>
    </div>
  `).join("");

  contentArea.querySelectorAll(".btn-install-mod").forEach((btn) => {
    btn.addEventListener("click", async () => {
      btn.textContent = "Installing...";
      try {
        await window.__TAURI__.core.invoke("install_mod", { url: btn.dataset.url });
        showToast("Mod installed!");
        switchModTab("installed");
      } catch (e) {
        showToast(`Failed: ${e}`);
        btn.textContent = "Install";
      }
    });
  });
}

function renderFishstrapMods(mods) {
  const contentArea = document.getElementById("mods-content-area");
  if (!mods || mods.length === 0) {
    contentArea.innerHTML = '<div class="empty-state"><p>No mods found.</p></div>';
    return;
  }

  contentArea.innerHTML = mods.map((mod) => `
    <div class="mod-card">
      <div class="mod-info">
        <h4>${mod.name}</h4>
        <p class="mod-type">${mod.type || "fishstrap"}</p>
      </div>
      <button class="btn-install-mod secondary-btn" data-url="${mod.url}" style="padding: 4px 10px; font-size: 11px;">Install</button>
    </div>
  `).join("");

  contentArea.querySelectorAll(".btn-install-mod").forEach((btn) => {
    btn.addEventListener("click", async () => {
      btn.textContent = "Installing...";
      try {
        await window.__TAURI__.core.invoke("install_mod", { url: btn.dataset.url });
        showToast("Mod installed!");
        switchModTab("installed");
      } catch (e) {
        showToast(`Failed: ${e}`);
        btn.textContent = "Install";
      }
    });
  });
}

function formatFileSize(bytes) {
  if (!bytes) return "Unknown";
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  return (bytes / (1024 * 1024)).toFixed(1) + " MB";
}

export function refreshMods() {
  renderModsContent();
}