use std::process::Command;

#[tauri::command]
pub async fn execute_shell_command(command: String) -> Result<String, String> {
    let (shell, args) = if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", &command])
    } else {
        ("sh", vec!["-c", &command])
    };

    let output = Command::new(shell)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
