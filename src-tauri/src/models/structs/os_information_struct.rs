use serde::Serialize;

#[derive(Serialize)]
pub struct OsInformation<'a> {
    pub arch: &'a str,
    pub exe_extension: String,
    pub family: &'a str,
    pub hostname: String,
    pub locale: Option<String>,
    pub platform: &'a str,
    pub type_: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct MemoryInformation {
    pub total_memory: u64,
    pub available_memory: u64,
}
