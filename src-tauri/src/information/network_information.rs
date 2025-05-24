use get_if_addrs::IfAddr;

#[tauri::command]
pub async fn get_ipv4_address() -> String {
    let if_addrs = get_if_addrs::get_if_addrs().unwrap();
    for iface in if_addrs {
        if let IfAddr::V4(addr) = iface.addr {
            if addr.ip.is_private() {
                println!("Interface: {}", iface.name);
                println!("IP Address: {}", addr.ip);
                return addr.ip.to_string();
            }
        }
    }
    String::from("No private IPv4 address found")
}
