// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::setup_logging;
use models::AppState;

mod auth;
mod config;
mod customer;
mod events;
mod models;
mod permissions;
mod users;

fn main() {
    setup_logging();

    tauri::Builder::default()
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
            permissions::get_permissions,
            permissions::export_user_permissions,
            permissions::export_all_user_permissions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
