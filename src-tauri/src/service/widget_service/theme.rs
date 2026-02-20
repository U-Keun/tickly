use super::*;

impl WidgetService {
    pub(super) fn resolve_widget_theme(conn: &Connection) -> WidgetTheme {
        let default_theme = Self::default_widget_theme();
        let raw_theme = match SettingsRepository::get(conn, THEME_SETTING_KEY) {
            Ok(Some(value)) => value,
            Ok(None) | Err(_) => return default_theme,
        };

        let saved_theme: SavedThemeSetting = match serde_json::from_str(&raw_theme) {
            Ok(theme) => theme,
            Err(error) => {
                log::warn!("Failed to parse saved theme for widget snapshot: {}", error);
                return default_theme;
            }
        };

        if let Some(custom_colors) = saved_theme.custom_colors {
            return Self::theme_from_custom_colors(custom_colors, &default_theme);
        }

        if let Some(preset_id) = saved_theme.preset_id {
            if let Some(preset_theme) = Self::theme_from_preset_id(&preset_id) {
                return preset_theme;
            }
        }

        default_theme
    }

    fn theme_from_custom_colors(
        colors: SavedThemeColors,
        default_theme: &WidgetTheme,
    ) -> WidgetTheme {
        WidgetTheme {
            paper: colors.paper.unwrap_or_else(|| default_theme.paper.clone()),
            canvas: colors
                .canvas
                .unwrap_or_else(|| default_theme.canvas.clone()),
            stroke: colors
                .stroke
                .unwrap_or_else(|| default_theme.stroke.clone()),
            ink: colors.ink.unwrap_or_else(|| default_theme.ink.clone()),
            ink_muted: colors
                .ink_muted
                .unwrap_or_else(|| default_theme.ink_muted.clone()),
            accent_sky: colors
                .accent_sky
                .unwrap_or_else(|| default_theme.accent_sky.clone()),
            accent_sky_strong: colors
                .accent_sky_strong
                .unwrap_or_else(|| default_theme.accent_sky_strong.clone()),
        }
    }

    fn theme_from_preset_id(preset_id: &str) -> Option<WidgetTheme> {
        let normalized = preset_id.trim().to_lowercase();
        let theme = match normalized.as_str() {
            "default" => WidgetTheme {
                paper: "#f8f7f3".to_string(),
                canvas: "#f2efe8".to_string(),
                stroke: "#e2ded5".to_string(),
                ink: "#5b5852".to_string(),
                ink_muted: "#7a776f".to_string(),
                accent_sky: "#a8bddb".to_string(),
                accent_sky_strong: "#8ea9cf".to_string(),
            },
            "dark" => WidgetTheme {
                paper: "#1f2937".to_string(),
                canvas: "#111827".to_string(),
                stroke: "#4b5563".to_string(),
                ink: "#f3f4f6".to_string(),
                ink_muted: "#9ca3af".to_string(),
                accent_sky: "#0042a9".to_string(),
                accent_sky_strong: "#3a87fe".to_string(),
            },
            "ocean" => WidgetTheme {
                paper: "#e0f2fe".to_string(),
                canvas: "#bae6fd".to_string(),
                stroke: "#38bdf8".to_string(),
                ink: "#0c4a6e".to_string(),
                ink_muted: "#075985".to_string(),
                accent_sky: "#0284c7".to_string(),
                accent_sky_strong: "#0369a1".to_string(),
            },
            "forest" => WidgetTheme {
                paper: "#ecfdf5".to_string(),
                canvas: "#cde8b5".to_string(),
                stroke: "#38571a".to_string(),
                ink: "#263e0f".to_string(),
                ink_muted: "#4e7a27".to_string(),
                accent_sky: "#6f760a".to_string(),
                accent_sky_strong: "#4f5504".to_string(),
            },
            "sunset" => WidgetTheme {
                paper: "#fef3c7".to_string(),
                canvas: "#fde68a".to_string(),
                stroke: "#fbbf24".to_string(),
                ink: "#78350f".to_string(),
                ink_muted: "#92400e".to_string(),
                accent_sky: "#ff9300".to_string(),
                accent_sky_strong: "#ff6a00".to_string(),
            },
            _ => return None,
        };

        Some(theme)
    }

    fn default_widget_theme() -> WidgetTheme {
        Self::theme_from_preset_id("default").unwrap_or(WidgetTheme {
            paper: "#f8f7f3".to_string(),
            canvas: "#f2efe8".to_string(),
            stroke: "#e2ded5".to_string(),
            ink: "#5b5852".to_string(),
            ink_muted: "#7a776f".to_string(),
            accent_sky: "#a8bddb".to_string(),
            accent_sky_strong: "#8ea9cf".to_string(),
        })
    }
}
