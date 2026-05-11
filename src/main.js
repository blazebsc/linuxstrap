const { invoke } = window.__TAURI__.core;

// State
let config = {
  discordRpc: true,
  discordRpcJoinButton: true,
  maxFps: 60,
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
const maxFpsInput = document.getElementById("setting-maxfps");
const closeOnLeaveSwitch = document.getElementById("setting-close-on-leave");
const rpcJoinSwitch = document.getElementById("setting-rpc-join");
const serverLocationSwitch = document.getElementById("setting-server-location");
const hidpiSwitch = document.getElementById("setting-hidpi");
const gamemodeSwitch = document.getElementById("setting-gamemode");
const consoleExpSwitch = document.getElementById("setting-console-exp");

const lightingSelect = document.getElementById("setting-lighting");
const textureSelect = document.getElementById("setting-texture");
const msaaSelect = document.getElementById("setting-msaa");
const bubbleChatSwitch = document.getElementById("setting-bubble-chat");
const playerShadowsSwitch = document.getElementById("setting-player-shadows");

const oldAvatarBgSwitch = document.getElementById("setting-old-avatar-bg");
const oldSoundsSwitch = document.getElementById("setting-old-sounds");
const cursorTypeSelect = document.getElementById("setting-cursor-type");

const btnAddFflag = document.getElementById("btn-add-fflag");

async function loadConfig() {
  try {
    config = await invoke("get_config");
    
    // Update UI
    rpcSwitch.checked = config.discordRpc;
    rpcJoinSwitch.checked = config.discordRpcJoinButton;
    rendererSelect.value = config.renderer || "vulkan";
    maxFpsInput.value = config.maxFps || 60;
    closeOnLeaveSwitch.checked = config.closeOnLeave;
    serverLocationSwitch.checked = config.serverLocationIndicator;
    hidpiSwitch.checked = config.enableHidpi;
    gamemodeSwitch.checked = config.enableGamemode;
    consoleExpSwitch.checked = config.useConsoleExperience;
    lightingSelect.value = config.lightingTechnology || "default";
    textureSelect.value = config.textureQuality || "default";
    msaaSelect.value = config.msaa || "default";
    bubbleChatSwitch.checked = config.disableBubbleChat;
    playerShadowsSwitch.checked = config.disablePlayerShadows;

    oldAvatarBgSwitch.checked = config.useOldAvatarBackground;
    oldSoundsSwitch.checked = config.useOldCharacterSounds;
    cursorTypeSelect.value = config.cursorType || "default";

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
    saveConfig();
  });

  // Custom FFlags Logic
  btnAddFflag.addEventListener("click", () => {
    createFflagRow("", "");
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
    debounceTimeout = setTimeout(() => {
      saveCustomFflagsFromUI();
    }, 500);
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
async function loadPatches() {
  const listEl = document.getElementById("patches-list");
  try {
    const patches = await invoke("fetch_patch_index");
    
    listEl.innerHTML = "";
    
    if (patches.length === 0) {
      listEl.innerHTML = `
        <div class="empty-state">
          <p>No patches available in the store right now.</p>
        </div>
      `;
      return;
    }

    patches.forEach(patch => {
      const isInstalled = config.patches.includes(patch.url);
      
      const item = document.createElement("div");
      item.className = "patch-item";
      
      item.innerHTML = `
        <div class="patch-info">
          <h3>${patch.title}</h3>
          <p>${patch.url}</p>
        </div>
        <div class="patch-action">
          <button class="${isInstalled ? 'secondary-btn' : 'primary-btn'} patch-btn" data-url="${patch.url}">
            ${isInstalled ? 'Uninstall' : 'Install'}
          </button>
        </div>
      `;
      
      const btn = item.querySelector(".patch-btn");
      btn.addEventListener("click", async () => {
        const url = btn.getAttribute("data-url");
        
        btn.disabled = true;
        btn.textContent = isInstalled ? "Uninstalling..." : "Installing...";
        
        try {
          if (isInstalled) {
            await invoke("uninstall_patch", { url });
            showToast(`Uninstalled ${patch.title}`);
          } else {
            await invoke("install_patch", { url });
            showToast(`Installed ${patch.title}`);
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
        <p>Failed to load patch store.</p>
        <p style="font-size: 12px; margin-top: 8px;">${e}</p>
      </div>
    `;
  }
}

