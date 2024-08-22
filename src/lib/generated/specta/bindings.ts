/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function exit() {
    return invoke()<null>("exit")
}

export function windowFocus() {
    return invoke()<null>("window_focus")
}

export function windowHide() {
    return invoke()<null>("window_hide")
}

export function openWindow(label: string, url: string, title: string | null) {
    return invoke()<null>("open_window", { label,url,title })
}

export function closeWindow(label: string) {
    return invoke()<null>("close_window", { label })
}

export function getWindows() {
    return invoke()<WindowData[]>("get_windows")
}

export type WindowData = { title: string; label: string; zoom: number }
