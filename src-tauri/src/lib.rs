#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default();
    
    #[cfg(not(debug_assertions))]
    {
        builder = builder.plugin(tauri_plugin_prevent_default::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
