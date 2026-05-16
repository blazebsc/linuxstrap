import * as api from "./api.js";
import { defaultConfig, applyConfigToUi, collectConfigFromUi } from "./config.js";
import { initNavigation, initSidebar, showToast, showError, showSuccess, initThemePresets, renderCustomFflags, addFflagRow, collectFflags, runSystemChecks, showThemeProgress, updateThemeProgress } from "./ui.js";
import { initMods, refreshMods } from "./mods.js";

let config = { ...defaultConfig };

async function init() {
  console.log("[linuxstrap] Initializing...");

  initNavigation();
  initSidebar();
  initThemePresets(config);
  initMods();

  await loadConfig();
  runSystemChecks();
  setInterval(runSystemChecks, 5000);

  setupEventListeners();
  setupThemeListener();
}

async function loadConfig() {
  try {
    const loaded = await api.getConfig();
    config = { ...defaultConfig, ...loaded };
    applyConfigToUi(config);
    console.log("[linuxstrap] Config loaded");
  } catch (e) {
    console.error("[linuxstrap] Failed to load config:", e);
  }
}

async function saveConfig() {
  try {
    const uiConfig = collectConfigFromUi();
    config = { ...config, ...uiConfig, customFflags: collectFflags() };
    await api.saveConfig(config);
    console.log("[linuxstrap] Config saved");
  } catch (e) {
    console.error("[linuxstrap] Failed to save config:", e);
  }
}

function setupEventListeners() {
  document.getElementById("btn-play").addEventListener("click", async () => {
    try {
      await api.launchSober();
      showToast("Launching Sober...");
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-store").addEventListener("click", async () => {
    window.open("https://www.gamebanana.com/games/2879", "_blank");
  });

  document.getElementById("btn-open-mods").addEventListener("click", async () => {
    try {
      await api.openOverlayFolder();
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-sober-config").addEventListener("click", async () => {
    try {
      await api.openSoberConfig();
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-kill-sober-sidebar").addEventListener("click", async () => {
    try {
      await api.killSober();
      showToast("Sober killed");
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-reset-sober").addEventListener("click", async () => {
    try {
      await api.resetSoberConfig();
      showToast("Sober config reset");
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-kill-sober").addEventListener("click", async () => {
    try {
      await api.killSober();
      showToast("Sober killed");
      runSystemChecks();
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-xdg-portal").addEventListener("click", async () => {
    try {
      await api.fixXdgPortals();
      showToast("XDG portals configured");
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-pick-cursor").addEventListener("click", async () => {
    try {
      const path = await api.pickFile();
      if (path) {
        await api.installCursor(path);
        config.customCursorPath = path;
        document.getElementById("label-cursor-path").textContent = path.split("/").pop();
        document.getElementById("custom-cursor-row").style.display = "flex";
        showToast("Cursor installed");
        await saveConfig();
      }
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-pick-font-file").addEventListener("click", async () => {
    try {
      const path = await api.pickFile();
      if (path) {
        await api.installFont(path);
        config.customFontPath = path;
        const name = path.split("/").pop();
        document.getElementById("label-font-path").textContent = name.length > 20 ? name.slice(0, 17) + "..." : name;
        document.getElementById("custom-font-row").style.display = "flex";
        showToast("Font installed");
        await saveConfig();
      }
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-pick-font-folder").addEventListener("click", async () => {
    try {
      const path = await api.pickFolder();
      if (path) {
        config.customFontPath = path;
        const name = path.split("/").pop();
        document.getElementById("label-font-path").textContent = name.length > 20 ? name.slice(0, 17) + "..." : name;
        document.getElementById("custom-font-row").style.display = "flex";
        showToast("Font folder set");
        await saveConfig();
      }
    } catch (e) {
      showError(e);
    }
  });

  document.getElementById("btn-recolor-font").addEventListener("click", async () => {
    const color = document.getElementById("setting-theme-color").value;
    if (config.customFontPath) {
      try {
        await api.recolorFont(config.customFontPath, color);
        showToast("Font recolored to " + color);
      } catch (e) {
        showError(e);
      }
    } else {
      showError("No font selected");
    }
  });

  document.getElementById("btn-add-fflag").addEventListener("click", addFflagRow);

  document.getElementById("btn-generate-theme").addEventListener("click", async () => {
    const color = document.getElementById("setting-theme-color").value;
    showThemeProgress(true);
    updateThemeProgress("processing", 10, "Starting theme generation...");

    try {
      await api.generateTheme(color);
      updateThemeProgress("complete", 100, "Theme applied!");
      showToast("Theme applied! Restart Sober for changes to take effect.");
      config.themeColor = color;
      await saveConfig();
    } catch (e) {
      showError(e);
      showThemeProgress(false);
    }
  });

  // Auto-save on setting changes
  document.querySelectorAll("input, select").forEach((el) => {
    if (el.id && !el.id.startsWith("btn-") && !el.id.includes("custom-fflags")) {
      el.addEventListener("change", saveConfig);
    }
  });
}

function setupThemeListener() {
  const { listen } = window.__TAURI__.event;
  listen("theme_progress", (event) => {
    const { status, progress, message } = event.payload;
    updateThemeProgress(status, progress, message);
    if (status === "complete") {
      setTimeout(() => showThemeProgress(false), 3000);
    }
  });
}

init();