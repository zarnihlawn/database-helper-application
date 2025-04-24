use tauri::AppHandle;

#[tauri::command]
pub async fn get_application_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}
