#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::commands;
use app::menus;

fn main() {
    let menus = menus::menus();

    let tray = menus::system_tray();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::my_custom_command])
        .menu(menus)
        // .on_menu_event()
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
