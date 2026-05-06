mod commands;
mod notes;
mod search;

use crate::notes::NotesStore;
use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let store = NotesStore::new();
    let notes_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .join("notes");
    store.load_all(&notes_dir);

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![commands::search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
