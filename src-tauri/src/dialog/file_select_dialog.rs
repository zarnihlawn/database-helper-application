use tauri::command;
use tauri_plugin_dialog::DialogExt;

#[command]
pub async fn open_sqlite_file_selection_dialog(
    app_handle: tauri::AppHandle,
) -> Result<Option<String>, String> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("SQLite Database", &["db", "sqlite", "sqlite3"])
        .blocking_pick_file();
    Ok(file_path.map(|p| p.to_string()))
}
