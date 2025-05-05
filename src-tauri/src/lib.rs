#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod modules;

pub mod database_connection;

use database_connection::mongo_database_connection::{
    get_database_from_mongo, test_mongo_connection,
};

use database_connection::app_database_connection::{
    app_database_init, create_file_for_database, create_new_query_block, delete_query_block,
    get_content_from_query_block, get_content_type, get_database_connection, get_datasource,
    get_file_collection, get_query_blocks, get_user_by_email, run_query_block,
    save_query_content_to_the_block, signup_user, store_file_with_database,
    update_query_block_content_type_id,
};
use database_connection::maria_database_connection::{
    run_query_block_maria, save_maria_connection, test_maria_connection,
};
use database_connection::mongo_database_connection::save_mongo_connection;
use database_connection::mssql_database_connection::{
    get_database_from_mssql, run_query_block_mssql, save_mssql_connection, test_mssql_connection,
};
use database_connection::mysql_database_connection::{
    get_database_from_mysql, run_query_block_mysql, save_mysql_connection, test_mysql_connection,
};
use database_connection::postgres_database_connection::{
    get_database_from_postgres, run_query_block_postgresql, save_postgres_connection,
    test_postgres_connection,
};
use database_connection::sqlite_database_connection::{
    get_database_from_sqlite, run_query_block_sqlite, save_sqlite_connection,
    test_sqlite_connection,
};

use crate::modules::bcrypt_controller::encrypt_bcrypt;
use crate::modules::docker_controller::{
    check_docker_status, delete_docker_container, delete_docker_image, get_all_docker_containers,
    get_all_docker_images, pull_docker_image, restart_docker_service, search_docker_image,
    start_docker_container, start_docker_service, stop_docker_container, stop_docker_service,
};
use crate::modules::terminal_controller::execute_shell_command;
use crate::modules::window_controller::{
    default_window_application, exit_application, maximize_application, minimize_application,
    refresh_window,
};

pub mod models;

pub mod information;
use crate::information::application_information::get_application_version;

pub mod dialog;
use crate::dialog::file_select_dialog::open_sqlite_file_selection_dialog;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            refresh_window,
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
            create_file_for_database,
            store_file_with_database,
            get_file_collection,
            create_new_query_block,
            get_query_blocks,
            save_query_content_to_the_block,
            update_query_block_content_type_id,
            delete_query_block,
            run_query_block,
            get_content_from_query_block,
            // Sqlite Database
            test_sqlite_connection,
            save_sqlite_connection,
            get_database_from_sqlite,
            run_query_block_sqlite,
            // Postgres Database
            test_postgres_connection,
            save_postgres_connection,
            get_database_from_postgres,
            run_query_block_postgresql,
            // Mongo Database
            test_mongo_connection,
            save_mongo_connection,
            get_database_from_mongo,
            // Mysql Database
            test_mysql_connection,
            save_mysql_connection,
            get_database_from_mysql,
            // Maria Database
            test_maria_connection,
            save_maria_connection,
            run_query_block_maria,
            // MSSQL Database
            test_mssql_connection,
            save_mssql_connection,
            get_database_from_mssql,
            // Dialogs
            open_sqlite_file_selection_dialog,
            // Information
            get_application_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
