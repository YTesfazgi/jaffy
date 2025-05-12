// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod ffmpeg;

use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Create menu items
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_item])?;

            // Create and setup tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
									"quit" => {
										println!("quit menu item was clicked");
										app.exit(0);
									}
									_ => {
										println!("menu item {:?} not handled", event.id);
									}
								})
								.on_tray_icon_event(|tray, event| match event {
									TrayIconEvent::Click {
										button: MouseButton::Left,
										button_state: MouseButtonState::Up,
										..
									} => {
										println!("left click pressed and released");
										// in this example, let's show and focus the main window when the tray is clicked
										let app = tray.app_handle();
										if let Some(window) = app.get_webview_window("main") {
											let _ = window.show();
											let _ = window.set_focus();
										}
									}
									_ => {
										println!("unhandled event {event:?}");
									}
								})
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ffmpeg::start_ffmpeg,
            ffmpeg::stop_ffmpeg,
            ffmpeg::ffmpeg_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
