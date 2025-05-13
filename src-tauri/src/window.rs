use tauri::{AppHandle, Manager, RunEvent};
use crate::errors::WindowError;

/// Handles all window-related events for the application.
///
/// # Arguments
///
/// * `app_handle` - The application handle
/// * `event` - The run event to handle
pub fn handle_window_events(app_handle: &AppHandle, event: &RunEvent) -> Result<(), WindowError> {
    match event {
        RunEvent::ExitRequested { api, code, .. } => {
            // Keep the event loop running even if all windows are closed
            // This allows us to catch tray icon events when there is no window
            // if we manually requested an exit (code is Some(_)) we will let it go through
            if code.is_none() {
                api.prevent_exit();
            }
        }
        RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            label,
            ..
        } => {
            println!("closing window...");
            // Prevent the window from closing and hide it instead
            api.prevent_close();
            if let Some(window) = app_handle.get_webview_window(label) {
                let _ = window.hide();
            } else {
                return Err(WindowError::NotFound(label.to_string()));
            }
        }
        _ => (),
    }
    Ok(())
}
