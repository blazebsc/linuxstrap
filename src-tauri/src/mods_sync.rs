use crate::config::LinuxstrapConfig;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

pub fn get_sober_overlay_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".var/app/org.vinegarhq.Sober/data/sober/asset_overlay");
    fs::create_dir_all(&path).ok();
    path
}

pub fn sync_mods(app: &tauri::AppHandle, config: &LinuxstrapConfig) -> Result<(), String> {
    let overlay_dir = get_sober_overlay_dir();

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

    for f in &fonts_to_replace {
        let p = font_dir.join(f);
        if p.exists() {
            fs::remove_file(p).ok();
        }
    }

    if config.font_type == "custom" && !config.custom_font_path.is_empty() {
        fs::create_dir_all(&font_dir).ok();
        let source_path = PathBuf::from(&config.custom_font_path);

        if source_path.is_dir() {
            // Copy all font files from folder
            if let Ok(entries) = fs::read_dir(&source_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "ttf" || e == "otf" || e == "ttc").unwrap_or(false) {
                        if let Some(filename) = path.file_name() {
                            fs::copy(&path, font_dir.join(filename)).ok();
                        }
                    }
                }
            }
        } else if source_path.extension().map(|e| e == "zip").unwrap_or(false) {
            let font_dir_clone = font_dir.clone();
            crate::zip_extractor::extract_fonts_from_zip(&source_path, &font_dir_clone);
        } else {
            // Single font file - copy to all replacements
            for f in &fonts_to_replace {
                fs::copy(&source_path, font_dir.join(f)).ok();
            }
        }
    }

    Ok(())
}
