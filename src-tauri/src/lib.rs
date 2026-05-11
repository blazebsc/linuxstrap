use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LucemConfig {
    pub discord_rpc: bool,
    pub discord_rpc_join_button: bool,
    pub max_fps: Option<u16>,
    pub patches: Vec<String>,
    pub renderer: String,
    pub close_on_leave: bool,
    pub enable_gamemode: bool,
    pub enable_hidpi: bool,
    pub server_location_indicator: bool,
    pub use_console_experience: bool,

    // FastFlags presets
    pub lighting_technology: String, // "default", "voxel", "shadowmap", "future"
    pub texture_quality: String,     // "default", "0", "1", "2", "3", "4"
    pub msaa: String,                // "default", "off", "1", "2", "4", "8"
    pub disable_bubble_chat: bool,
    pub disable_player_shadows: bool,
    
    // Mods
    pub use_old_avatar_background: bool,
    pub use_old_character_sounds: bool,
    pub cursor_type: String, // "default", "2006", "2013"
    
    // Custom FFlags
    pub custom_fflags: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for LucemConfig {
    fn default() -> Self {
        Self {
            discord_rpc: true,
            discord_rpc_join_button: true,
            max_fps: Some(60),
            patches: vec![],
            renderer: "vulkan".into(),
            close_on_leave: false,
            enable_gamemode: true,
            enable_hidpi: false,
            server_location_indicator: true,
            use_console_experience: false,
            lighting_technology: "default".into(),
            texture_quality: "default".into(),
            msaa: "default".into(),
            disable_bubble_chat: false,
            disable_player_shadows: false,
            use_old_avatar_background: false,
            use_old_character_sounds: false,
            cursor_type: "default".into(),
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
        obj.insert("discord_rpc_enabled".to_string(), serde_json::json!(config.discord_rpc));
        obj.insert("discord_rpc_show_join_button".to_string(), serde_json::json!(config.discord_rpc_join_button));
        obj.insert("use_opengl".to_string(), serde_json::json!(config.renderer == "opengl"));
        obj.insert("close_on_leave".to_string(), serde_json::json!(config.close_on_leave));
        obj.insert("enable_gamemode".to_string(), serde_json::json!(config.enable_gamemode));
        obj.insert("enable_hidpi".to_string(), serde_json::json!(config.enable_hidpi));
        obj.insert("server_location_indicator_enabled".to_string(), serde_json::json!(config.server_location_indicator));
        obj.insert("use_console_experience".to_string(), serde_json::json!(config.use_console_experience));

        // Setup FFlags
        let fflags = obj.entry("fflags".to_string()).or_insert_with(|| serde_json::json!({}));
        if let Some(fflags_obj) = fflags.as_object_mut() {
            // Max FPS
            if let Some(fps) = config.max_fps {
                fflags_obj.insert("DFIntTaskSchedulerTargetFps".to_string(), serde_json::json!(format!("{}", fps)));
            }

            // Lighting Technology
            fflags_obj.remove("DFFlagDebugRenderForceTechnologyVoxel");
            fflags_obj.remove("FFlagDebugForceFutureIsBrightPhase2");
            fflags_obj.remove("FFlagDebugForceFutureIsBrightPhase3");
            match config.lighting_technology.as_str() {
                "voxel" => { fflags_obj.insert("DFFlagDebugRenderForceTechnologyVoxel".to_string(), serde_json::json!(true)); },
                "shadowmap" => { fflags_obj.insert("FFlagDebugForceFutureIsBrightPhase2".to_string(), serde_json::json!(true)); },
                "future" => { fflags_obj.insert("FFlagDebugForceFutureIsBrightPhase3".to_string(), serde_json::json!(true)); },
                _ => {} // Default
            }

            // Texture Quality
            fflags_obj.remove("DFFlagTextureQualityOverrideEnabled");
            fflags_obj.remove("DFIntTextureQualityOverride");
            if config.texture_quality != "default" {
                if let Ok(quality_level) = config.texture_quality.parse::<u8>() {
                    fflags_obj.insert("DFFlagTextureQualityOverrideEnabled".to_string(), serde_json::json!(true));
                    fflags_obj.insert("DFIntTextureQualityOverride".to_string(), serde_json::json!(quality_level));
                }
            }

            // MSAA
            fflags_obj.remove("FFlagDebugDisableMSAA");
            fflags_obj.remove("FIntMSAASampleCount");
            match config.msaa.as_str() {
                "off" => { fflags_obj.insert("FFlagDebugDisableMSAA".to_string(), serde_json::json!(true)); },
                "1" | "2" | "4" | "8" => { 
                    fflags_obj.insert("FIntMSAASampleCount".to_string(), serde_json::json!(config.msaa)); 
                },
                _ => {} // default
            }

            // Bubble Chat
            fflags_obj.remove("FFlagEnableBubbleChatFromChatService");
            if config.disable_bubble_chat {
                fflags_obj.insert("FFlagEnableBubbleChatFromChatService".to_string(), serde_json::json!(false));
            }

            // Player Shadows
            fflags_obj.remove("FIntRenderShadowIntensity");
            if config.disable_player_shadows {
                fflags_obj.insert("FIntRenderShadowIntensity".to_string(), serde_json::json!("0"));
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
            let cleaned_content: String = content.lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<&str>>()
                .join("\n");
            
            if let Ok(sober_json) = serde_json::from_str::<Value>(&cleaned_content) {
                if let Some(fflags_obj) = sober_json.get("fflags").and_then(|f| f.as_object()) {
                    let managed_keys = vec![
                        "DFIntTaskSchedulerTargetFps",
                        "DFFlagDebugRenderForceTechnologyVoxel",
                        "FFlagDebugForceFutureIsBrightPhase2",
                        "FFlagDebugForceFutureIsBrightPhase3",
                        "DFFlagTextureQualityOverrideEnabled",
                        "DFIntTextureQualityOverride",
                        "FFlagDebugDisableMSAA",
                        "FIntMSAASampleCount",
                        "FFlagEnableBubbleChatFromChatService",
                        "FIntRenderShadowIntensity"
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
        if let Some(p) = bg_path.parent() { fs::create_dir_all(p).ok(); }
        fs::copy(assets_dir.join("OldAvatarBackground.rbxl"), bg_path).ok();
    } else {
        if bg_path.exists() { fs::remove_file(bg_path).ok(); }
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
            if p.exists() { fs::remove_file(p).ok(); }
        }
    }

    // Cursors
    let cursor_dir = overlay_dir.join("content/textures/Cursors/KeyboardMouse");
    let cursors = vec!["ArrowCursor.png", "ArrowFarCursor.png"];
    
    // First remove existing
    for c in &cursors {
        let p = cursor_dir.join(c);
        if p.exists() { fs::remove_file(p).ok(); }
    }

    if config.cursor_type == "2006" {
        fs::create_dir_all(&cursor_dir).ok();
        for c in &cursors {
            fs::copy(assets_dir.join(format!("Cursor/From2006/{}", c)), cursor_dir.join(c)).ok();
        }
    } else if config.cursor_type == "2013" {
        fs::create_dir_all(&cursor_dir).ok();
        for c in &cursors {
            fs::copy(assets_dir.join(format!("Cursor/From2013/{}", c)), cursor_dir.join(c)).ok();
        }
    }

    Ok(())
}

#[tauri::command]
fn launch_sober() -> Result<(), String> {
    Command::new("flatpak")
        .args(["run", "org.vinegarhq.Sober"])
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

// --- Patch Management Engine ---

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchIndexEntry {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchMetadata {
    pub name: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchFile {
    pub metadata: PatchMetadata,
    pub inputs: std::collections::HashMap<String, String>,
    pub outputs: std::collections::HashMap<String, String>,
}

fn get_cache_dir() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("lucem");
    fs::create_dir_all(&path).ok();
    path
}

fn get_sober_overlay_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".var/app/org.vinegarhq.Sober/data/sober/asset_overlay");
    fs::create_dir_all(&path).ok();
    path
}

#[tauri::command]
async fn fetch_patch_index() -> Result<Vec<PatchIndexEntry>, String> {
    let url = "https://raw.githubusercontent.com/equinoxhq/patch-store/refs/heads/master/index.json";
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let entries: Vec<PatchIndexEntry> = response.json().await.map_err(|e| e.to_string())?;
    Ok(entries)
}

#[tauri::command]
async fn install_patch(app: tauri::AppHandle, url: String) -> Result<(), String> {
    // 1. Fetch Patch JSON
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let patch: PatchFile = response.json().await.map_err(|e| e.to_string())?;

    let cache_dir = get_cache_dir();
    let overlay_dir = get_sober_overlay_dir();

    // 2. Process Inputs (Download)
    for (input_url, _input_name) in &patch.inputs {
        let safe_url = URL_SAFE.encode(input_url);
        let cache_file_path = cache_dir.join(&safe_url);

        if !cache_file_path.exists() {
            let bytes = reqwest::get(input_url).await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
            fs::write(&cache_file_path, bytes).map_err(|e| e.to_string())?;
        }
    }

    // 3. Process Outputs (Copy to Overlay)
    for (output_path, input_name) in &patch.outputs {
        // Find which URL corresponds to this input_name
        let mut input_url_opt = None;
        for (url_key, name) in &patch.inputs {
            if name == input_name {
                input_url_opt = Some(url_key);
                break;
            }
        }

        if let Some(input_url) = input_url_opt {
            let safe_url = URL_SAFE.encode(input_url);
            let cache_file_path = cache_dir.join(&safe_url);
            
            let final_output_path = overlay_dir.join(output_path);
            if let Some(parent) = final_output_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            fs::copy(cache_file_path, final_output_path).map_err(|e| e.to_string())?;
        }
    }

    // 4. Register to Lucem config
    let mut config = get_config();
    if !config.patches.contains(&url) {
        config.patches.push(url);
        save_config(app, config)?;
    }

    Ok(())
}

#[tauri::command]
async fn uninstall_patch(app: tauri::AppHandle, url: String) -> Result<(), String> {
    // We need to fetch it to know what files to remove
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let patch: PatchFile = response.json().await.map_err(|e| e.to_string())?;

    let overlay_dir = get_sober_overlay_dir();

    // Remove Outputs
    for (output_path, _) in &patch.outputs {
        let final_output_path = overlay_dir.join(output_path);
        if final_output_path.exists() {
            fs::remove_file(final_output_path).ok();
        }
    }

    // Unregister from config
    let mut config = get_config();
    config.patches.retain(|p| p != &url);
    save_config(app, config)?;

    Ok(())
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            launch_sober,
            launch_sober_config,
            fetch_patch_index,
            install_patch,
            uninstall_patch,
            open_mod_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
