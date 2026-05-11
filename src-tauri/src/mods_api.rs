use crate::{get_cache_dir, get_sober_overlay_dir, get_config, save_config, ModInfo, zip_extractor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

// Fetch Fishstrap Mods
#[tauri::command]
pub async fn fetch_fishstrap_mods() -> Result<Vec<ModInfo>, String> {
    let client = reqwest::Client::builder().user_agent("linuxstrap").build().unwrap();
    let res = client
        .get("https://api.github.com/repos/fishstrap/mods/contents/RobloxMods")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let items: Vec<Value> = res.json().await.map_err(|e| e.to_string())?;
    let mut mods = Vec::new();

    for item in items {
        if let Some(name) = item["name"].as_str() {
            if name != "[ Content Deleted ]" && item["type"].as_str() == Some("dir") {
                // Fetch manifest
                let manifest_url = format!("https://raw.githubusercontent.com/fishstrap/mods/main/RobloxMods/{}/manifest.json", urlencoding::encode(name));
                if let Ok(manifest_res) = client.get(&manifest_url).send().await {
                    if let Ok(manifest) = manifest_res.json::<Value>().await {
                        mods.push(ModInfo {
                            id: name.to_string(),
                            title: manifest["title"].as_str().unwrap_or(name).to_string(),
                            author: manifest["author"].as_str().unwrap_or("Unknown").to_string(),
                            source: "fishstrap".to_string(),
                            image_url: Some(format!("https://raw.githubusercontent.com/fishstrap/mods/main/RobloxMods/{}/preview.webp", urlencoding::encode(name))),
                        });
                    }
                }
            }
        }
    }
    Ok(mods)
}

// Fetch GameBanana Mods
#[tauri::command]
pub async fn fetch_gamebanana_mods(page: u32) -> Result<Vec<ModInfo>, String> {
    let url = format!("https://gamebanana.com/apiv11/Game/2879/Subfeed?_nPage={}&_sSort=new&_csvModelInclusions=Mod", page);
    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let data: Value = res.json().await.map_err(|e| e.to_string())?;

    let mut mods = Vec::new();
    if let Some(records) = data["_aRecords"].as_array() {
        for record in records {
            let id = record["_idRow"].as_i64().unwrap_or(0).to_string();
            let title = record["_sName"].as_str().unwrap_or("Unknown").to_string();
            let author = "GameBanana User".to_string(); // we could fetch from submitter if needed
            
            let mut image_url = None;
            if let Some(images) = record["_aPreviewMedia"]["_aImages"].as_array() {
                if let Some(img) = images.first() {
                    if let (Some(base), Some(file)) = (img["_sBaseUrl"].as_str(), img["_sFile220"].as_str()) {
                        image_url = Some(format!("{}/{}", base, file));
                    }
                }
            }

            mods.push(ModInfo {
                id,
                title,
                author,
                source: "gamebanana".to_string(),
                image_url,
            });
        }
    }
    Ok(mods)
}

#[tauri::command]
pub async fn install_mod(app: tauri::AppHandle, id: String, source: String) -> Result<(), String> {
    let client = reqwest::Client::builder().user_agent("linuxstrap").build().unwrap();
    let download_url = if source == "fishstrap" {
        format!("https://raw.githubusercontent.com/fishstrap/mods/main/RobloxMods/{}/{}.zip", urlencoding::encode(&id), urlencoding::encode(&id))
    } else {
        // Gamebanana
        let profile_url = format!("https://gamebanana.com/apiv11/Mod/{}/ProfilePage", id);
        let res = client.get(&profile_url).send().await.map_err(|e| e.to_string())?;
        let profile: Value = res.json().await.map_err(|e| e.to_string())?;
        
        let mut d_url = String::new();
        if let Some(files) = profile["_aFiles"].as_array() {
            if let Some(file) = files.first() {
                if let Some(url) = file["_sDownloadUrl"].as_str() {
                    d_url = url.to_string();
                }
            }
        }
        if d_url.is_empty() {
            return Err("No download link found".to_string());
        }
        d_url
    };

    let cache_dir = get_cache_dir();
    let safe_id = id.replace("/", "_");
    let zip_path = cache_dir.join(format!("{}_{}.zip", source, safe_id));
    
    // Download zip
    let bytes = client.get(&download_url).send().await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
    fs::write(&zip_path, bytes).map_err(|e| e.to_string())?;

    // Extract
    let overlay_dir = get_sober_overlay_dir();
    zip_extractor::extract_mod_zip(&zip_path, &overlay_dir)?;

    // Save to config
    let mut config = get_config();
    let patch_key = format!("{}:{}", source, id);
    if !config.patches.contains(&patch_key) {
        config.patches.push(patch_key);
        save_config(app, config)?;
    }

    Ok(())
}

#[tauri::command]
pub async fn uninstall_mod(app: tauri::AppHandle, id: String, source: String) -> Result<(), String> {
    // Uninstalling is tricky because we just extracted a zip. We don't track which files were extracted by which mod.
    // The easiest way for now is to just remove it from config. 
    // To properly uninstall, we'd need to re-download the zip, list its files, and remove them from overlay.
    let cache_dir = get_cache_dir();
    let zip_path = cache_dir.join(format!("{}_{}.zip", source, id));
    
    if zip_path.exists() {
        let overlay_dir = get_sober_overlay_dir();
        if let Ok(file) = fs::File::open(&zip_path) {
            if let Ok(mut archive) = zip::ZipArchive::new(file) {
                for i in 0..archive.len() {
                    if let Ok(file) = archive.by_index(i) {
                        if let Some(outpath) = file.enclosed_name() {
                            let mut target_path = None;
                            let components: Vec<_> = outpath.components().collect();
                            for (idx, comp) in components.iter().enumerate() {
                                let comp_str = comp.as_os_str().to_string_lossy().to_lowercase();
                                if comp_str == "extracontent" || comp_str == "platformcontent" || comp_str == "content" {
                                    let mut relative_path = PathBuf::new();
                                    for c in &components[idx..] {
                                        relative_path.push(c);
                                    }
                                    target_path = Some(overlay_dir.join(relative_path));
                                    break;
                                }
                            }
                            if let Some(target) = target_path {
                                if target.is_file() {
                                    fs::remove_file(target).ok();
                                }
                            }
                        }
                    }
                }
            }
        }
        // fs::remove_file(&zip_path).ok(); // keep cache for re-install?
    }

    let mut config = get_config();
    let patch_key = format!("{}:{}", source, id);
    config.patches.retain(|p| p != &patch_key);
    save_config(app, config)?;

    Ok(())
}
