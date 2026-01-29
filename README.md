Judith# 📝 Rust Notes App

A beautiful, full-featured notes application built in Rust with both command-line and web interfaces. Features colorful terminal output, persistent JSON storage, and a modern web interface.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Actix-web](https://img.shields.io/badge/Actix--web-4.0-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

## ✨ Features

### 🎨 **CLI Interface**
- Colorful terminal output using `colored` crate
- Interactive menu system
- Real-time search across notes
- Tag management and organization

### 🌐 **Web Interface**
- Modern REST API built with Actix-web
- Responsive HTML/CSS/JavaScript frontend
- Full CRUD operations (Create, Read, Update, Delete)
- Real-time search and filtering

### 💾 **Storage**
- Automatic JSON persistence
- Notes saved to `data/notes.json`
- UUID-based note identification
- Timestamps (created/updated)

## 🚀 Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/) 1.70 or higher
- Cargo (comes with Rust)

### Installation
```bash
# Clone the repository
git clone https://github.com/coderjudith/notes_app.git
cd rust-notes-app

# Build the project
cargo build

# Or run directly
cargo run
or
cargo run -- web
