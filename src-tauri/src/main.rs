#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::Rng;

#[tauri::command]
fn decide() -> String {
    let mut rng = rand::thread_rng();
    let decision: u8 = rng.gen_range(0..100);
    format!("{}", decision)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![decide])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
