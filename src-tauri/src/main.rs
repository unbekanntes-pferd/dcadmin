// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::setup_logging;
use models::AppState;
pub use models::{
    ROLE_AUDITOR, ROLE_CONFIG_MANAGER, ROLE_GROUP_MANAGER, ROLE_GUEST_USER, ROLE_ROOM_MANAGER,
    ROLE_USER_MANAGER,
};

mod auth;
mod config;
mod customer;
mod events;
mod groups;
mod models;
mod permissions;
pub(crate) mod users;

fn main() {
    let config_dir = config::get_or_create_config_dir();
    setup_logging(&config_dir, false);

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            auth::init_auth_code_flow,
            auth::connect,
            auth::validate_dracoon_url,
            customer::get_customer_info,
            events::get_events,
            events::get_operation_types,
            events::export_events,
            users::get_users,
            users::export_users,
            permissions::get_permissions,
            permissions::export_user_permissions,
            permissions::export_all_user_permissions,
            groups::get_group,
            groups::get_groups,
            groups::export_groups,
            groups::get_group_users,
            groups::export_group_users,
            groups::export_all_group_users,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
