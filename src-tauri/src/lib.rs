mod commands;
mod notes;
mod search;

use crate::notes::NotesStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let store = NotesStore::new();
    let notes_dir = dirs::home_dir()
        .expect("home directory not found")
        .join("notes");
    store.load_all(&notes_dir);

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![commands::search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
