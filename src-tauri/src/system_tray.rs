use tauri::{
    App, Manager, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[derive(Debug)]
pub struct TraySetup {}

impl TraySetup {
    pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&quit_i])?;
        TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(false)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "quit" => {
                    println!("Quit tray menu was clicked!");
                    app.exit(0);
                }
                _ => {
                    println!("Unknown menu item was clicked!")
                }
            })
            .on_tray_icon_event(|tray, event| {
                match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        println!("Tray Icon left pressed");
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    },
                    _ => {
                        println!("Unhandled event {event:?}")
                    }
                }
            })
            .build(app)?;
        Ok(())
    }
}
