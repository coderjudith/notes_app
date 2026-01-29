use crate::models::Note;
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use std::sync::{Arc, Mutex}; // ✅ Keep this in storage.rs

pub struct NotesManager {
    pub notes: Vec<Note>,
    storage_path: String,
}

impl NotesManager {
    pub fn new(storage_path: &str) -> io::Result<Self> {
        let notes = Self::load_notes(storage_path)?;
        Ok(NotesManager {
            notes,
            storage_path: storage_path.to_string(),
        })
    }

    fn load_notes(path: &str) -> io::Result<Vec<Note>> {
        let path = Path::new(path);

        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(notes) => Ok(notes),
            Err(_) => Ok(Vec::new()),
        }
    }

    pub fn save_notes(&self) -> io::Result<()> {
        let path = Path::new(&self.storage_path);
        let parent = path.parent().unwrap_or(Path::new("."));

        fs::create_dir_all(parent)?;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.notes)?;

        Ok(())
    }

    pub fn add_note(
        &mut self,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> io::Result<Note> {
        let note = Note::new(title, content, tags);
        self.notes.push(note.clone());
        self.save_notes()?;
        Ok(note)
    }

    pub fn list_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    pub fn get_note(&self, id: &str) -> Option<Note> {
        self.notes.iter().find(|note| note.id == id).cloned()
    }

    pub fn get_note_by_index(&self, index: usize) -> Option<&Note> {
        self.notes.get(index)
    }

    pub fn search_notes(&self, query: &str) -> Vec<Note> {
        let query_lower = query.to_lowercase();
        self.notes
            .iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower)
                    || note.content.to_lowercase().contains(&query_lower)
                    || note
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    pub fn delete_note(&mut self, id: &str) -> io::Result<bool> {
        let initial_len = self.notes.len();
        self.notes.retain(|note| note.id != id);
        let removed = self.notes.len() < initial_len;
        if removed {
            self.save_notes()?;
        }
        Ok(removed)
    }

    pub fn update_note(
        &mut self,
        id: &str,
        title: Option<String>,
        content: Option<String>,
        tags: Option<Vec<String>>,
    ) -> io::Result<Option<Note>> {
        // Find index first
        if let Some(index) = self.notes.iter().position(|note| note.id == id) {
            // Update the note
            self.notes[index].update(title, content, tags);
            let updated_note = self.notes[index].clone();
            self.save_notes()?;
            Ok(Some(updated_note))
        } else {
            Ok(None)
        }
    }

    pub fn delete_note_by_index(&mut self, index: usize) -> io::Result<()> {
        if index < self.notes.len() {
            self.notes.remove(index);
            self.save_notes()
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid index"))
        }
    }

    pub fn notes_count(&self) -> usize {
        self.notes.len()
    }
}

pub type SharedNotesManager = Arc<Mutex<NotesManager>>;
