mod color;
// mod colors_inmemory;
// mod colors_sqlite;
mod colors_sqlite2;

use std::sync::Mutex;
use tauri::{Manager, State};
use crate::color::{Color, NewColor};
// use crate::colors_inmemory::ColorsInMemory;
// use crate::colors_sqlite::ColorsSqlite;
use crate::colors_sqlite2::ColorsSqlite2;

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Utf8(#[from] std::str::Utf8Error),
  #[error(transparent)]
  Sqlite(#[from] rusqlite::Error),  
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

struct AppData {
    header_text: &'static str,
    // colors: ColorsInMemory,
    colors: ColorsSqlite2,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increment(count: u32) -> u32 {
    count + 1
}

#[tauri::command]
fn get_header_text(state: State<'_, Mutex<AppData>>) -> String {
    let state = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Warning: Mutex is poisoned, recovering...");
            poisoned.into_inner()
        }
    };
    state.header_text.to_string()
}

#[tauri::command]
fn get_colors(state: State<'_, Mutex<AppData>>) -> Result<Vec<Color>, Error> {
    let state = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Warning: Mutex is poisoned, recovering...");
            poisoned.into_inner()
        }
    };
    Ok(state.colors.get_colors()?)
}

#[tauri::command]
fn add_color(state: State<'_, Mutex<AppData>>, new_color: NewColor) -> Result<(), Error> {
    let state = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Warning: Mutex is poisoned, recovering...");
            poisoned.into_inner()
        }
    };
    Ok(state.colors.add_color(new_color)?)
}

#[tauri::command]
fn delete_color(state: State<'_, Mutex<AppData>>, color_id_to_delete: usize) -> Result<(), Error> {
    let state = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("Warning: Mutex is poisoned, recovering...");
            poisoned.into_inner()
        }
    };
    Ok(state.colors.delete_color(color_id_to_delete)?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                header_text: "Tools App",
                colors: ColorsSqlite2::new("app.db").expect("Failed to initialize database"),
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            increment,
            get_header_text,
            get_colors,
            add_color,
            delete_color,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
