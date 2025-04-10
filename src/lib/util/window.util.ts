import { invoke } from '@tauri-apps/api/core';

export function exitApplication() {
	invoke('exit_application');
}

export function minimizeApplication() {
  invoke('minimize_application');
}

export function maximizeApplication() {
  invoke('maximize_application');
}