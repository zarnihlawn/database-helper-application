use sysinfo::System;
use tauri_plugin_os::{arch, exe_extension, family, hostname, locale, platform, type_, version};

use crate::models::structs::os_information_struct::{MemoryInformation, OsInformation};

#[tauri::command]
pub fn get_os_information() -> OsInformation<'static> {
    OsInformation {
        arch: arch(),
        exe_extension: exe_extension().to_string(),
        family: family(),
        hostname: hostname(),
        locale: locale(),
        platform: platform(),
        type_: type_().to_string(),
        version: version().to_string(),
    }
}

#[tauri::command]
pub fn get_memory_information() -> MemoryInformation {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory = sys.total_memory();
    let available_memory = sys.available_memory();

    MemoryInformation {
        total_memory,
        available_memory,
    }
}
