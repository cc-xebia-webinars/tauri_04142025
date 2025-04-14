use std::sync::Mutex;
use tauri::{Manager, State};


struct AppData {
    colors: Vec<String>,
}

#[tauri::command]
fn get_colors(state: State<'_, Mutex<AppData>>) -> Vec<String> {
    let app_data = state.lock().unwrap();
    app_data.colors.clone()
}

#[tauri::command]
fn add_color(state: State<'_, Mutex<AppData>>, new_color: String) {
    let mut state = state.lock().unwrap();
    state.colors.push(new_color);
}

#[tauri::command]
fn delete_color(state: State<'_, Mutex<AppData>>, color_to_delete: String) {
    let mut state = state.lock().unwrap();

    let mut color_index: Option<usize> = None;
    for (i, color) in state.colors.iter().enumerate() {
        if *color == color_to_delete {
            color_index = Some(i);
        }
    }

    if let Some(color_index) = color_index {
        state.colors.remove(color_index);
    }
}


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increment(count: i32) -> i32 {
    count + 1
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                colors: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, increment, get_colors, add_color, delete_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
