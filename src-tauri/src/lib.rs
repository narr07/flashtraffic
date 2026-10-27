mod commands;
mod safety;
mod proxy;
mod browserstack;

use commands::*;

#[cfg(desktop)]
use tauri::{
	menu::{Menu, MenuItem},
	tray::TrayIconBuilder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_notification::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_store::Builder::new().build())
		.invoke_handler(tauri::generate_handler![
			start_campaign,
			stop_campaign,
			test_proxy_list,
			fetch_proxies_from_url,
			test_browserstack_credentials,
			fetch_sitemap_urls,
			get_engine_status,
			clear_campaign_logs,
			get_installed_browsers
		])
		.setup(|_app| {
			#[cfg(desktop)]
			{
				let quit = MenuItem::with_id(_app, "quit", "Quit FlashTraffic", true, None::<&str>)?;
				let menu = Menu::with_items(_app, &[&quit])?;

				TrayIconBuilder::new()
					.icon(_app.default_window_icon().expect("missing default window icon").clone())
					.menu(&menu)
					.show_menu_on_left_click(true)
					.on_menu_event(|app, event| {
						if event.id.as_ref() == "quit" {
							safety::kill_all_child_processes();
							app.exit(0);
						}
					})
					.build(_app)?;
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
