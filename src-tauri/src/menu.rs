use tauri::{menu::{Menu, MenuItem}, App, Runtime, AppHandle};
use crate::errors::MenuError;

/// Creates and configures the application menu with a quit option.
/// 
/// # Arguments
/// 
/// * `app` - A reference to the Tauri application instance
/// 
/// # Returns
/// 
/// * `Result<Menu<R>, MenuError>` - Returns the configured menu if successful,
///   or an error if menu creation fails
/// 
/// # Example
/// 
/// ```rust
/// let menu = create_menu(app)?;
/// ```
pub fn create_menu<R: Runtime>(app: &App<R>) -> Result<Menu<R>, MenuError> {
    // Create menu items
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)
        .map_err(|e| MenuError::ItemCreation(e.to_string()))?;
    
    // Create menu with items
    let menu = Menu::with_items(app, &[&quit_item])
        .map_err(|e| MenuError::Creation(e.to_string()))?;
    
    Ok(menu)
}

/// Handles menu events for the application.
/// 
/// # Arguments
/// 
/// * `app` - A reference to the Tauri application handle
/// * `event` - The menu event to handle
/// 
/// # Example
/// 
/// ```rust
/// on_menu_event(app, event);
/// ```
pub fn on_menu_event<R: Runtime>(app: &AppHandle<R>, event: &tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}

/// Sets up the application menu by creating and setting it.
/// 
/// # Arguments
/// 
/// * `app` - A reference to the Tauri application instance
/// 
/// # Returns
/// 
/// * `Result<(), MenuError>` - Returns Ok if successful, or an error if menu setup fails
pub fn setup_menu<R: Runtime>(app: &App<R>) -> Result<(), MenuError> {
    let menu = create_menu(app)?;
    app.set_menu(menu)
        .map_err(|e| MenuError::SetMenu(e.to_string()))?;
    Ok(())
}
