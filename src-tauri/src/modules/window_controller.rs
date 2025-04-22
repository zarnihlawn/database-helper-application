use tauri::{AppHandle, Manager, PhysicalSize, Runtime};

#[tauri::command]
pub async fn exit_application<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn minimize_application<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn maximize_application<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_maximized().unwrap_or(false) {
            default_window_application(app).await?;
        } else {
            window.maximize().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn refresh_window<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.reload();");
    }

    Ok(())
}

#[tauri::command]
pub async fn default_window_application<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_minimized().unwrap_or(false) {
            window.unminimize().map_err(|e| e.to_string())?;
        }

        // Get the monitor size
        let monitor = window
            .current_monitor()
            .map_err(|e| e.to_string())?
            .ok_or("No monitor detected")?;

        let scale_factor = monitor.scale_factor();

        let width = (1400.0_f64 * scale_factor) as u32;
        let height = (800.0_f64 * scale_factor) as u32;

        window
            .set_size(tauri::Size::Physical(PhysicalSize { width, height }))
            .map_err(|e| e.to_string())?;

        window.center().map_err(|e| e.to_string())?;

        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}
