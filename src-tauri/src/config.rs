use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinuxstrapConfig {
    pub discord_rpc: bool,
    pub discord_rpc_join_button: bool,
    pub patches: Vec<String>,
    pub renderer: String,
    pub selected_gpu: String,
    pub close_on_leave: bool,
    pub enable_gamemode: bool,
    pub enable_hidpi: bool,
    pub server_location_indicator: bool,
    pub use_console_experience: bool,

    pub allow_gamepad_permission: bool,
    pub touch_mode: String,
    pub use_libsecret: bool,
    pub graphics_optimization_mode: String,

    pub lighting_technology: String,
    pub texture_quality: String,
    pub msaa: String,
    pub disable_bubble_chat: bool,
    pub disable_player_shadows: bool,

    pub enable_super_performance: bool,
    pub enable_network_optimization: bool,
    pub enable_wayland_clipboard: bool,
    pub bring_back_oof: bool,

    pub use_old_avatar_background: bool,
    pub use_old_character_sounds: bool,
    pub cursor_type: String,
    pub custom_cursor_path: String,
    pub font_type: String,
    pub custom_font_path: String,

    pub theme_color: String,

    pub theme_preset: String,

    pub custom_fflags: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for LinuxstrapConfig {
    fn default() -> Self {
        Self {
            discord_rpc: true,
            discord_rpc_join_button: true,
            patches: vec![],
            renderer: "vulkan".into(),
            selected_gpu: "default".into(),
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
            enable_super_performance: false,
            enable_network_optimization: false,
            enable_wayland_clipboard: false,
            bring_back_oof: true,
            use_old_avatar_background: false,
            use_old_character_sounds: false,
            cursor_type: "default".into(),
            custom_cursor_path: "".into(),
            font_type: "default".into(),
            custom_font_path: "".into(),
            theme_color: "#e74c3c".into(),
            theme_preset: "red".into(),
            custom_fflags: std::collections::HashMap::new(),
        }
    }
}
