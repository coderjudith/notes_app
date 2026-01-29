mod models;
mod storage;
mod web;

use chrono::Local;
use colored::*;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use storage::{NotesManager, SharedNotesManager};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn display_header(title: &str) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!(
        "{}",
        format!("  {}  ", title)
            .bright_white()
            .on_bright_blue()
            .bold()
    );
    println!("{}", "═".repeat(60).bright_blue());
}

fn cli_mode(manager: SharedNotesManager) -> io::Result<()> {
    println!("{}", "✨ Rust Notes App ✨".bright_magenta().bold());
    println!("{}", "─".repeat(40).bright_black());

    loop {
        println!("\n{}", "Available commands:".bright_cyan().bold());
        println!("  {} - Add new note", "1".bright_green());
        println!("  {} - List all notes", "2".bright_yellow());
        println!("  {} - View note details", "3".bright_blue());
        println!("  {} - Search notes", "4".bright_magenta());
        println!("  {} - Update note", "5".bright_cyan());
        println!("  {} - Delete note", "6".bright_red());
        println!("  {} - Start web server", "7".bright_green().bold());
        println!("  {} - Exit", "8".bright_red().bold());

        let choice = get_input(&format!("\n{} ", "Enter your choice:".bright_white()));

        match choice.as_str() {
            "1" => add_note(&manager),
            "2" => list_notes(&manager),
            "3" => view_note(&manager),
            "4" => search_notes(&manager),
            "5" => update_note(&manager),
            "6" => delete_note(&manager),
            "7" => {
                println!("{}", "🌐 Starting web server...".bright_green().bold());
                return Ok(());
            }
            "8" => {
                println!("{}", "👋 Goodbye!".bright_magenta().bold());
                std::process::exit(0);
            }
            _ => {
                println!(
                    "{}",
                    "❌ Invalid choice! Please enter a number between 1 and 8.".bright_red()
                );
            }
        }
    }
}

fn add_note(manager: &SharedNotesManager) {
    display_header("ADD NEW NOTE");
    let title = get_input(&format!("{} ", "Title:".bright_green()));
    if title.is_empty() {
        println!("{}", "⚠ Title cannot be empty!".bright_red());
        return;
    }

    println!(
        "{}",
        "Content (type 'END' on a new line to finish):".bright_yellow()
    );
    let mut content_lines = Vec::new();

    loop {
        let line = get_input("");
        if line == "END" {
            break;
        }
        content_lines.push(line);
    }

    let content = content_lines.join("\n");
    let tags_input = get_input("Enter tags (comma-separated, press Enter to skip): ");
    let tags: Vec<String> = if tags_input.is_empty() {
        Vec::new()
    } else {
        tags_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let mut mgr = manager.lock().unwrap();
    match mgr.add_note(title, content, tags) {
        Ok(note) => {
            println!(
                "{} {}",
                "✅ Note added successfully! ID:".bright_green(),
                note.id.bright_cyan()
            );
        }
        Err(e) => {
            println!("{} {}", "❌ Error:".bright_red(), e);
        }
    }
}

fn list_notes(manager: &SharedNotesManager) {
    display_header("ALL NOTES");
    let mgr = manager.lock().unwrap();
    let notes = mgr.list_notes();

    if notes.is_empty() {
        println!("{}", "📭 No notes found.".bright_yellow());
    } else {
        println!(
            "{} {}",
            "📝 Total notes:".bright_blue(),
            notes.len().to_string().bright_cyan()
        );
        for (i, note) in notes.iter().enumerate() {
            let truncated_content = if note.content.len() > 50 {
                format!("{}...", &note.content[..47])
            } else {
                note.content.clone()
            };

            println!(
                "{} {} {}",
                format!("[{:2}]", i + 1).bright_white().bold(),
                note.title.bold().green(),
                format!("({})", truncated_content).dimmed()
            );

            if !note.tags.is_empty() {
                println!(
                    "     {}",
                    note.tags
                        .iter()
                        .map(|t| format!("[{}]", t).bright_magenta().to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
        }
    }
}

fn view_note(manager: &SharedNotesManager) {
    display_header("VIEW NOTE");
    let index_input = get_input(&format!("{} ", "Enter note number to view:".bright_white()));
    if let Ok(index) = index_input.parse::<usize>() {
        let mgr = manager.lock().unwrap();
        if index > 0 && index <= mgr.notes_count() {
            if let Some(note) = mgr.get_note_by_index(index - 1) {
                println!("{}", "─".repeat(60).bright_black());
                println!("{}: {}", "ID".bright_cyan().bold(), note.id);
                println!("{}: {}", "Title".bright_green().bold(), note.title);
                println!("{}:\n{}", "Content".bright_white(), note.content);
                if !note.tags.is_empty() {
                    println!(
                        "{}: {}",
                        "Tags".bright_yellow().bold(),
                        note.tags
                            .iter()
                            .map(|tag| format!("#{}", tag).bright_magenta().to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    );
                }
                println!("{}: {}", "Created".bright_blue(), note.created_at);
                println!("{}: {}", "Updated".bright_blue(), note.updated_at);
                println!("{}", "─".repeat(60).bright_black());
            }
        } else {
            println!("{}", "❌ Invalid note number!".bright_red());
        }
    } else {
        println!("{}", "❌ Please enter a valid number!".bright_red());
    }
}

fn search_notes(manager: &SharedNotesManager) {
    display_header("SEARCH NOTES");
    let query = get_input(&format!("{} ", "Enter search query:".bright_white()));
    if !query.is_empty() {
        let mgr = manager.lock().unwrap();
        let results = mgr.search_notes(&query);
        if results.is_empty() {
            println!(
                "{} '{}'",
                "🔍 No notes found matching".bright_yellow(),
                query.bright_white()
            );
        } else {
            println!(
                "{} {} {}",
                "🔍 Found".bright_green(),
                results.len().to_string().bright_cyan(),
                "notes:".bright_green()
            );
            for (i, note) in results.iter().enumerate() {
                println!(
                    "{} {} {}",
                    format!("[{:2}]", i + 1).bright_white().bold(),
                    note.title.bold().green(),
                    format!("({} chars)", note.content.len()).dimmed()
                );
            }
        }
    }
}

fn update_note(manager: &SharedNotesManager) {
    display_header("UPDATE NOTE");
    let index_input = get_input(&format!(
        "{} ",
        "Enter note number to update:".bright_white()
    ));
    if let Ok(index) = index_input.parse::<usize>() {
        let mut mgr = manager.lock().unwrap();
        if index > 0 && index <= mgr.notes_count() {
            println!(
                "{}",
                "ℹ Leave field blank to keep current value.".bright_blue()
            );

            let current_note = &mgr.notes[index - 1];
            let new_title = get_input(&format!(
                "{} [{}]: ",
                "Title".bright_green(),
                current_note.title
            ));
            let title = if new_title.is_empty() {
                None
            } else {
                Some(new_title)
            };

            println!(
                "{}",
                "Content (type 'END' on new line to finish, 'KEEP' to keep current):"
                    .bright_yellow()
            );
            println!("{}", "Current content:".bright_blue());
            println!("{}", current_note.content);

            let mut new_content_lines = Vec::new();
            loop {
                let line = get_input("");
                if line == "END" {
                    break;
                } else if line == "KEEP" {
                    new_content_lines.clear();
                    break;
                }
                new_content_lines.push(line);
            }

            let content = if !new_content_lines.is_empty() {
                Some(new_content_lines.join("\n"))
            } else {
                None
            };

            let new_tags_input = get_input(&format!(
                "{} [{}]: ",
                "Tags".bright_magenta(),
                current_note.tags.join(", ")
            ));
            let tags = if new_tags_input.is_empty() {
                None
            } else {
                Some(
                    new_tags_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                )
            };

            let id = mgr.notes[index - 1].id.clone();
            match mgr.update_note(&id, title, content, tags) {
                Ok(Some(_)) => println!("{}", "✅ Note updated successfully!".bright_green()),
                Ok(None) => println!("{}", "❌ Note not found!".bright_red()),
                Err(e) => println!("{} {}", "❌ Error:".bright_red(), e),
            }
        } else {
            println!("{}", "❌ Invalid note number!".bright_red());
        }
    }
}

fn delete_note(manager: &SharedNotesManager) {
    display_header("DELETE NOTE");
    let index_input = get_input(&format!(
        "{} ",
        "Enter note number to delete:".bright_white()
    ));
    if let Ok(index) = index_input.parse::<usize>() {
        let mut mgr = manager.lock().unwrap();
        if index > 0 && index <= mgr.notes_count() {
            match mgr.delete_note_by_index(index - 1) {
                Ok(_) => println!("{}", "✅ Note deleted successfully!".bright_green()),
                Err(e) => println!("{} {}", "❌ Error:".bright_red(), e),
            }
        } else {
            println!("{}", "❌ Invalid note number!".bright_red());
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let storage_path = "data/notes.json";
    let manager = Arc::new(Mutex::new(NotesManager::new(storage_path)?));

    // Check command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "web" {
        println!(
            "{}",
            "🌐 Starting Rust Notes Web Server...".bright_green().bold()
        );
        web::start_web_server(manager).await;
    } else {
        // CLI mode
        cli_mode(manager.clone())?;

        // After CLI mode, ask if user wants to start web server
        println!("\n{}", "─".repeat(60).bright_blue());
        println!(
            "{}",
            "Would you like to start the web interface? (y/n)"
                .bright_cyan()
                .bold()
        );
        let response = get_input("> ").to_lowercase();

        if response == "y" || response == "yes" {
            println!("{}", "🌐 Starting web server...".bright_green().bold());
            web::start_web_server(manager).await;
        }
    }

    Ok(())
}
