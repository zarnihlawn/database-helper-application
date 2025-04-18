#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod modules;

use crate::modules::bcrypt_controller::encrypt_bcrypt;
use crate::modules::docker_controller::{
    check_docker_status, delete_docker_container, delete_docker_image, get_all_docker_containers,
    get_all_docker_images, pull_docker_image, restart_docker_service, search_docker_image,
    start_docker_container, start_docker_service, stop_docker_container, stop_docker_service,
};
use crate::modules::terminal_controller::execute_shell_command;
use crate::modules::window_controller::{
    default_window_application, exit_application, maximize_application, minimize_application,
};

pub mod models;

pub mod database_connection;
use crate::database_connection::app_database_connection::{
    app_database_init, get_content_type, get_database_connection, get_datasource,
    get_user_by_email, signup_user,
};
use crate::database_connection::mongo_database_connection::{
    get_database_from_mongo, save_mongo_connection, test_mongo_connection,
};
use crate::database_connection::oracle_database_connection::{
    get_database_from_oracle, save_oracle_connection, test_oracle_connection,
};
use crate::database_connection::postgres_database_connection::{
    get_database_from_postgres, save_postgres_connection, test_postgres_connection,
};
use crate::database_connection::sqlite_database_connection::{
    get_database_from_sqlite, save_sqlite_connection, test_sqlite_connection,
};

pub mod dialog;
use crate::dialog::file_select_dialog::open_sqlite_file_selection_dialog;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app_database_init();

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // window Controller
            exit_application,
            minimize_application,
            maximize_application,
            default_window_application,
            // docker Controller
            check_docker_status,
            start_docker_service,
            stop_docker_service,
            restart_docker_service,
            get_all_docker_images,
            get_all_docker_containers,
            start_docker_container,
            stop_docker_container,
            delete_docker_container,
            delete_docker_image,
            search_docker_image,
            pull_docker_image,
            // terminal Controller
            execute_shell_command,
            // bcrypt_controller
            encrypt_bcrypt,
            // Database Connection
            // App Database
            get_user_by_email,
            signup_user,
            get_datasource,
            get_content_type,
            get_database_connection,
            // Sqlite Database
            test_sqlite_connection,
            save_sqlite_connection,
            get_database_from_sqlite,
            // Postgres Database
            test_postgres_connection,
            save_postgres_connection,
            get_database_from_postgres,
            // Mongo Database
            test_mongo_connection,
            save_mongo_connection,
            get_database_from_mongo,
            // Oracle Database
            test_oracle_connection,
            save_oracle_connection,
            get_database_from_oracle,
            // Dialogs
            open_sqlite_file_selection_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
