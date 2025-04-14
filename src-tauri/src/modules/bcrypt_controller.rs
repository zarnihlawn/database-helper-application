extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

#[tauri::command]
pub async fn encrypt_bcrypt(ctx: String) {}
