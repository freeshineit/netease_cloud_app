use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

// 托盘菜单
pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");

    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(quit)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(hide),
    )
}

// 菜单
pub fn menus() -> Menu {
    let submenu_music = Submenu::new(
        "Music",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "about_music".to_string(),
                "About Music",
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "preferences".to_string(),
                "Preferences",
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("hide_music".to_string(), "Hide Music"))
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let submenu_edit = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            // .add_native_item(MenuItem::About((), ()))
            .add_item(CustomMenuItem::new("delete".to_string(), "Delete"))
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            // 开始听写
            .add_item(CustomMenuItem::new(
                "start_dictation".to_string(),
                "Start Dictation",
            )),
    );

    let submenu_controls = Submenu::new(
        "Controls",
        Menu::new()
            .add_item(CustomMenuItem::new("play".to_string(), "Play"))
            .add_item(CustomMenuItem::new("next".to_string(), "Next"))
            .add_item(CustomMenuItem::new("previous".to_string(), "Previous"))
            .add_item(CustomMenuItem::new(
                "increase_volume".to_string(),
                "Increase Volume",
            ))
            .add_item(CustomMenuItem::new(
                "decrease_volume".to_string(),
                "Decrease Volume",
            ))
            .add_item(CustomMenuItem::new("dislike".to_string(), "Dislike"))
            .add_item(CustomMenuItem::new("repeat".to_string(), "Repeat"))
            .add_item(CustomMenuItem::new("shuffle".to_string(), "Shuffle"))
            .add_item(CustomMenuItem::new("heart_beat".to_string(), "Heart Beat"))
            .add_item(CustomMenuItem::new(
                "show_hide_lyrics".to_string(),
                "Show/Hide Lyrics",
            ))
            .add_item(CustomMenuItem::new(
                "customize_touch_bar".to_string(),
                "Customize Touch Bar",
            )),
    );

    let submenu_window = Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Zoom)
            .add_item(CustomMenuItem::new(
                "tile_window_to_left_of_screen".to_string(),
                "Tile Window to Left of Screen",
            ))
            .add_item(CustomMenuItem::new(
                "tile_window_to_right_of_screen".to_string(),
                "Tile Window to Right of Screen",
            ))
            .add_item(CustomMenuItem::new(
                "replace_tiled_window".to_string(),
                "Replace Tiled Window",
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::EnterFullScreen)
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "show_previous_tab".to_string(),
                "Show Previous Tab",
            ))
            .add_item(CustomMenuItem::new(
                "show_next_tab".to_string(),
                "Show Next Tab",
            ))
            .add_item(CustomMenuItem::new(
                "move_tab_to_new_window".to_string(),
                "Move Tab to New Window",
            ))
            .add_item(CustomMenuItem::new(
                "merge_all_windows".to_string(),
                "Merge All Windows",
            ))
            .add_item(CustomMenuItem::new(
                "show_tab_bar".to_string(),
                "Show Tab Bar",
            ))
            .add_item(CustomMenuItem::new(
                "show_all_tabs".to_string(),
                "Show All Tabs",
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "switch_to_mini_player".to_string(),
                "Switch To MiniPlayer",
            ))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "bring_all_to_front".to_string(),
                "Bring All to Front",
            )),
    );

    let submenu_help = Submenu::new(
        "Help",
        Menu::new().add_item(CustomMenuItem::new(
            "terms_service".to_string(),
            "Terms of Service",
        )),
    );

    Menu::new()
        .add_submenu(submenu_music)
        .add_submenu(submenu_edit)
        .add_submenu(submenu_controls)
        .add_submenu(submenu_window)
        .add_submenu(submenu_help)
}
