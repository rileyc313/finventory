// Finventory: finds the episodes missing from your Jellyfin shows. All the work happens in ui/index.html.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Finventory failed to start");
}
