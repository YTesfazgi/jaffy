use thiserror::Error;
use tauri::Error as TauriError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Tray error: {0}")]
    Tray(#[from] TrayError),
    
    #[error("Menu error: {0}")]
    Menu(#[from] MenuError),
    
    #[error("Window error: {0}")]
    Window(#[from] WindowError),
    
    #[error("FFmpeg error: {0}")]
    FFmpeg(#[from] FFmpegError),

    #[error("Tauri error: {0}")]
    Tauri(#[from] TauriError),
}

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Failed to create tray icon: {0}")]
    Creation(#[from] TauriError),
    
    #[error("Failed to set tray menu: {0}")]
    Menu(#[from] MenuError),
}

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("Failed to create menu item: {0}")]
    ItemCreation(String),
    
    #[error("Failed to create menu: {0}")]
    Creation(String),
    
    #[error("Failed to set menu: {0}")]
    SetMenu(String),
}

impl From<TauriError> for MenuError {
    fn from(error: TauriError) -> Self {
        MenuError::Creation(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Failed to handle window event: {0}")]
    EventHandling(String),
    
    #[error("Window not found: {0}")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum FFmpegError {
    #[error("Failed to start FFmpeg process: {0}")]
    StartProcess(String),
    
    #[error("Failed to stop FFmpeg process: {0}")]
    StopProcess(String),
    
    #[error("Failed to get FFmpeg status: {0}")]
    Status(String),
} 