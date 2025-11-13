mod commands;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::greet,
      commands::add_numbers,
      commands::get_user,
      commands::create_user,
      commands::search_products,
      commands::update_order_status,
      commands::create_order,
      commands::update_settings,
      commands::process_task,
      commands::stream_logs,
      commands::monitor_system,
      commands::divide,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
