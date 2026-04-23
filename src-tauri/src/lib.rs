use tauri::{
    menu::{Menu, MenuItem, Submenu},
    Manager, Url,
};

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use tauri::webview::PlatformWebview;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use webkit2gtk::{
    glib::prelude::ObjectExt,
    NotificationPermissionRequest, PermissionRequest, SettingsExt, UserMediaPermissionRequest,
    WebViewExt,
    PermissionRequestExt,
};

const REMOTE_APP_URL: &str = "https://qxp.kisakay.com/#/";
const RESET_WEB_PERMISSIONS_MENU_ID: &str = "reset-web-permissions";

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let handle = app.handle();
            let default_menu = Menu::default(&handle)?;
            let reset_permissions_item = MenuItem::with_id(
                app,
                RESET_WEB_PERMISSIONS_MENU_ID,
                "Reinitialiser les permissions web",
                true,
                Some("CmdOrCtrl+Shift+R"),
            )?;
            let tools_menu = Submenu::with_items(
                app,
                "Outils",
                true,
                &[&reset_permissions_item],
            )?;
            default_menu.append(&tools_menu)?;
            app.set_menu(default_menu)?;

            app.on_menu_event(|app, event| {
                if event.id().as_ref() != RESET_WEB_PERMISSIONS_MENU_ID {
                    return;
                }

                if let Some(webview_window) = app.get_webview_window("main") {
                    if let Err(error) = webview_window.clear_all_browsing_data() {
                        eprintln!("failed to clear browsing data: {error}");
                    }

                    match Url::parse(REMOTE_APP_URL) {
                        Ok(url) => {
                            if let Err(error) = webview_window.navigate(url) {
                                eprintln!("failed to navigate after clearing browsing data: {error}");
                                let _ = webview_window.reload();
                            }
                        }
                        Err(error) => {
                            eprintln!("failed to parse remote app url: {error}");
                            let _ = webview_window.reload();
                        }
                    }
                }
            });

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            {
                let webview_window = app.get_webview_window("main").expect("main window not found");
                webview_window.with_webview(|webview: PlatformWebview| {
                    let webview = webview.inner();

                    if let Some(settings) = webview.settings() {
                        settings.set_enable_media(true);
                        settings.set_enable_media_stream(true);
                        settings.set_enable_webrtc(true);
                        settings.set_media_playback_requires_user_gesture(false);
                    }

                    webview.connect_permission_request(|_, request: &PermissionRequest| {
                        if request.is::<UserMediaPermissionRequest>()
                            || request.is::<NotificationPermissionRequest>()
                        {
                            request.allow();
                            return true;
                        }

                        false
                    });
                })?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running LQXP Client");
}
