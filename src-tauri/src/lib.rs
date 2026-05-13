mod commands;
mod config;
mod image_recolor;
mod mods_api;
mod mods_sync;
mod sober_sync;
mod zip_extractor;

pub use commands::{
    get_config, get_system_fonts, generate_theme, import_fflags_json, launch_sober, launch_sober_config,
    open_mod_folder, save_config, validate_fflag, pick_file, pick_image, pick_folder, is_directory,
    recolor_fonts, kill_sober, check_sober_running, check_sse42, get_audio_driver, set_audio_driver,
    wake_nvidia_gpu, setup_xdg_portal, reset_sober_config,
};
pub use mods_api::{fetch_fishstrap_mods, fetch_gamebanana_mods, install_mod, uninstall_mod};

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
            open_mod_folder,
            get_system_fonts,
            pick_file,
            pick_image,
            pick_folder,
            is_directory,
            recolor_fonts,
            generate_theme,
            kill_sober,
            check_sober_running,
            check_sse42,
            get_audio_driver,
            set_audio_driver,
            wake_nvidia_gpu,
            setup_xdg_portal,
            reset_sober_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
