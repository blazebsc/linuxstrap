use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

mod zip_extractor;
mod mods_api;
pub use mods_api::{fetch_fishstrap_mods, fetch_gamebanana_mods, install_mod, uninstall_mod};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LucemConfig {
    pub discord_rpc: bool,
    pub discord_rpc_join_button: bool,
    pub patches: Vec<String>,
    pub renderer: String,
    pub close_on_leave: bool,
    pub enable_gamemode: bool,
    pub enable_hidpi: bool,
    pub server_location_indicator: bool,
    pub use_console_experience: bool,

    // New Sober settings
    pub allow_gamepad_permission: bool,
    pub touch_mode: String,
    pub use_libsecret: bool,
    pub graphics_optimization_mode: String,

    // FastFlags presets
    pub lighting_technology: String, // "default", "voxel", "shadowmap", "future"
    pub texture_quality: String,     // "default", "0", "1", "2", "3", "4"
    pub msaa: String,                // "default", "off", "1", "2", "4", "8"
    pub disable_bubble_chat: bool,
    pub disable_player_shadows: bool,

    // Mods
    pub use_old_avatar_background: bool,
    pub use_old_character_sounds: bool,
    pub cursor_type: String, // "default", "2006", "2013", "custom"
    pub custom_cursor_path: String,
    pub font_type: String, // "default", "custom"
    pub custom_font_path: String,

    // Custom FFlags
    pub custom_fflags: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for LucemConfig {
    fn default() -> Self {
        Self {
            discord_rpc: true,
            discord_rpc_join_button: true,
            patches: vec![],
            renderer: "vulkan".into(),
            close_on_leave: false,
            enable_gamemode: true,
            enable_hidpi: false,
            server_location_indicator: true,
            use_console_experience: false,
            allow_gamepad_permission: false,
            touch_mode: "off".into(),
            use_libsecret: false,
            graphics_optimization_mode: "quality".into(),
            lighting_technology: "default".into(),
            texture_quality: "default".into(),
            msaa: "default".into(),
            disable_bubble_chat: false,
            disable_player_shadows: false,
            use_old_avatar_background: false,
            use_old_character_sounds: false,
            cursor_type: "default".into(),
            custom_cursor_path: "".into(),
            font_type: "default".into(),
            custom_font_path: "".into(),
            custom_fflags: std::collections::HashMap::new(),
        }
    }
}

// --- SOber Config Synchronization ---
fn get_sober_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".var/app/org.vinegarhq.Sober/config/sober/config.json");
    path
}

fn sync_to_sober_config(config: &LucemConfig) -> Result<(), String> {
    let path = get_sober_config_path();

    // Ensure parent dir exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    let mut comments = Vec::new();
    let mut json_lines = Vec::new();

    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        for line in content.lines() {
            if line.trim_start().starts_with("//") {
                comments.push(line.to_string());
            } else {
                json_lines.push(line.to_string());
            }
        }
    }

    let mut sober_json: Value = if !json_lines.is_empty() {
        let cleaned_content = json_lines.join("\n");
        serde_json::from_str(&cleaned_content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if let Some(obj) = sober_json.as_object_mut() {
        obj.insert(
            "discord_rpc_enabled".to_string(),
            serde_json::json!(config.discord_rpc),
        );
        obj.insert(
            "discord_rpc_show_join_button".to_string(),
            serde_json::json!(config.discord_rpc_join_button),
        );
        obj.insert(
            "use_opengl".to_string(),
            serde_json::json!(config.renderer == "opengl"),
        );
        obj.insert(
            "close_on_leave".to_string(),
            serde_json::json!(config.close_on_leave),
        );
        obj.insert(
            "enable_gamemode".to_string(),
            serde_json::json!(config.enable_gamemode),
        );
        obj.insert(
            "enable_hidpi".to_string(),
            serde_json::json!(config.enable_hidpi),
        );
        obj.insert(
            "server_location_indicator_enabled".to_string(),
            serde_json::json!(config.server_location_indicator),
        );
        obj.insert(
            "use_console_experience".to_string(),
            serde_json::json!(config.use_console_experience),
        );

        obj.insert(
            "allow_gamepad_permission".to_string(),
            serde_json::json!(config.allow_gamepad_permission),
        );
        obj.insert(
            "touch_mode".to_string(),
            serde_json::json!(config.touch_mode),
        );
        obj.insert(
            "use_libsecret".to_string(),
            serde_json::json!(config.use_libsecret),
        );
        obj.insert(
            "graphics_optimization_mode".to_string(),
            serde_json::json!(config.graphics_optimization_mode),
        );

        // Setup FFlags
        let fflags = obj
            .entry("fflags".to_string())
            .or_insert_with(|| serde_json::json!({}));
        if let Some(fflags_obj) = fflags.as_object_mut() {
            // Lighting Technology
            fflags_obj.remove("DFFlagDebugRenderForceTechnologyVoxel");
            fflags_obj.remove("FFlagDebugForceFutureIsBrightPhase2");
            fflags_obj.remove("FFlagDebugForceFutureIsBrightPhase3");
            match config.lighting_technology.as_str() {
                "voxel" => {
                    fflags_obj.insert(
                        "DFFlagDebugRenderForceTechnologyVoxel".to_string(),
                        serde_json::json!(true),
                    );
                }
                "shadowmap" => {
                    fflags_obj.insert(
                        "FFlagDebugForceFutureIsBrightPhase2".to_string(),
                        serde_json::json!(true),
                    );
                }
                "future" => {
                    fflags_obj.insert(
                        "FFlagDebugForceFutureIsBrightPhase3".to_string(),
                        serde_json::json!(true),
                    );
                }
                _ => {} // Default
            }

            // Texture Quality
            fflags_obj.remove("DFFlagTextureQualityOverrideEnabled");
            fflags_obj.remove("DFIntTextureQualityOverride");
            if config.texture_quality != "default" {
                if let Ok(quality_level) = config.texture_quality.parse::<u8>() {
                    fflags_obj.insert(
                        "DFFlagTextureQualityOverrideEnabled".to_string(),
                        serde_json::json!(true),
                    );
                    fflags_obj.insert(
                        "DFIntTextureQualityOverride".to_string(),
                        serde_json::json!(quality_level),
                    );
                }
            }

            // MSAA
            fflags_obj.remove("FFlagDebugDisableMSAA");
            fflags_obj.remove("FIntMSAASampleCount");
            match config.msaa.as_str() {
                "off" => {
                    fflags_obj.insert("FFlagDebugDisableMSAA".to_string(), serde_json::json!(true));
                }
                "1" | "2" | "4" | "8" => {
                    fflags_obj.insert(
                        "FIntMSAASampleCount".to_string(),
                        serde_json::json!(config.msaa),
                    );
                }
                _ => {} // default
            }

            // Bubble Chat
            fflags_obj.remove("FFlagEnableBubbleChatFromChatService");
            if config.disable_bubble_chat {
                fflags_obj.insert(
                    "FFlagEnableBubbleChatFromChatService".to_string(),
                    serde_json::json!(false),
                );
            }

            // Player Shadows
            fflags_obj.remove("FIntRenderShadowIntensity");
            if config.disable_player_shadows {
                fflags_obj.insert(
                    "FIntRenderShadowIntensity".to_string(),
                    serde_json::json!("0"),
                );
            }

            // Custom FFlags
            for (key, val) in &config.custom_fflags {
                fflags_obj.insert(key.clone(), val.clone());
            }
        }
    }

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    sober_json.serialize(&mut ser).map_err(|e| e.to_string())?;
    let new_json_string = String::from_utf8(buf).map_err(|e| e.to_string())?;

    let mut final_content = comments.join("\n");
    if !final_content.is_empty() {
        final_content.push('\n');
    }
    final_content.push_str(&new_json_string);
    final_content.push('\n');

    fs::write(path, final_content).map_err(|e| e.to_string())
}

// --- Lucem Config ---
fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("lucem");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

#[tauri::command]
fn get_config() -> LucemConfig {
    let path = get_config_path();
    let mut config = if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        LucemConfig::default()
    };

    // Load any existing FastFlags from Sober that are NOT managed by presets
    let sober_path = get_sober_config_path();
    if sober_path.exists() {
        if let Ok(content) = fs::read_to_string(&sober_path) {
            let cleaned_content: String = content
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<&str>>()
                .join("\n");

            if let Ok(sober_json) = serde_json::from_str::<Value>(&cleaned_content) {
                // Read general options back from Sober to keep in sync
                if let Some(val) = sober_json
                    .get("discord_rpc_enabled")
                    .and_then(|v| v.as_bool())
                {
                    config.discord_rpc = val;
                }
                if let Some(val) = sober_json
                    .get("discord_rpc_show_join_button")
                    .and_then(|v| v.as_bool())
                {
                    config.discord_rpc_join_button = val;
                }
                if let Some(val) = sober_json.get("use_opengl").and_then(|v| v.as_bool()) {
                    config.renderer = if val {
                        "opengl".to_string()
                    } else {
                        "vulkan".to_string()
                    };
                }
                if let Some(val) = sober_json.get("close_on_leave").and_then(|v| v.as_bool()) {
                    config.close_on_leave = val;
                }
                if let Some(val) = sober_json.get("enable_gamemode").and_then(|v| v.as_bool()) {
                    config.enable_gamemode = val;
                }
                if let Some(val) = sober_json.get("enable_hidpi").and_then(|v| v.as_bool()) {
                    config.enable_hidpi = val;
                }
                if let Some(val) = sober_json
                    .get("server_location_indicator_enabled")
                    .and_then(|v| v.as_bool())
                {
                    config.server_location_indicator = val;
                }
                if let Some(val) = sober_json
                    .get("use_console_experience")
                    .and_then(|v| v.as_bool())
                {
                    config.use_console_experience = val;
                }
                if let Some(val) = sober_json
                    .get("allow_gamepad_permission")
                    .and_then(|v| v.as_bool())
                {
                    config.allow_gamepad_permission = val;
                }
                if let Some(val) = sober_json.get("touch_mode").and_then(|v| v.as_str()) {
                    config.touch_mode = val.to_string();
                }
                if let Some(val) = sober_json.get("use_libsecret").and_then(|v| v.as_bool()) {
                    config.use_libsecret = val;
                }
                if let Some(val) = sober_json
                    .get("graphics_optimization_mode")
                    .and_then(|v| v.as_str())
                {
                    config.graphics_optimization_mode = val.to_string();
                }

                if let Some(fflags_obj) = sober_json.get("fflags").and_then(|f| f.as_object()) {
                    let managed_keys = vec![
                        "DFFlagDebugRenderForceTechnologyVoxel",
                        "FFlagDebugForceFutureIsBrightPhase2",
                        "FFlagDebugForceFutureIsBrightPhase3",
                        "DFFlagTextureQualityOverrideEnabled",
                        "DFIntTextureQualityOverride",
                        "FFlagDebugDisableMSAA",
                        "FIntMSAASampleCount",
                        "FFlagEnableBubbleChatFromChatService",
                        "FIntRenderShadowIntensity",
                    ];

                    for (k, v) in fflags_obj {
                        if !managed_keys.contains(&k.as_str()) {
                            // Only insert if it doesn't already exist in custom_fflags, or overwrite it?
                            // Overwrite ensures the UI reflects actual Sober state
                            config.custom_fflags.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }

    config
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: LucemConfig) -> Result<(), String> {
    let path = get_config_path();
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;

    // Sync settings to Sober
    sync_to_sober_config(&config)?;
    sync_mods(&app, &config)?;
    Ok(())
}

fn sync_mods(app: &tauri::AppHandle, config: &LucemConfig) -> Result<(), String> {
    let overlay_dir = get_sober_overlay_dir();

    // Path to assets
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    let assets_dir = resource_dir.join("assets");

    // Old Avatar Background
    let bg_path = overlay_dir.join("ExtraContent/places/Mobile.rbxl");
    if config.use_old_avatar_background {
        if let Some(p) = bg_path.parent() {
            fs::create_dir_all(p).ok();
        }
        fs::copy(assets_dir.join("OldAvatarBackground.rbxl"), bg_path).ok();
    } else {
        if bg_path.exists() {
            fs::remove_file(bg_path).ok();
        }
    }

    // Old Character Sounds
    let sound_dir = overlay_dir.join("content/sounds");
    let sounds = vec![
        ("action_footsteps_plastic.mp3", "Sounds/OldWalk.mp3"),
        ("action_jump.mp3", "Sounds/OldJump.mp3"),
        ("action_get_up.mp3", "Sounds/OldGetUp.mp3"),
        ("action_falling.mp3", "Sounds/Empty.mp3"),
        ("action_jump_land.mp3", "Sounds/Empty.mp3"),
        ("action_swim.mp3", "Sounds/Empty.mp3"),
        ("impact_water.mp3", "Sounds/Empty.mp3"),
    ];
    if config.use_old_character_sounds {
        fs::create_dir_all(&sound_dir).ok();
        for (out_name, in_name) in sounds {
            fs::copy(assets_dir.join(in_name), sound_dir.join(out_name)).ok();
        }
    } else {
        for (out_name, _) in sounds {
            let p = sound_dir.join(out_name);
            if p.exists() {
                fs::remove_file(p).ok();
            }
        }
    }

    // Cursors
    let cursor_dir = overlay_dir.join("content/textures/Cursors/KeyboardMouse");
    let cursors = vec!["ArrowCursor.png", "ArrowFarCursor.png"];

    // First remove existing
    for c in &cursors {
        let p = cursor_dir.join(c);
        if p.exists() {
            fs::remove_file(p).ok();
        }
    }

    if config.cursor_type == "2006" {
        fs::create_dir_all(&cursor_dir).ok();
        for c in &cursors {
            fs::copy(
                assets_dir.join(format!("Cursor/From2006/{}", c)),
                cursor_dir.join(c),
            )
            .ok();
        }
    } else if config.cursor_type == "2013" {
        fs::create_dir_all(&cursor_dir).ok();
        for c in &cursors {
            fs::copy(
                assets_dir.join(format!("Cursor/From2013/{}", c)),
                cursor_dir.join(c),
            )
            .ok();
        }
    } else if config.cursor_type == "custom" && !config.custom_cursor_path.is_empty() {
        fs::create_dir_all(&cursor_dir).ok();
        for c in &cursors {
            fs::copy(&config.custom_cursor_path, cursor_dir.join(c)).ok();
        }
    }

    // Fonts
    let font_dir = overlay_dir.join("content/fonts");
    let fonts_to_replace = vec![
        "Arial.ttf",
        "Arialbd.ttf",
        "BuilderSans-Bold.ttf",
        "BuilderSans-BoldItalic.ttf",
        "BuilderSans-ExtraBold.ttf",
        "BuilderSans-ExtraBoldItalic.ttf",
        "BuilderSans-Italic.ttf",
        "BuilderSans-Light.ttf",
        "BuilderSans-LightItalic.ttf",
        "BuilderSans-Medium.ttf",
        "BuilderSans-MediumItalic.ttf",
        "BuilderSans-Regular.ttf",
        "ComicNeue-Angular-Bold.ttf",
        "ComicNeue-Angular-Light.ttf",
        "ComicNeue-Angular-Regular.ttf",
        "CourierPrime-Bold.ttf",
        "CourierPrime-Regular.ttf",
        "Creepster-Regular.ttf",
        "DenkOne-Regular.ttf",
        "Fondamento-Italic.ttf",
        "Fondamento-Regular.ttf",
        "FredokaOne-Regular.ttf",
        "Garamond-Bold.ttf",
        "Garamond-Regular.ttf",
        "GothamSSm-Black.otf",
        "GothamSSm-Bold.otf",
        "GothamSSm-Book.otf",
        "GothamSSm-BookItalic.otf",
        "GothamSSm-Light.otf",
        "GothamSSm-Medium.otf",
        "GrenzeGotisch-Bold.ttf",
        "GrenzeGotisch-Light.ttf",
        "GrenzeGotisch-Regular.ttf",
        "HighwayGothic.ttf",
        "JosefinSans-Bold.ttf",
        "JosefinSans-Light.ttf",
        "JosefinSans-Regular.ttf",
        "Jura-Bold.ttf",
        "Jura-Light.ttf",
        "Jura-Regular.ttf",
        "Kalam-Bold.ttf",
        "Kalam-Light.ttf",
        "Kalam-Regular.ttf",
        "LuckiestGuy-Regular.ttf",
        "Merriweather-Bold.ttf",
        "Merriweather-Light.ttf",
        "Merriweather-Regular.ttf",
        "Michroma-Regular.ttf",
        "Nunito-Bold.ttf",
        "Nunito-Light.ttf",
        "Nunito-Regular.ttf",
        "Oswald-Bold.ttf",
        "Oswald-Light.ttf",
        "Oswald-Regular.ttf",
        "PatrickHand-Regular.ttf",
        "PermanentMarker-Regular.ttf",
        "Roboto-Black.ttf",
        "Roboto-BlackItalic.ttf",
        "Roboto-Bold.ttf",
        "Roboto-BoldItalic.ttf",
        "Roboto-Italic.ttf",
        "Roboto-Light.ttf",
        "Roboto-LightItalic.ttf",
        "Roboto-Medium.ttf",
        "Roboto-MediumItalic.ttf",
        "Roboto-Regular.ttf",
        "Roboto-Thin.ttf",
        "Roboto-ThinItalic.ttf",
        "RobotoCondensed-Bold.ttf",
        "RobotoCondensed-Light.ttf",
        "RobotoCondensed-Regular.ttf",
        "RobotoMono-Bold.ttf",
        "RobotoMono-Light.ttf",
        "RobotoMono-Regular.ttf",
        "Sarpanch-Bold.ttf",
        "Sarpanch-Regular.ttf",
        "SciFiPt-bmg0.ttf",
        "SpecialElite-Regular.ttf",
        "TitilliumWeb-Bold.ttf",
        "TitilliumWeb-Light.ttf",
        "TitilliumWeb-Regular.ttf",
        "Ubuntu-Bold.ttf",
        "Ubuntu-Light.ttf",
        "Ubuntu-Regular.ttf",
        "Zekton-Bold.ttf",
        "Zekton-Regular.ttf",
    ];

    // Remove existing custom fonts
    for f in &fonts_to_replace {
        let p = font_dir.join(f);
        if p.exists() {
            fs::remove_file(p).ok();
        }
    }

    if config.font_type == "custom" && !config.custom_font_path.is_empty() {
        fs::create_dir_all(&font_dir).ok();
        for f in &fonts_to_replace {
            fs::copy(&config.custom_font_path, font_dir.join(f)).ok();
        }
    }

    Ok(())
}

#[tauri::command]
fn launch_sober() -> Result<(), String> {
    Command::new("sh")
        .arg("-c")
        .arg("nohup flatpak run org.vinegarhq.Sober > /dev/null 2>&1 &")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn launch_sober_config() -> Result<(), String> {
    Command::new("flatpak")
        .args(["run", "org.vinegarhq.Sober", "config"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn validate_fflag(flag: String) -> Result<bool, String> {
    let cache_dir = get_cache_dir();
    let files = vec!["PCDesktopClient.json", "AndroidApp.json"];
    let mut was_checked = false;

    for file_name in files {
        let tracker_path = cache_dir.join(file_name);

        // Fetch if it doesn't exist or is older than 24h
        let should_fetch = if !tracker_path.exists() {
            true
        } else {
            if let Ok(metadata) = fs::metadata(&tracker_path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = std::time::SystemTime::now().duration_since(modified) {
                        duration.as_secs() > 86400 // 24 hours
                    } else { true }
                } else { true }
            } else { true }
        };

        if should_fetch {
            let url = format!("https://raw.githubusercontent.com/MaximumADHD/Roblox-FFlag-Tracker/main/{}", file_name);
            if let Ok(response) = reqwest::get(&url).await {
                if let Ok(bytes) = response.bytes().await {
                    fs::write(&tracker_path, bytes).ok();
                }
            }
        }

        if tracker_path.exists() {
            if let Ok(content) = fs::read_to_string(&tracker_path) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(obj) = json.as_object() {
                        was_checked = true;
                        if obj.contains_key(&flag) {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }

    if was_checked {
        Ok(false)
    } else {
        Ok(true) // Default to true if we entirely failed to check
    }
}

#[tauri::command]
fn import_fflags_json(path: String) -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    let mut flags = std::collections::HashMap::new();
    
    // Some FFlag packs wrap them in an "fflags" object, others just put them at the root
    if let Some(fflags_obj) = json.get("fflags").and_then(|f| f.as_object()) {
        for (k, v) in fflags_obj {
            flags.insert(k.clone(), v.clone());
        }
    } else if let Some(root_obj) = json.as_object() {
        for (k, v) in root_obj {
            flags.insert(k.clone(), v.clone());
        }
    }
    
    Ok(flags)
}

// --- Patch Management Engine ---

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub id: String, // GameBanana ID or Fishstrap folder name
    pub title: String,
    pub author: String,
    pub source: String, // "gamebanana" or "fishstrap"
    pub image_url: Option<String>,
}


pub fn get_cache_dir() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("lucem");
    fs::create_dir_all(&path).ok();
    path
}

pub fn get_sober_overlay_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".var/app/org.vinegarhq.Sober/data/sober/asset_overlay");
    fs::create_dir_all(&path).ok();
    path
}

#[tauri::command]
fn open_mod_folder() -> Result<(), String> {
    let path = get_sober_overlay_dir();

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            launch_sober,
            launch_sober_config,
            validate_fflag,
            import_fflags_json,
            fetch_fishstrap_mods,
            fetch_gamebanana_mods,
            install_mod,
            uninstall_mod,
            open_mod_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
