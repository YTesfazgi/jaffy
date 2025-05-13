// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod ffmpeg;
mod app;
mod menu;
mod tray;
mod window;
mod errors; 
use errors::AppError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), AppError> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(app::setup)
        .invoke_handler(tauri::generate_handler![
            ffmpeg::start_ffmpeg,
            ffmpeg::stop_ffmpeg,
            ffmpeg::ffmpeg_status
        ])
        .build(tauri::generate_context!())
        .map_err(AppError::from)?
        .run(move |app_handle, event| {
            if let Err(e) = window::handle_window_events(app_handle, &event) {
                eprintln!("Window event error: {}", e);
            }
        });
    
    Ok(())
}
