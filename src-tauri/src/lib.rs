use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WebviewWindow,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

// Window sizes
const PILL_W: f64 = 260.0;
const PILL_H: f64 = 48.0;
const PANEL_W: f64 = 1100.0;
const PANEL_H: f64 = 740.0;

#[tauri::command]
fn set_expanded(window: WebviewWindow, expanded: bool) {
    if expanded {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: PANEL_W,
            height: PANEL_H,
        }));
        let _ = window.set_resizable(true);
    } else {
        let _ = window.set_resizable(false);
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: PILL_W,
            height: PILL_H,
        }));
    }
}

#[tauri::command]
fn set_ignore_mouse(window: WebviewWindow, ignore: bool) {
    let _ = window.set_ignore_cursor_events(ignore);
}

#[tauri::command]
fn start_drag(window: WebviewWindow) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Build system tray menu
            let quit_item = MenuItemBuilder::with_id("quit", "Quit H.U.D Manager Overlay").build(app)?;
            let toggle_item = MenuItemBuilder::with_id("toggle", "Toggle (Ctrl+Shift+H)").build(app)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&toggle_item)
                .separator()
                .item(&quit_item)
                .build()?;

            TrayIconBuilder::new()
                .menu(&tray_menu)
                .tooltip("H.U.D Manager Overlay")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "toggle" => {
                        if let Some(w) = app.get_webview_window("overlay") {
                            let _ = w.emit("hotkey-toggle", ());
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(w) = tray.app_handle().get_webview_window("overlay") {
                            let _ = w.emit("hotkey-toggle", ());
                        }
                    }
                })
                .build(app)?;

            // Register global hotkey Ctrl+Shift+H
            let app_handle = app.handle().clone();
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyH);
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                if let Some(w) = app_handle.get_webview_window("overlay") {
                    let _ = w.emit("hotkey-toggle", ());
                }
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_expanded,
            set_ignore_mouse,
            start_drag,
            quit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
