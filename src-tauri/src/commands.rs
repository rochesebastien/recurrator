use crate::notes::NotesStore;
use crate::search::{search as run_search, Match, SearchMode};
use tauri::State;

#[tauri::command]
pub fn search(query: String, state: State<'_, NotesStore>) -> Vec<Match> {
    let notes = state.notes.read().unwrap();
    run_search(&query, notes.as_slice(), SearchMode::Literal)
}
