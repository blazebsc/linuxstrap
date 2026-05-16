// Default config structure
export const defaultConfig = {
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
  enableSuperPerformance: false,
  enableNetworkOptimization: false,
  enableWaylandClipboard: false,
  bringBackOof: true,
  useOldAvatarBackground: false,
  useOldCharacterSounds: false,
  cursorType: "default",
  customCursorPath: "",
  fontType: "default",
  customFontPath: "",
  themeColor: "#e74c3c",
  themePreset: "red",
  customFflags: {},
  audioDriver: "default",
  allowGamepad: true,
  useLibsecret: false,
  touchMode: "off",
  graphicsOptimization: "quality",
};

// Apply config to UI elements
export function applyConfigToUi(config) {
  document.getElementById("setting-rpc").checked = config.discordRpc;
  document.getElementById("setting-rpc-join").checked = config.discordRpcJoinButton;
  document.getElementById("setting-renderer").value = config.renderer || "vulkan";
  document.getElementById("setting-close-on-leave").checked = config.closeOnLeave;
  document.getElementById("setting-gamemode").checked = config.enableGamemode;
  document.getElementById("setting-hidpi").checked = config.enableHidpi;
  document.getElementById("setting-server-location").checked = config.serverLocationIndicator;
  document.getElementById("setting-console-exp").checked = config.useConsoleExperience;
  document.getElementById("setting-lighting").value = config.lightingTechnology || "default";
  document.getElementById("setting-texture").value = config.textureQuality || "default";
  document.getElementById("setting-msaa").value = config.msaa || "default";
  document.getElementById("setting-bubble-chat").checked = config.disableBubbleChat;
  document.getElementById("setting-player-shadows").checked = config.disablePlayerShadows;
  document.getElementById("setting-super-performance").checked = config.enableSuperPerformance;
  document.getElementById("setting-network-optimization").checked = config.enableNetworkOptimization;
  document.getElementById("setting-wayland-clipboard").checked = config.enableWaylandClipboard;
  document.getElementById("setting-bring-back-oof").checked = config.bringBackOof;
  document.getElementById("setting-old-sounds").checked = config.useOldCharacterSounds;
  document.getElementById("setting-cursor-type").value = config.cursorType || "default";
  document.getElementById("setting-font-type").value = config.fontType || "default";
  document.getElementById("setting-theme-color").value = config.themeColor || "#e74c3c";
  document.getElementById("setting-audio-driver").value = config.audioDriver || "default";
  document.getElementById("setting-allow-gamepad").checked = config.allowGamepad;
  document.getElementById("setting-use-libsecret").checked = config.useLibsecret;
  document.getElementById("setting-touch-mode").value = config.touchMode || "off";
  document.getElementById("setting-graphics-optimization").value = config.graphicsOptimization || "quality";

  // Custom cursor path
  if (config.customCursorPath) {
    document.getElementById("label-cursor-path").textContent = config.customCursorPath.split("/").pop();
    document.getElementById("custom-cursor-row").style.display = "flex";
  }

  // Custom font path
  if (config.customFontPath) {
    const name = config.customFontPath.split("/").pop();
    document.getElementById("label-font-path").textContent = name.length > 20 ? name.slice(0, 17) + "..." : name;
    document.getElementById("custom-font-row").style.display = "flex";
  }

  // Update theme preset buttons
  const presetBtn = document.querySelector(`.preset-btn[data-color="${config.themeColor.toLowerCase()}"]`);
  if (presetBtn) {
    document.querySelectorAll(".preset-btn").forEach((b) => b.classList.remove("active"));
    presetBtn.classList.add("active");
  }

  // Custom FFlags
  renderCustomFflags(config.customFflags || {});
}

// Collect config from UI elements
export function collectConfigFromUi() {
  return {
    discordRpc: document.getElementById("setting-rpc").checked,
    discordRpcJoinButton: document.getElementById("setting-rpc-join").checked,
    renderer: document.getElementById("setting-renderer").value,
    closeOnLeave: document.getElementById("setting-close-on-leave").checked,
    enableGamemode: document.getElementById("setting-gamemode").checked,
    enableHidpi: document.getElementById("setting-hidpi").checked,
    serverLocationIndicator: document.getElementById("setting-server-location").checked,
    useConsoleExperience: document.getElementById("setting-console-exp").checked,
    lightingTechnology: document.getElementById("setting-lighting").value,
    textureQuality: document.getElementById("setting-texture").value,
    msaa: document.getElementById("setting-msaa").checked ? "on" : document.getElementById("setting-msaa").value,
    disableBubbleChat: document.getElementById("setting-bubble-chat").checked,
    disablePlayerShadows: document.getElementById("setting-player-shadows").checked,
    enableSuperPerformance: document.getElementById("setting-super-performance").checked,
    enableNetworkOptimization: document.getElementById("setting-network-optimization").checked,
    enableWaylandClipboard: document.getElementById("setting-wayland-clipboard").checked,
    bringBackOof: document.getElementById("setting-bring-back-oof").checked,
    useOldCharacterSounds: document.getElementById("setting-old-sounds").checked,
    cursorType: document.getElementById("setting-cursor-type").value,
    fontType: document.getElementById("setting-font-type").value,
    themeColor: document.getElementById("setting-theme-color").value,
    audioDriver: document.getElementById("setting-audio-driver").value,
    allowGamepad: document.getElementById("setting-allow-gamepad").checked,
    useLibsecret: document.getElementById("setting-use-libsecret").checked,
    touchMode: document.getElementById("setting-touch-mode").value,
    graphicsOptimization: document.getElementById("setting-graphics-optimization").value,
  };
}