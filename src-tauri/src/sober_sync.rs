use crate::config::LinuxstrapConfig;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub fn get_sober_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".var/app/org.vinegarhq.Sober/config/sober/config.json");
    path
}

pub fn sync_to_sober_config(config: &LinuxstrapConfig) -> Result<(), String> {
    let path = get_sober_config_path();

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
                _ => {}
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
                _ => {}
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

            // Tuxstrap Default FFlags
            if config.bring_back_oof {
                fflags_obj.insert(
                    "FFlagDisableFeedbackSoothsayerCheck".to_string(),
                    serde_json::json!(true),
                );
            }
            fflags_obj.insert(
                "FFlagLuaAppUseUIBloxColorPalettes1".to_string(),
                serde_json::json!(true),
            );
            fflags_obj.insert(
                "FFlagUIBloxUseNewThemeColorPalettes".to_string(),
                serde_json::json!(true),
            );
            fflags_obj.insert(
                "DFIntS2PhysicsSendRate".to_string(),
                serde_json::json!(38000),
            );
            fflags_obj.insert(
                "DFIntTaskSchedulerTargetFps".to_string(),
                serde_json::json!(999999),
            );
            fflags_obj.insert(
                "FIntTargetRefreshRate".to_string(),
                serde_json::json!(999999),
            );
            fflags_obj.insert(
                "FStringAdGuiHorizontalRobloxFallbackImageAssetId".to_string(),
                serde_json::json!("86999279798758"),
            );
            fflags_obj.insert(
                "FStringAdGuiHorizontalStudioPlaceHolderImageAssetId".to_string(),
                serde_json::json!("86999279798758"),
            );
            fflags_obj.insert(
                "FStringAdGuiLivePreviewWatermarkV2".to_string(),
                serde_json::json!("86999279798758"),
            );

            // Tuxstrap Super Performance FFlags
            if config.enable_super_performance {
                fflags_obj.insert(
                    "DFFlagDebugRenderForceTechnologyVoxel".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert("FIntFRMMinGrassDistance".to_string(), serde_json::json!(0));
                fflags_obj.insert("FIntFRMMaxGrassDistance".to_string(), serde_json::json!(0));
                fflags_obj.insert(
                    "FIntRenderGrassDetailStrands".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "FIntRenderGrassHeightScaler".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFFlagDebugPauseVoxelizer".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert("DFFlagDisableDPIScale".to_string(), serde_json::json!(true));
                fflags_obj.insert(
                    "FStringPartTexturePackTablePre2022".to_string(),
                    serde_json::json!(""),
                );
                fflags_obj.insert(
                    "FStringPartTexturePackTable2022".to_string(),
                    serde_json::json!(""),
                );
                fflags_obj.insert(
                    "FStringTerrainMaterialTablePre2022".to_string(),
                    serde_json::json!(""),
                );
                fflags_obj.insert(
                    "FStringTerrainMaterialTable2022".to_string(),
                    serde_json::json!(""),
                );
                fflags_obj.insert(
                    "FIntRenderShadowIntensity".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert("FFlagDisablePostFx".to_string(), serde_json::json!(true));
                fflags_obj.insert(
                    "FIntDebugForceMSAASamples".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFFlagTextureQualityOverrideEnabled".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert(
                    "DFIntTextureQualityOverride".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntCSGLevelOfDetailSwitchingDistance".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntCSGLevelOfDetailSwitchingDistanceL12".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntCSGLevelOfDetailSwitchingDistanceL23".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntCSGLevelOfDetailSwitchingDistanceL34".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "FIntRenderLocalLightUpdatesMax".to_string(),
                    serde_json::json!(1),
                );
                fflags_obj.insert(
                    "FIntRenderLocalLightUpdatesMin".to_string(),
                    serde_json::json!(1),
                );
                fflags_obj.insert("FFlagDebugSkyGray".to_string(), serde_json::json!(true));
                fflags_obj.insert(
                    "FFlagCoreGuiTypeSelfViewPresent".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "DFIntDebugFRMQualityLevelOverride".to_string(),
                    serde_json::json!(1),
                );
                fflags_obj.insert(
                    "FFlagRenderCheckThreading".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert(
                    "DFIntTextureCompositorActiveJobs".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "FFlagTaskSchedulerLimitTargetFpsTo2402".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "FFlagNewLightAttenuation".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "FIntCSGVoxelizerFadeRadius".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "FIntTerrainArraySliceSize".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "FIntRomarkStartWithGraphicQualityLevel".to_string(),
                    serde_json::json!(1),
                );
                fflags_obj.insert("FFlagMSRefactor5".to_string(), serde_json::json!(false));
                fflags_obj.insert(
                    "FIntDebugTextureManagerSkipMips".to_string(),
                    serde_json::json!(-1),
                );
                fflags_obj.insert(
                    "FFlagEnableQuickGameLaunch".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "FFlagGlobalWindActivated".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert("FFlagDebugSSAOForce".to_string(), serde_json::json!(false));
                fflags_obj.insert("FIntSSAOMipLevels".to_string(), serde_json::json!(0));
                fflags_obj.insert(
                    "FFlagAdServiceEnabled".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "FFlagEnableCommandAutocomplete".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert(
                    "FIntRobloxGuiBlurIntensity".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntAnimationLodFacsDistanceMin".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntAnimationLodFacsDistanceMax".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert(
                    "DFIntAnimationLodFacsVisibilityDenominator".to_string(),
                    serde_json::json!(0),
                );
                fflags_obj.insert("FIntViewportFrameMaxSize".to_string(), serde_json::json!(0));
                fflags_obj.insert(
                    "FFlagUseUnifiedRenderStepped".to_string(),
                    serde_json::json!(false),
                );
                fflags_obj.insert("DFIntMaxFrameBufferSize".to_string(), serde_json::json!(4));
            }

            // Tuxstrap Network Optimization FFlags
            if config.enable_network_optimization {
                fflags_obj.insert("FFlagOptimizeNetwork".to_string(), serde_json::json!(true));
                fflags_obj.insert(
                    "FFlagOptimizeNetworkTransport".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert("DFIntConnectionMTUSize".to_string(), serde_json::json!(900));
                fflags_obj.insert("FFlagEnableNewInput".to_string(), serde_json::json!(true));

                // Voidstrap network optimizations
                fflags_obj.insert("DFIntRccMaxPayloadSnd".to_string(), serde_json::json!(131072));
                fflags_obj.insert("DFIntCliMaxPayloadRcv".to_string(), serde_json::json!(131072));
                fflags_obj.insert("DFIntCliMaxPayloadSnd".to_string(), serde_json::json!(131072));
                fflags_obj.insert("DFIntRccMaxPayloadRcv".to_string(), serde_json::json!(131072));
                fflags_obj.insert("DFIntCliTcMaxPayloadRcv".to_string(), serde_json::json!(65536));
                fflags_obj.insert("DFIntRccTcMaxPayloadRcv".to_string(), serde_json::json!(65536));
                fflags_obj.insert("DFIntCliTcMaxPayloadSnd".to_string(), serde_json::json!(65536));
                fflags_obj.insert("DFIntRccTcMaxPayloadSnd".to_string(), serde_json::json!(65536));

                // Network buffering
                fflags_obj.insert("DFIntBandwidthManagerApplicationDefaultBps".to_string(), serde_json::json!(10485760));
                fflags_obj.insert("DFIntBandwidthManagerDataSenderMaxWorkCatchupMs".to_string(), serde_json::json!(20));

                // Asset preloading
                fflags_obj.insert("DFFlagEnableMeshPreloading2".to_string(), serde_json::json!(true));
                fflags_obj.insert("DFIntNumAssetsMaxToPreload".to_string(), serde_json::json!(100));
            }

            // Tuxstrap Wayland Clipboard FFlags
            if config.enable_wayland_clipboard {
                fflags_obj.insert(
                    "FFlagClientAllowClipboardControl".to_string(),
                    serde_json::json!(true),
                );
                fflags_obj.insert("FFlagClientAllowDBus".to_string(), serde_json::json!(true));
                fflags_obj.insert("FFlagIsLinux".to_string(), serde_json::json!(true));
            }

            // Voidstrap Extra Rendering FFlags
            // Remove grass for performance
            fflags_obj.insert("FIntFRMMinGrassDistance".to_string(), serde_json::json!(0));
            fflags_obj.insert("FIntFRMMaxGrassDistance".to_string(), serde_json::json!(0));
            fflags_obj.insert("FIntRenderGrassDetailStrands".to_string(), serde_json::json!(0));

            // Better visuals
            fflags_obj.insert("FFlagRenderFixFog".to_string(), serde_json::json!(true));

            // CPU threading optimizations
            fflags_obj.insert("DFIntRuntimeConcurrency".to_string(), serde_json::json!(4));
            fflags_obj.insert("DFIntInterpolationNumParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("DFIntMegaReplicatorNumParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("DFIntNetworkClusterPacketCacheNumParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("DFIntReplicationDataCacheNumParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("FIntLuaGcParallelMinMultiTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("FIntSmoothClusterTaskQueueMaxParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("DFIntPhysicsReceiveNumParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("FIntTaskSchedulerAutoThreadCount".to_string(), serde_json::json!(4));
            fflags_obj.insert("FIntSimWorldTaskQueueParallelTasks".to_string(), serde_json::json!(4));
            fflags_obj.insert("FIntTaskSchedulerAsyncTasksMinimumThreadCount".to_string(), serde_json::json!(4));

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
