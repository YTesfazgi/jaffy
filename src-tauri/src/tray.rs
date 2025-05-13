use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager
};

use crate::menu;
use crate::errors::TrayError;

/// Creates and sets up the tray icon for the application
/// 
/// # Arguments
/// * `app` - The Tauri application handle
/// 
/// # Returns
/// * `Result<(), TrayError>` - Result indicating success or failure
pub fn setup_tray(app: &mut App) -> Result<(), TrayError> {
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu::create_menu(app)?)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| menu::on_menu_event(app, &event))
        .on_tray_icon_event(|tray, event| handle_tray_click(tray, event))
        .build(app)?;
    Ok(())
}

/// Handles tray icon click events
/// 
/// # Arguments
/// * `tray` - The tray icon handle
/// * `event` - The tray icon event
fn handle_tray_click(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    }
}
