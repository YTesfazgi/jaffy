// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod ffmpeg;
mod app;
mod menu;
mod tray;
mod window;

use tauri::{
    Manager, RunEvent
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Create and setup tray icon
            tray::setup_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ffmpeg::start_ffmpeg,
            ffmpeg::stop_ffmpeg,
            ffmpeg::ffmpeg_status
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_app_handle, event| {
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
                    if let Some(window) = _app_handle.get_webview_window(&label) {
                        let _ = window.hide();
                    }
                }
                _ => (),
            }
        });
}
