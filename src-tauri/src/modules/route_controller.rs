use tauri::{AppHandle, Manager, Runtime};

pub fn go_to_tab_one<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.href = '/app/documentation-dashboard/';");
    }
    Ok(())
}
pub fn go_to_tab_two<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.href = '/app/docker-dashboard/';");
    }
    Ok(())
}
pub fn go_to_tab_three<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.href = '/app/workspace-dashboard/';");
    }
    Ok(())
}
pub fn go_to_tab_four<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.href = '/app/settings-dashboard/';");
    }
    Ok(())
}
