use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

pub struct Note {
    pub path: PathBuf,
    pub content: String,
}

pub struct NotesStore {
    pub notes: RwLock<Vec<Note>>,
}

impl NotesStore {
    pub fn new() -> Self {
        Self {
            notes: RwLock::new(Vec::new()),
        }
    }

    pub fn load_all(&self, dir: &Path) {
        let mut loaded = Vec::new();
        let walker = WalkBuilder::new(dir)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .ignore(false)
            .hidden(false)
            .build();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };
            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let path = entry.path();
            let is_target = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("md") | Some("txt")
            );
            if !is_target {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(path) {
                loaded.push(Note {
                    path: path.to_path_buf(),
                    content,
                });
            }
        }

        *self.notes.write().unwrap() = loaded;
    }
}
