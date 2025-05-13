use tauri::App;
use std::error::Error;

/// Initializes the application by setting up plugins, tray, and menu.
/// 
/// # Arguments
/// 
/// * `app` - The Tauri application instance
/// 
/// # Returns
/// 
/// Returns a Result that contains the app handle if successful, or an error if initialization fails.
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    // Setup tray icon
    super::tray::setup_tray(app)?;

    // Setup menu
    super::menu::setup_menu(app)?;

    Ok(())
}
