// Tauri API wrappers
const { invoke } = window.__TAURI__.core;

export async function getConfig() {
  return await invoke("get_config");
}

export async function saveConfig(config) {
  return await invoke("save_config", { config });
}

export async function launchSober() {
  return await invoke("launch_sober");
}

export async function getGpus() {
  return await invoke("get_gpus");
}

export async function killSober() {
  return await invoke("kill_sober");
}

export async function resetSoberConfig() {
  return await invoke("reset_sober_config");
}

export async function openSoberConfig() {
  return await invoke("open_sober_config");
}

export async function generateTheme(colorHex) {
  return await invoke("generate_theme", { colorHex });
}

export async function getMods() {
  return await invoke("get_mods");
}

export async function installMod(url) {
  return await invoke("install_mod", { url });
}

export async function uninstallMod(id) {
  return await invoke("uninstall_mod", { id });
}

export async function openOverlayFolder() {
  return await invoke("open_overlay_folder");
}

export async function openUrl(url) {
  return await invoke("open_url", { url });
}

export async function pickFile() {
  return await invoke("pick_file");
}

export async function pickFolder() {
  return await invoke("pick_folder");
}

export async function installCursor(path) {
  return await invoke("install_cursor", { path });
}

export async function installFont(path) {
  return await invoke("install_font", { path });
}

export async function recolorFont(path, color) {
  return await invoke("recolor_font", { path, color });
}

export async function wakeGpu() {
  return await invoke("wake_gpu");
}

export async function fixXdgPortals() {
  return await invoke("fix_xdg_portals");
}

export async function checkSse42() {
  return await invoke("check_sse42");
}

export async function checkSoberStatus() {
  return await invoke("check_sober_status");
}

export async function getGameBananaMods(page = 1) {
  return await invoke("get_gamebanana_mods", { page });
}

export async function getFishstrapMods() {
  return await invoke("get_fishstrap_mods");
}