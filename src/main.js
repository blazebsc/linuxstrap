const { invoke } = window.__TAURI__.core;

// State
let config = {
  discordRpc: true,
  discordRpcJoinButton: true,
  patches: [],
  renderer: "vulkan",
  closeOnLeave: false,
  enableGamemode: true,
  enableHidpi: false,
  serverLocationIndicator: true,
  useConsoleExperience: false,
  lightingTechnology: "default",
  textureQuality: "default",
  msaa: "default",
  disableBubbleChat: false,
  disablePlayerShadows: false,
  useOldAvatarBackground: false,
  useOldCharacterSounds: false,
  cursorType: "default",
  customCursorPath: "",
  fontType: "default",
  customFontPath: "",
  customFflags: {}
};

// UI Elements
const sidebar = document.getElementById("sidebar");
const toggleSidebarBtn = document.getElementById("toggleSidebar");
const navItems = document.querySelectorAll(".nav-item");
const views = document.querySelectorAll(".view");

const btnPlay = document.getElementById("btn-play");
const btnStore = document.getElementById("btn-store");
const btnOpenMods = document.getElementById("btn-open-mods");
const btnSoberConfig = document.getElementById("btn-sober-config");

// Settings Elements
const rpcSwitch = document.getElementById("setting-rpc");
const rendererSelect = document.getElementById("setting-renderer");
const closeOnLeaveSwitch = document.getElementById("setting-close-on-leave");
const rpcJoinSwitch = document.getElementById("setting-rpc-join");
const serverLocationSwitch = document.getElementById("setting-server-location");
const hidpiSwitch = document.getElementById("setting-hidpi");
const gamemodeSwitch = document.getElementById("setting-gamemode");
const consoleExpSwitch = document.getElementById("setting-console-exp");

// New Sober Elements
const allowGamepadSwitch = document.getElementById("setting-allow-gamepad");
const touchModeSelect = document.getElementById("setting-touch-mode");
const useLibsecretSwitch = document.getElementById("setting-use-libsecret");
const graphicsOptSelect = document.getElementById("setting-graphics-optimization");

const lightingSelect = document.getElementById("setting-lighting");
const textureSelect = document.getElementById("setting-texture");
const msaaSelect = document.getElementById("setting-msaa");
const bubbleChatSwitch = document.getElementById("setting-bubble-chat");
const playerShadowsSwitch = document.getElementById("setting-player-shadows");

const oldAvatarBgSwitch = document.getElementById("setting-old-avatar-bg");
const oldSoundsSwitch = document.getElementById("setting-old-sounds");
const cursorTypeSelect = document.getElementById("setting-cursor-type");
const fontTypeSelect = document.getElementById("setting-font-type");

const customCursorRow = document.getElementById("custom-cursor-row");
const btnPickCursor = document.getElementById("btn-pick-cursor");
const labelCursorPath = document.getElementById("label-cursor-path");

const customFontRow = document.getElementById("custom-font-row");
const btnPickFont = document.getElementById("btn-pick-font");
const labelFontPath = document.getElementById("label-font-path");

const btnAddFflag = document.getElementById("btn-add-fflag");
const btnImportFflags = document.getElementById("btn-import-fflags");

async function loadConfig() {
  try {
    config = await invoke("get_config");
    
    // Update UI
    rpcSwitch.checked = config.discordRpc;
    rpcJoinSwitch.checked = config.discordRpcJoinButton;
    rendererSelect.value = config.renderer || "vulkan";
    closeOnLeaveSwitch.checked = config.closeOnLeave;
    serverLocationSwitch.checked = config.serverLocationIndicator;
    hidpiSwitch.checked = config.enableHidpi;
    gamemodeSwitch.checked = config.enableGamemode;
    consoleExpSwitch.checked = config.useConsoleExperience;

    allowGamepadSwitch.checked = config.allowGamepadPermission;
    touchModeSelect.value = config.touchMode || "off";
    useLibsecretSwitch.checked = config.useLibsecret;
    graphicsOptSelect.value = config.graphicsOptimizationMode || "quality";

    lightingSelect.value = config.lightingTechnology || "default";
    textureSelect.value = config.textureQuality || "default";
    msaaSelect.value = config.msaa || "default";
    bubbleChatSwitch.checked = config.disableBubbleChat;
    playerShadowsSwitch.checked = config.disablePlayerShadows;

    oldAvatarBgSwitch.checked = config.useOldAvatarBackground;
    oldSoundsSwitch.checked = config.useOldCharacterSounds;
    cursorTypeSelect.value = config.cursorType || "default";
    fontTypeSelect.value = config.fontType || "default";

    labelCursorPath.innerText = config.customCursorPath ? config.customCursorPath.split('/').pop() : "No file";
    labelFontPath.innerText = config.customFontPath ? config.customFontPath.split('/').pop() : "No file";

    updateVisibility();
    renderCustomFflags();
  } catch (error) {
    console.error("Failed to load config:", error);
  }
}

async function saveConfig() {
  try {
    await invoke("save_config", { config });
  } catch (error) {
    console.error("Failed to save config:", error);
  }
}

function updateVisibility() {
  customCursorRow.style.display = config.cursorType === "custom" ? "flex" : "none";
  customFontRow.style.display = config.fontType === "custom" ? "flex" : "none";
}

// Event Listeners
window.addEventListener("DOMContentLoaded", () => {
  loadConfig();

  // Sidebar Toggle
  document.getElementById("toggle-sidebar").addEventListener("click", () => {
    sidebar.classList.toggle("collapsed");
  });

  // Navigation
  navItems.forEach(item => {
    item.addEventListener("click", () => {
      // Update active nav
      navItems.forEach(n => n.classList.remove("active"));
      item.classList.add("active");

      // Update active view
      const targetView = item.getAttribute("data-view");
      views.forEach(v => v.classList.remove("active"));
      document.getElementById(`view-${targetView}`).classList.add("active");
    });
  });

  // Settings Changes
  rpcSwitch.addEventListener("change", (e) => {
    config.discordRpc = e.target.checked;
    saveConfig();
  });

  rpcJoinSwitch.addEventListener("change", (e) => {
    config.discordRpcJoinButton = e.target.checked;
    saveConfig();
  });

  closeOnLeaveSwitch.addEventListener("change", (e) => {
    config.closeOnLeave = e.target.checked;
    saveConfig();
  });

  serverLocationSwitch.addEventListener("change", (e) => {
    config.serverLocationIndicator = e.target.checked;
    saveConfig();
  });

  hidpiSwitch.addEventListener("change", (e) => {
    config.enableHidpi = e.target.checked;
    saveConfig();
  });

  gamemodeSwitch.addEventListener("change", (e) => {
    config.enableGamemode = e.target.checked;
    saveConfig();
  });

  consoleExpSwitch.addEventListener("change", (e) => {
    config.useConsoleExperience = e.target.checked;
    saveConfig();
  });

  allowGamepadSwitch.addEventListener("change", (e) => {
    config.allowGamepadPermission = e.target.checked;
    saveConfig();
  });

  touchModeSelect.addEventListener("change", (e) => {
    config.touchMode = e.target.value;
    saveConfig();
  });

  useLibsecretSwitch.addEventListener("change", (e) => {
    config.useLibsecret = e.target.checked;
    saveConfig();
  });

  graphicsOptSelect.addEventListener("change", (e) => {
    config.graphicsOptimizationMode = e.target.value;
    saveConfig();
  });

  rendererSelect.addEventListener("change", (e) => {
    config.renderer = e.target.value;
    saveConfig();
  });

  lightingSelect.addEventListener("change", (e) => {
    config.lightingTechnology = e.target.value;
    saveConfig();
  });

  textureSelect.addEventListener("change", (e) => {
    config.textureQuality = e.target.value;
    saveConfig();
  });

  msaaSelect.addEventListener("change", (e) => {
    config.msaa = e.target.value;
    saveConfig();
  });

  bubbleChatSwitch.addEventListener("change", (e) => {
    config.disableBubbleChat = e.target.checked;
    saveConfig();
  });

  playerShadowsSwitch.addEventListener("change", (e) => {
    config.disablePlayerShadows = e.target.checked;
    saveConfig();
  });

  oldAvatarBgSwitch.addEventListener("change", (e) => {
    config.useOldAvatarBackground = e.target.checked;
    saveConfig();
  });

  oldSoundsSwitch.addEventListener("change", (e) => {
    config.useOldCharacterSounds = e.target.checked;
    saveConfig();
  });

  cursorTypeSelect.addEventListener("change", (e) => {
    config.cursorType = e.target.value;
    updateVisibility();
    saveConfig();
  });

  fontTypeSelect.addEventListener("change", (e) => {
    config.fontType = e.target.value;
    updateVisibility();
    saveConfig();
  });

  const { open } = window.__TAURI__.plugin_dialog || window.__TAURI__.dialog;

  btnPickCursor.addEventListener("click", async () => {
    try {
      const selected = await window.__TAURI__.plugin_dialog.open({
        multiple: false,
        filters: [{ name: 'Image', extensions: ['png'] }]
      });
      if (selected) {
        config.customCursorPath = selected;
        labelCursorPath.innerText = selected.split('/').pop() || selected.split('\\').pop();
        saveConfig();
      }
    } catch (e) {
      console.error(e);
    }
  });

  btnPickFont.addEventListener("click", async () => {
    try {
      const selected = await window.__TAURI__.plugin_dialog.open({
        multiple: false,
        filters: [{ name: 'Font', extensions: ['ttf', 'otf'] }]
      });
      if (selected) {
        config.customFontPath = selected;
        labelFontPath.innerText = selected.split('/').pop() || selected.split('\\').pop();
        saveConfig();
      }
    } catch (e) {
      console.error(e);
    }
  });

  // Custom FFlags Logic
  btnAddFflag.addEventListener("click", () => {
    createFflagRow("", "");
  });

  btnImportFflags.addEventListener("click", async () => {
    try {
      const selected = await window.__TAURI__.plugin_dialog.open({
        multiple: false,
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (selected) {
        // Read file contents (Tauri v2 doesn't have an easy fs read from JS without plugin, so we can use a new backend command)
        const newFlags = await invoke("import_fflags_json", { path: selected });
        // merge into config
        config.customFflags = { ...config.customFflags, ...newFlags };
        renderCustomFflags();
        saveConfig();
        showToast("Imported FastFlags successfully.");
      }
    } catch (e) {
      console.error(e);
      showToast("Failed to import: " + e, true);
    }
  });

  // Launch Sober
  btnSoberConfig.addEventListener("click", async () => {
    try {
      await invoke("launch_sober_config");
      showToast("Opened Sober Settings.");
    } catch (e) {
      console.error("Failed to open Sober Settings:", e);
      showToast("Failed to open Sober Settings: " + e, true);
    }
  });

  btnPlay.addEventListener("click", async () => {
    const originalText = btnPlay.innerHTML;
    btnPlay.innerHTML = "Launching...";
    btnPlay.disabled = true;
    
    try {
      await invoke("launch_sober");
      showToast("Sober launched successfully.");
    } catch (e) {
      console.error("Failed to launch Sober:", e);
      showToast("Failed to launch Sober: " + e, true);
    } finally {
      setTimeout(() => {
        btnPlay.innerHTML = originalText;
        btnPlay.disabled = false;
      }, 1000);
    }
  });

  // Load Patches
  btnStore.addEventListener("click", () => {
    // Switch to patches view manually
    navItems.forEach(n => n.classList.remove("active"));
    document.querySelector('[data-view="patches"]').classList.add("active");
    views.forEach(v => v.classList.remove("active"));
    document.getElementById("view-patches").classList.add("active");
  });

  btnOpenMods.addEventListener("click", async () => {
    try {
      await invoke("open_mod_folder");
      showToast("Opened overlay folder");
    } catch(e) {
      showToast("Error opening folder: " + e, true);
    }
  });

  loadPatches();
});

// Toast System
function showToast(message, isError = false) {
  const container = document.getElementById("toast-container");
  const toast = document.createElement("div");
  toast.className = `toast ${isError ? 'error' : ''}`;
  toast.textContent = message;
  
  container.appendChild(toast);
  
  // Remove after animation (3s total)
  setTimeout(() => {
    toast.remove();
  }, 3000);
}

// Custom FFlags System
function renderCustomFflags() {
  customFflagsList.innerHTML = "";
  
  for (const [key, value] of Object.entries(config.customFflags)) {
    createFflagRow(key, value);
  }
}

function createFflagRow(key = "", value = "") {
  const row = document.createElement("div");
  row.className = "fflag-row";
  
  const keyInput = document.createElement("input");
  keyInput.type = "text";
  keyInput.className = "fflag-key-input";
  keyInput.placeholder = "FFlagName";
  keyInput.value = key;
  
  const valInput = document.createElement("input");
  valInput.type = "text";
  valInput.className = "fflag-val-input";
  valInput.placeholder = "Value (true, false, etc)";
  valInput.value = value.toString();
  
  const delBtn = document.createElement("button");
  delBtn.className = "fflag-del-btn";
  delBtn.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>`;
  
  let debounceTimeout;
  const saveRowChange = () => {
    clearTimeout(debounceTimeout);
    debounceTimeout = setTimeout(async () => {
      saveCustomFflagsFromUI();
      if (keyInput.value.trim().length > 0) {
        try {
          const isValid = await invoke("validate_fflag", { flag: keyInput.value.trim() });
          if (!isValid) {
            showToast(`Warning: ${keyInput.value.trim()} is not in the public tracker. Use at your own risk.`, true);
          }
        } catch (e) {
          console.error(e);
        }
      }
    }, 1000);
  };
  
  keyInput.addEventListener("input", saveRowChange);
  valInput.addEventListener("input", saveRowChange);
  
  delBtn.addEventListener("click", () => {
    row.remove();
    saveCustomFflagsFromUI();
  });
  
  row.appendChild(keyInput);
  row.appendChild(valInput);
  row.appendChild(delBtn);
  
  customFflagsList.appendChild(row);
}

function saveCustomFflagsFromUI() {
  const newFflags = {};
  const rows = customFflagsList.querySelectorAll(".fflag-row");
  
  rows.forEach(row => {
    let key = row.querySelector(".fflag-key-input").value.trim();
    let valStr = row.querySelector(".fflag-val-input").value.trim();
    
    // Sanitize key (remove spaces)
    key = key.replace(/\s+/g, '');

    if (key !== "") {
      let finalVal = valStr;
      
      // Auto convert booleans
      if (valStr.toLowerCase() === "true") {
        finalVal = true;
      } else if (valStr.toLowerCase() === "false") {
        finalVal = false;
      } else if (!isNaN(valStr) && valStr !== "") {
        // Keep as string if it is an integer for FInt flags, but if it starts with FInt,
        // Sober usually handles string numbers fine. If it's pure number, we can parse it 
        // to be safe or leave as string. For Roblox FInts, string "100" or int 100 works.
        // Let's pass it as a number if it is safely parsable to match JSON cleanly, unless
        // it requires string format. We will leave it as string unless the user types pure number.
        finalVal = Number(valStr);
      }
      
      newFflags[key] = finalVal;
    }
  });
  
  config.customFflags = newFflags;
  saveConfig();
}

// Patch Store System
let currentModsView = 'installed';
let gamebananaPage = 1;
let cachedFishstrapMods = [];
let cachedGamebananaMods = [];
let allLoadedMods = []; // Combined cache for lookups

async function loadPatches() {
  const listEl = document.getElementById("mods-content-area");
  
  listEl.innerHTML = `
    <div class="empty-state">
      <svg class="spinner" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="opacity: 0.5; margin-bottom: 12px; animation: spin 1s linear infinite;"><line x1="12" y1="2" x2="12" y2="6"></line><line x1="12" y1="18" x2="12" y2="22"></line><line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line><line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line><line x1="2" y1="12" x2="6" y2="12"></line><line x1="18" y1="12" x2="22" y2="12"></line><line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line><line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line></svg>
      <p>Loading...</p>
    </div>
  `;

  try {
    let modsToDisplay = [];

    if (currentModsView === 'installed') {
      // Installed mods are fetched from config.patches array, but we need details.
      // We will try to find them in our cache, or create basic info.
      for (const patchKey of config.patches) {
        let [source, id] = patchKey.split(":");
        // Try to find in cache
        let found = allLoadedMods.find(m => m.source === source && m.id === id);
        if (found) {
          modsToDisplay.push(found);
        } else {
          modsToDisplay.push({
            id: id,
            title: `${source} Mod (${id})`,
            author: "Unknown",
            source: source,
            image_url: null
          });
        }
      }
    } else if (currentModsView === 'gamebanana') {
      if (cachedGamebananaMods.length === 0) {
        cachedGamebananaMods = await invoke("fetch_gamebanana_mods", { page: gamebananaPage });
        allLoadedMods = [...allLoadedMods, ...cachedGamebananaMods];
      }
      modsToDisplay = cachedGamebananaMods;
    } else if (currentModsView === 'fishstrap') {
      if (cachedFishstrapMods.length === 0) {
        cachedFishstrapMods = await invoke("fetch_fishstrap_mods");
        allLoadedMods = [...allLoadedMods, ...cachedFishstrapMods];
      }
      modsToDisplay = cachedFishstrapMods;
    }
    
    listEl.innerHTML = "";
    
    if (modsToDisplay.length === 0) {
      listEl.innerHTML = `
        <div class="empty-state">
          <p>No mods found.</p>
        </div>
      `;
      return;
    }

    modsToDisplay.forEach(mod => {
      const patchKey = `${mod.source}:${mod.id}`;
      const isInstalled = config.patches.includes(patchKey);
      
      const item = document.createElement("div");
      item.className = "patch-item";
      
      item.innerHTML = `
        <div style="display: flex; gap: 12px; align-items: center; width: 100%;">
          <div style="width: 48px; height: 48px; border-radius: 4px; overflow: hidden; background: rgba(255,255,255,0.05); flex-shrink: 0; display: flex; align-items: center; justify-content: center;">
            ${mod.image_url ? `<img src="${mod.image_url}" style="width: 100%; height: 100%; object-fit: cover;" onerror="this.style.display='none'">` : '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="opacity: 0.5;"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect><line x1="8" y1="21" x2="16" y2="21"></line><line x1="12" y1="17" x2="12" y2="21"></line></svg>'}
          </div>
          <div class="patch-info" style="flex-grow: 1;">
            <h3 style="margin: 0; font-size: 14px;">${mod.title}</h3>
            <p style="margin: 2px 0 0 0; font-size: 11px; opacity: 0.7;">By ${mod.author} • ${mod.source}</p>
          </div>
          <div class="patch-action" style="flex-shrink: 0;">
            <button class="${isInstalled ? 'secondary-btn' : 'primary-btn'} patch-btn" data-id="${mod.id}" data-source="${mod.source}">
              ${isInstalled ? 'Uninstall' : 'Install'}
            </button>
          </div>
        </div>
      `;
      
      const btn = item.querySelector(".patch-btn");
      btn.addEventListener("click", async () => {
        const id = btn.getAttribute("data-id");
        const source = btn.getAttribute("data-source");
        const key = `${source}:${id}`;
        
        btn.disabled = true;
        btn.textContent = isInstalled ? "Uninstalling..." : "Installing...";
        
        try {
          if (isInstalled) {
            await invoke("uninstall_mod", { id, source });
            showToast(`Uninstalled ${mod.title}`);
          } else {
            await invoke("install_mod", { id, source });
            showToast(`Installed ${mod.title}`);
          }
          // Reload config to sync states and re-render
          await loadConfig();
          await loadPatches();
        } catch (e) {
          showToast(`Error: ${e}`, true);
          btn.disabled = false;
          btn.textContent = isInstalled ? "Uninstall" : "Install";
        }
      });
      
      listEl.appendChild(item);
    });

  } catch (e) {
    listEl.innerHTML = `
      <div class="empty-state" style="color: #e06c75;">
        <p>Failed to load mods.</p>
        <p style="font-size: 12px; margin-top: 8px;">${e}</p>
      </div>
    `;
  }
}

document.getElementById("tab-installed").addEventListener("click", (e) => {
  document.querySelectorAll(".tabs button").forEach(b => b.style.background = "");
  e.target.style.background = "rgba(255,255,255,0.2)";
  currentModsView = 'installed';
  loadPatches();
});

document.getElementById("tab-gamebanana").addEventListener("click", (e) => {
  document.querySelectorAll(".tabs button").forEach(b => b.style.background = "");
  e.target.style.background = "rgba(255,255,255,0.2)";
  currentModsView = 'gamebanana';
  loadPatches();
});

document.getElementById("tab-fishstrap").addEventListener("click", (e) => {
  document.querySelectorAll(".tabs button").forEach(b => b.style.background = "");
  e.target.style.background = "rgba(255,255,255,0.2)";
  currentModsView = 'fishstrap';
  loadPatches();
});

