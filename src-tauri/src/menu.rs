use tauri::{menu::{Menu, MenuItem}, App, Runtime, AppHandle};

/// Creates and configures the application menu with a quit option.
/// 
/// # Arguments
/// 
/// * `app` - A reference to the Tauri application instance
/// 
/// # Returns
/// 
/// * `Result<Menu<R>, Box<dyn std::error::Error>>` - Returns the configured menu if successful,
///   or an error if menu creation fails
/// 
/// # Example
/// 
/// ```rust
/// let menu = create_menu(app)?;
/// ```
pub fn create_menu<R: Runtime>(app: &App<R>) -> Result<Menu<R>, Box<dyn std::error::Error>> {
    // Create menu items
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
    // Create menu with items
    let menu = Menu::with_items(app, &[&quit_item])?;
    
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
