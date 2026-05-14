use crate::config::LinuxstrapConfig;
use crate::image_recolor::{recolor_image, walkdir};
use crate::mods_sync::{get_sober_overlay_dir, sync_mods};
use crate::sober_sync::{get_sober_config_path, sync_to_sober_config};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::{Emitter, Manager};
use tauri_plugin_dialog::DialogExt;

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("linuxstrap");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

pub fn get_cache_dir() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("linuxstrap");
    fs::create_dir_all(&path).ok();
    path
}

#[tauri::command]
pub fn get_config() -> LinuxstrapConfig {
    let path = get_config_path();
    eprintln!("[linuxstrap] Loading config from: {}", path.display());

    // First check: read from linuxstrap's own config
    let config: LinuxstrapConfig = if let Ok(content) = fs::read_to_string(&path) {
        eprintln!("[linuxstrap] Found existing config, parsing...");
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        // First run: import settings from Sober config
        eprintln!("[linuxstrap] No config found, importing from Sober...");
        let sober_path = get_sober_config_path();
        let mut config = LinuxstrapConfig::default();

        if sober_path.exists() {
            if let Ok(content) = fs::read_to_string(&sober_path) {
                let cleaned_content: String = content
                    .lines()
                    .filter(|line| !line.trim_start().starts_with("//"))
                    .collect::<Vec<&str>>()
                    .join("\n");

                if let Ok(sober_json) = serde_json::from_str::<Value>(&cleaned_content) {
                    if let Some(val) = sober_json.get("discord_rpc_enabled").and_then(|v| v.as_bool()) {
                        config.discord_rpc = val;
                    }
                    if let Some(val) = sober_json.get("discord_rpc_show_join_button").and_then(|v| v.as_bool()) {
                        config.discord_rpc_join_button = val;
                    }
                    if let Some(val) = sober_json.get("use_opengl").and_then(|v| v.as_bool()) {
                        config.renderer = if val { "opengl".to_string() } else { "vulkan".to_string() };
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
                    if let Some(val) = sober_json.get("server_location_indicator_enabled").and_then(|v| v.as_bool()) {
                        config.server_location_indicator = val;
                    }
                    if let Some(val) = sober_json.get("use_console_experience").and_then(|v| v.as_bool()) {
                        config.use_console_experience = val;
                    }
                    if let Some(val) = sober_json.get("allow_gamepad_permission").and_then(|v| v.as_bool()) {
                        config.allow_gamepad_permission = val;
                    }
                    if let Some(val) = sober_json.get("touch_mode").and_then(|v| v.as_str()) {
                        config.touch_mode = val.to_string();
                    }
                    if let Some(val) = sober_json.get("use_libsecret").and_then(|v| v.as_bool()) {
                        config.use_libsecret = val;
                    }
                    if let Some(val) = sober_json.get("graphics_optimization_mode").and_then(|v| v.as_str()) {
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
                                config.custom_fflags.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
            }
        }
        config
    };

    config
}

#[tauri::command]
pub fn save_config(app: tauri::AppHandle, config: LinuxstrapConfig) -> Result<(), String> {
    eprintln!("[linuxstrap] Saving config...");

    let path = get_config_path();
    let path_str = path.display().to_string();
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    eprintln!("[linuxstrap] Config saved to {}", path_str);

    // Sync to Sober config and mods
    eprintln!("[linuxstrap] Syncing to Sober config...");
    sync_to_sober_config(&config)?;
    eprintln!("[linuxstrap] Syncing mods...");
    sync_mods(&app, &config)?;
    eprintln!("[linuxstrap] Save complete.");
    Ok(())
}

#[tauri::command]
pub fn launch_sober() -> Result<(), String> {
    eprintln!("[linuxstrap] Launching Sober...");

    let config_path = get_config_path();
    let config_content = fs::read_to_string(&config_path).unwrap_or_default();
    let config: LinuxstrapConfig = serde_json::from_str(&config_content).unwrap_or_default();

    eprintln!("[linuxstrap] Config loaded - renderer: {}, gpu: {}, gamemode: {}", 
        config.renderer, config.selected_gpu, config.enable_gamemode);

    let mut use_dri_prime = false;

    if config.selected_gpu != "default" && !config.selected_gpu.is_empty() {
        let gpu_lower = config.selected_gpu.to_lowercase();
        if gpu_lower.contains("nvidia") || 
           gpu_lower.contains("discrete") ||
           gpu_lower.contains("gtx") ||
           gpu_lower.contains("rtx") {
            use_dri_prime = true;
            eprintln!("[linuxstrap] Setting DRI_PRIME=1 for GPU: {}", config.selected_gpu);
        }
    }

    if config.enable_gamemode {
        if use_dri_prime {
            Command::new("sh")
                .arg("-c")
                .arg("nohup DRI_PRIME=1 gamemoderun flatpak run org.vinegarhq.Sober > /dev/null 2>&1 &")
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("nohup gamemoderun flatpak run org.vinegarhq.Sober > /dev/null 2>&1 &")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        eprintln!("[linuxstrap] Launched with gamemode + DRI_PRIME={}", use_dri_prime);
    } else {
        if use_dri_prime {
            Command::new("sh")
                .arg("-c")
                .arg("nohup DRI_PRIME=1 flatpak run org.vinegarhq.Sober > /dev/null 2>&1 &")
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("nohup flatpak run org.vinegarhq.Sober > /dev/null 2>&1 &")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        eprintln!("[linuxstrap] Launched without gamemode, DRI_PRIME={}", use_dri_prime);
    }
    Ok(())
}

#[tauri::command]
pub fn launch_sober_config() -> Result<(), String> {
    Command::new("flatpak")
        .args(["run", "org.vinegarhq.Sober", "config"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn validate_fflag(flag: String) -> Result<bool, String> {
    let cache_dir = get_cache_dir();
    let files = vec!["PCDesktopClient.json", "AndroidApp.json"];
    let mut was_checked = false;

    for file_name in files {
        let tracker_path = cache_dir.join(file_name);

        let should_fetch = if !tracker_path.exists() {
            true
        } else {
            if let Ok(metadata) = fs::metadata(&tracker_path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = std::time::SystemTime::now().duration_since(modified) {
                        duration.as_secs() > 86400
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                true
            }
        };

        if should_fetch {
            let url = format!(
                "https://raw.githubusercontent.com/MaximumADHD/Roblox-FFlag-Tracker/main/{}",
                file_name
            );
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

    Ok(if was_checked { false } else { true })
}

#[tauri::command]
pub fn import_fflags_json(
    path: String,
) -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let mut flags = std::collections::HashMap::new();

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

#[tauri::command]
pub fn get_system_fonts() -> Result<Vec<serde_json::Value>, String> {
    let output = Command::new("fc-list")
        .args(["--format", "%{file}|%{family}\n"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("fc-list failed".into());
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let mut fonts: Vec<serde_json::Value> = stdout_str
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            }
            let parts: Vec<&str> = l.splitn(2, '|').collect();
            if parts.len() != 2 {
                return None;
            }
            Some(serde_json::json!({
                "file": parts[0].to_string(),
                "family": parts[1].to_string()
            }))
        })
        .collect();

    fonts.sort_by(|a, b| {
        let fa = a.get("family").and_then(|v| v.as_str()).unwrap_or("");
        let fb = b.get("family").and_then(|v| v.as_str()).unwrap_or("");
        fa.cmp(fb)
    });
    fonts.dedup_by(|a, b| {
        a.get("family").and_then(|v| v.as_str()) == b.get("family").and_then(|v| v.as_str())
    });

    Ok(fonts)
}

#[tauri::command]
pub async fn pick_file(
    app: tauri::AppHandle,
    title: String,
    _filters: String,
) -> Result<Option<String>, String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_title(&title)
        .add_filter("Fonts", &["ttf", "otf", "zip"])
        .pick_file(move |file| {
            if let Some(f) = file {
                tx.send(f.to_string()).ok();
            } else {
                tx.send(String::new()).ok();
            }
        });

    let result = rx.recv().map_err(|e| e.to_string())?;
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[tauri::command]
pub async fn pick_image(
    app: tauri::AppHandle,
    title: String,
) -> Result<Option<String>, String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_title(&title)
        .add_filter("Image", &["png"])
        .pick_file(move |file| {
            if let Some(f) = file {
                tx.send(f.to_string()).ok();
            } else {
                tx.send(String::new()).ok();
            }
        });

    let result = rx.recv().map_err(|e| e.to_string())?;
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[tauri::command]
pub async fn pick_folder(
    app: tauri::AppHandle,
    title: String,
) -> Result<Option<String>, String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_title(&title)
        .pick_folder(move |file| {
            if let Some(f) = file {
                tx.send(f.to_string()).ok();
            } else {
                tx.send(String::new()).ok();
            }
        });

    let result = rx.recv().map_err(|e| e.to_string())?;
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[tauri::command]
pub fn is_directory(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}

#[tauri::command]
pub async fn generate_theme(app: tauri::AppHandle, color_hex: String) -> Result<usize, String> {
    use std::sync::atomic::{AtomicUsize, Ordering};

    eprintln!("[linuxstrap] Generating theme with color: {}", color_hex);

    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    let base_dir = resource_dir.join("assets/theme_base");

    if !base_dir.exists() {
        return Err("Theme base files not found. Please reinstall linuxstrap.".to_string());
    }

    let overlay_dir = get_sober_overlay_dir();
    eprintln!("[linuxstrap] Overlay dir: {}", overlay_dir.display());

    let color = color_hex.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).map_err(|_| "Invalid color")?;
    let g = u8::from_str_radix(&color[2..4], 16).map_err(|_| "Invalid color")?;
    let b = u8::from_str_radix(&color[4..6], 16).map_err(|_| "Invalid color")?;
    eprintln!("[linuxstrap] RGB: {}, {}, {}", r, g, b);

    app.emit("theme_progress", serde_json::json!({ "status": "scanning", "progress": 0, "message": "Scanning files..." }))
        .ok();

    let mut total_files = 0;
    for entry in walkdir(&base_dir).unwrap_or_default() {
        if let Some(ext) = entry.path().extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" {
                total_files += 1;
            }
        }
    }
    eprintln!("[linuxstrap] Found {} image files to process", total_files);

    let processed = std::sync::Arc::new(AtomicUsize::new(0));
    let processed_clone = processed.clone();
    let app_clone = app.clone();

    let content_dest = overlay_dir.join("content");
    let extra_dest = overlay_dir.join("ExtraContent");

    let _ = tokio::task::spawn_blocking(move || {
        eprintln!("[linuxstrap] Processing content/ directory...");
        process_directory_with_progress(&base_dir.join("content"), &content_dest, r, g, b, total_files, &processed_clone, &app_clone)?;
        eprintln!("[linuxstrap] Processing ExtraContent/ directory...");
        process_directory_with_progress(&base_dir.join("ExtraContent"), &extra_dest, r, g, b, total_files, &processed_clone, &app_clone)?;
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    app.emit("theme_progress", serde_json::json!({ "status": "complete", "progress": 100, "message": "Done!" }))
        .ok();

    let info_path = overlay_dir.join("info.json");
    let info_json = serde_json::json!({
        "FroststrapVersion": env!("CARGO_PKG_VERSION"),
        "CreatedUsing": "linuxstrap",
        "RobloxVersion": null,
        "RobloxVersionHash": null,
        "OptionsUsed": {
            "ColorCursors": true,
            "ColorShiftlock": true,
            "ColorVoicechat": true,
            "ColorEmoteWheel": true,
            "GradientAngle": 0
        },
        "ColorsUsed": {
            "SolidColor": format!("#{}", color.to_uppercase())
        }
    });

    fs::write(&info_path, serde_json::to_string_pretty(&info_json).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    Ok(processed.load(Ordering::SeqCst))
}

fn process_directory_with_progress(
    source_dir: &std::path::Path,
    dest_dir: &std::path::Path,
    r: u8,
    g: u8,
    b: u8,
    total_files: usize,
    processed: &std::sync::Arc<std::sync::atomic::AtomicUsize>,
    app: &tauri::AppHandle,
) -> Result<(), String> {
    std::fs::create_dir_all(dest_dir).map_err(|e| e.to_string())?;

    let entries = walkdir(source_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(ext) = entry_path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" {
                    let relative = entry_path.strip_prefix(source_dir).unwrap();
                    let dest_path = dest_dir.join(relative);

                    if let Err(e) = recolor_image(&entry_path, &dest_path, r, g, b) {
                        eprintln!("Failed to process {}: {}", entry_path.display(), e);
                    }

                    let current = processed.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    let progress = if total_files > 0 { (current * 100) / (total_files * 2) } else { 100 };

                    app.emit("theme_progress", serde_json::json!({
                        "status": "processing",
                        "progress": progress.min(99),
                        "current": current,
                        "total": total_files * 2,
                        "message": format!("Processing {}...", entry_path.file_name().unwrap_or_default().to_string_lossy())
                    })).ok();
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn kill_sober() -> Result<(), String> {
    eprintln!("[linuxstrap] Killing Sober processes...");
    Command::new("flatpak")
        .args(["kill", "org.vinegarhq.Sober"])
        .output()
        .map_err(|e| e.to_string())?;
    eprintln!("[linuxstrap] Kill command sent.");
    Ok(())
}

#[tauri::command]
pub fn check_sober_running() -> Result<bool, String> {
    let output = Command::new("flatpak")
        .args(["ps"])
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("org.vinegarhq.Sober"))
}

#[tauri::command]
pub fn check_sse42() -> bool {
    let output = Command::new("grep")
        .args(["-q", "sse4_2", "/proc/cpuinfo"])
        .output();
    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

#[tauri::command]
pub fn get_audio_driver() -> Result<String, String> {
    let output = Command::new("flatpak")
        .args(["info", "--show-env", "org.vinegarhq.Sober"])
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("SDL_AUDIO_DRIVER=") {
            let val = line.trim_start_matches("SDL_AUDIO_DRIVER=");
            if val.is_empty() {
                return Ok("default".to_string());
            }
            return Ok(val.to_string());
        }
    }
    Ok("default".to_string())
}

#[tauri::command]
pub fn set_audio_driver(driver: String) -> Result<(), String> {
    if driver == "default" {
        Command::new("flatpak")
            .args(["override", "--user", "--unset-env=SDL_AUDIO_DRIVER"])
            .output()
            .map_err(|e| e.to_string())?;
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("flatpak override --user --env=SDL_AUDIO_DRIVER={} org.vinegarhq.Sober", driver))
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn wake_nvidia_gpu() -> Result<(), String> {
    Command::new("sh")
        .arg("-c")
        .arg("vulkaninfo --summary 2>/dev/null || vkcube 2>/dev/null || echo 'done'")
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn setup_xdg_portal() -> Result<(), String> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
    Command::new("sh")
        .arg("-c")
        .arg(&format!(
            r#"mkdir -p ~/.config/xdg-desktop-portal
echo -e '[preferred]
default=gtk' > ~/.config/xdg-desktop-portal/{}-portals.conf
systemctl --user restart xdg-desktop-portal.service xdg-desktop-portal-gtk.service 2>/dev/null || true
xdg-mime default org.vinegarhq.Sober.desktop x-scheme-handler/roblox-player x-scheme-handler/roblox
"#,
            desktop
        ))
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn open_mod_folder() -> Result<(), String> {
    let path = get_sober_overlay_dir();
    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reset_sober_config() -> Result<(), String> {
    let path = get_sober_config_path();
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut comments = Vec::new();
    let mut json_lines = Vec::new();

    for line in content.lines() {
        if line.trim_start().starts_with("//") {
            comments.push(line.to_string());
        } else {
            json_lines.push(line.to_string());
        }
    }

    let cleaned_content = json_lines.join("\n");
    let mut sober_json: Value = serde_json::from_str(&cleaned_content).unwrap_or(serde_json::json!({}));

    if let Some(obj) = sober_json.as_object_mut() {
        obj.remove("fflags");
        obj.insert("fflags".to_string(), serde_json::json!({}));
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

#[tauri::command]
pub fn recolor_fonts(source_path: String, _color_hex: String) -> Result<(), String> {
    let path = std::path::Path::new(&source_path);

    if !path.exists() {
        return Err(format!("Path does not exist: {}", source_path));
    }

    let font_dir = get_sober_overlay_dir()
        .join("ExtraContent")
        .join("LuaPackages")
        .join("Packages")
        .join("_Index")
        .join("BuilderIcons")
        .join("BuilderIcons")
        .join("Font");

    std::fs::create_dir_all(&font_dir).map_err(|e| e.to_string())?;

    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "ttf" || ext_lower == "otf" {
                    install_single_font(&file_path, &font_dir)?;
                }
            }
        }
    } else if path.is_file() {
        install_single_font(path, &font_dir)?;
    }

    write_buildericons_json(&font_dir)?;

    Ok(())
}

fn install_single_font(
    source: &std::path::Path,
    dest_dir: &std::path::Path,
) -> Result<(), String> {
    let font_data = std::fs::read(source).map_err(|e| format!("Failed to read font: {}", e))?;

    let face = ttf_parser::Face::parse(&font_data, 0)
        .map_err(|e| format!("Failed to parse font: {}", e))?;

    let family_name = face
        .names()
        .into_iter()
        .find(|n| n.name_id == ttf_parser::name_id::FAMILY)
        .and_then(|n| n.to_string())
        .unwrap_or_else(|| "Custom".to_string());

    let dest_file = dest_dir.join(format!("{}.otf", family_name.replace(' ', "")));
    std::fs::copy(source, &dest_file).map_err(|e| format!("Failed to copy font: {}", e))?;

    Ok(())
}

fn write_buildericons_json(font_dir: &std::path::Path) -> Result<(), String> {
    let root = font_dir.parent().ok_or("Invalid font dir")?;

    let entries: Vec<_> = std::fs::read_dir(font_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.extension().map(|ext| ext == "otf" || ext == "ttf").unwrap_or(false) {
                let name = path.file_stem()?.to_str()?.to_string();
                Some(format!(
                    r#"{{
                "name": "{}",
                "weight": 400,
                "style": "normal",
                "assetId": "rbxasset://LuaPackages/Packages/_Index/BuilderIcons/BuilderIcons/Font/{}"
            }}"#,
                    name, path.file_name()?.to_string_lossy()
                ))
            } else {
                None
            }
        })
        .collect();

    let faces = if entries.is_empty() {
        r#"[
            {
                "name": "Regular",
                "weight": 400,
                "style": "normal",
                "assetId": "rbxasset://LuaPackages/Packages/_Index/BuilderIcons/BuilderIcons/Font/BuilderIcons-Regular.otf"
            }
        ]"#
    } else {
        &entries.join(",")
    };

    let json = format!(
        r#"{{
    "name": "Builder Icons",
    "loadStrategy": "sameFamilyOnly",
    "faces": [{}]
}}"#,
        faces
    );

    std::fs::write(root.join("BuilderIcons.json"), json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_gpus() -> Result<Vec<serde_json::Value>, String> {
    let mut gpus: Vec<serde_json::Value> = Vec::new();

    // Try lspci first - more reliable
    let lspci_output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|display' | sed 's/.*: //'")
        .output();

    if let Ok(out) = lspci_output {
        let vga_output = String::from_utf8_lossy(&out.stdout);
        for line in vga_output.lines() {
            let name = line.trim();
            if !name.is_empty() {
                gpus.push(serde_json::json!({
                    "id": name.to_string(),
                    "name": name.to_string()
                }));
            }
        }
    }

    // Try vulkaninfo as alternative
    if gpus.is_empty() {
        let vulkan_output = Command::new("sh")
            .arg("-c")
            .arg("vulkaninfo 2>/dev/null | grep -A5 'GPU[0-9]' | grep 'deviceName\\|deviceName' | head -5")
            .output();

        if let Ok(out) = vulkan_output {
            let vk_output = String::from_utf8_lossy(&out.stdout);
            for line in vk_output.lines() {
                if let Some(name) = line.split(':').nth(1) {
                    let name = name.trim().trim_start_matches('"').trim_end_matches('"');
                    if !name.is_empty() {
                        gpus.push(serde_json::json!({
                            "id": name.to_string(),
                            "name": name.to_string()
                        }));
                    }
                }
            }
        }
    }

    // Always add default option
    if gpus.is_empty() {
        gpus.push(serde_json::json!({
            "id": "default",
            "name": "Default (System)"
        }));
    } else {
        gpus.insert(0, serde_json::json!({
            "id": "default",
            "name": "Default (System)"
        }));
    }

    Ok(gpus)
}
