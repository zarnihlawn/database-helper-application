import { invoke } from "@tauri-apps/api/core";

export async function getIpV4Address(): Promise<string> {
  return await invoke<string>('get_ipv4_address');
}