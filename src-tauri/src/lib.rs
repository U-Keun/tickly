mod commands;
mod models;
mod repository;
mod service;

use commands::{OAuthStateStore, RealtimeState, *};
use repository::init_database;
use rusqlite::Connection;
use service::{SupabaseClient, SupabaseConfig};
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub supabase: Option<SupabaseClient>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_deep_link::init());

    // Add Sign In with Apple plugin on iOS
    #[cfg(target_os = "ios")]
    {
        builder = builder.plugin(tauri_plugin_sign_in_with_apple::init());
    }

    builder
        .setup(|app| {
            let conn = init_database(app.handle())?;

            // Initialize Supabase client if config is available
            let supabase = SupabaseConfig::from_env().map(SupabaseClient::new);

            if supabase.is_none() {
                log::warn!("Supabase not configured. Cloud sync will be disabled.");
            }

            app.manage(AppState {
                db: Mutex::new(conn),
                supabase,
            });

            // Initialize OAuth state store for Desktop Google OAuth
            app.manage(OAuthStateStore {
                code_verifier: Mutex::new(None),
                port: Mutex::new(None),
            });

            // Initialize Realtime state
            app.manage(RealtimeState::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Todo commands
            add_item,
            get_items,
            toggle_item,
            delete_item,
            edit_item,
            update_item_memo,
            update_item_repeat,
            reorder_items,
            reset_all_items,
            check_and_auto_reset,
            process_repeats,
            // Category commands
            get_categories,
            add_category,
            edit_category,
            delete_category,
            reorder_categories,
            // Settings commands
            get_setting,
            set_setting,
            // Streak commands
            get_tracked_items,
            get_item_heatmap_data,
            update_track_streak,
            // Auth commands
            sign_in_with_apple,
            sign_in_with_google,
            get_current_session,
            refresh_session,
            sign_out,
            get_user_profile,
            is_logged_in,
            // Mobile OAuth commands (iOS/Android)
            start_mobile_google_oauth,
            complete_mobile_google_oauth,
            // Desktop OAuth commands
            start_google_oauth,
            complete_google_oauth,
            // Sync commands
            trigger_sync,
            get_sync_status,
            get_pending_count,
            set_sync_enabled,
            is_sync_enabled,
            force_pull,
            // Tag commands
            get_all_tags,
            create_tag,
            delete_tag,
            add_tag_to_item,
            remove_tag_from_item,
            get_tags_for_item,
            get_items_by_tag,
            // Realtime commands
            connect_realtime,
            disconnect_realtime,
            get_realtime_status,
            is_realtime_connected
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
