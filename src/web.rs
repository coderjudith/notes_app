use crate::storage::SharedNotesManager;
use actix_cors::Cors;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateNoteRequest {
    title: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateNoteRequest {
    title: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

// Special implementation for error case (returns ApiResponse<()>)
impl ApiResponse<()> {
    fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

#[get("/api/notes")]
async fn get_notes(manager: web::Data<SharedNotesManager>) -> impl Responder {
    let mgr = manager.lock().unwrap();
    let notes = mgr.list_notes();
    HttpResponse::Ok().json(ApiResponse::success(notes, "Notes retrieved successfully"))
}

#[get("/api/notes/{id}")]
async fn get_note(id: web::Path<String>, manager: web::Data<SharedNotesManager>) -> impl Responder {
    let mgr = manager.lock().unwrap();

    match mgr.get_note(&id) {
        Some(note) => {
            HttpResponse::Ok().json(ApiResponse::success(note, "Note retrieved successfully"))
        }
        None => HttpResponse::NotFound().json(ApiResponse::error("Note not found")),
    }
}

#[post("/api/notes")]
async fn create_note(
    req: web::Json<CreateNoteRequest>,
    manager: web::Data<SharedNotesManager>,
) -> impl Responder {
    let mut mgr = manager.lock().unwrap();

    match mgr.add_note(req.title.clone(), req.content.clone(), req.tags.clone()) {
        Ok(note) => {
            HttpResponse::Created().json(ApiResponse::success(note, "Note created successfully"))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(&format!("Failed to create note: {}", e))),
    }
}

#[put("/api/notes/{id}")]
async fn update_note(
    id: web::Path<String>,
    req: web::Json<UpdateNoteRequest>,
    manager: web::Data<SharedNotesManager>,
) -> impl Responder {
    let mut mgr = manager.lock().unwrap();

    match mgr.update_note(
        &id,
        req.title.clone(),
        req.content.clone(),
        req.tags.clone(),
    ) {
        Ok(Some(note)) => {
            HttpResponse::Ok().json(ApiResponse::success(note, "Note updated successfully"))
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::error("Note not found")),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(&format!("Failed to update note: {}", e))),
    }
}

#[delete("/api/notes/{id}")]
async fn delete_note(
    id: web::Path<String>,
    manager: web::Data<SharedNotesManager>,
) -> impl Responder {
    let mut mgr = manager.lock().unwrap();

    match mgr.delete_note(&id) {
        Ok(true) => HttpResponse::Ok().json(ApiResponse::success((), "Note deleted successfully")),
        Ok(false) => HttpResponse::NotFound().json(ApiResponse::error("Note not found")),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(&format!("Failed to delete note: {}", e))),
    }
}

#[get("/api/notes/search/{query}")]
async fn search_notes(
    query: web::Path<String>,
    manager: web::Data<SharedNotesManager>,
) -> impl Responder {
    let mgr = manager.lock().unwrap();
    let notes = mgr.search_notes(&query);

    HttpResponse::Ok().json(ApiResponse::success(notes, "Search results"))
}

#[get("/")]
async fn index() -> impl Responder {
    let html_content = r#"<h1>Notes App</h1><p>API is running. Use /api/notes endpoints.</p>"#;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("OK", "Server is running"))
}

pub async fn start_web_server(manager: SharedNotesManager) {
    println!("🌐 Web server starting on http://localhost:8080");
    println!("📱 Access at http://localhost:8080");
    println!("📚 API at http://localhost:8080/api/*");
    println!("{}", "─".repeat(60));

    let manager_data = web::Data::new(manager);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(manager_data.clone())
            .service(index)
            .service(get_notes)
            .service(get_note)
            .service(create_note)
            .service(update_note)
            .service(delete_note)
            .service(search_notes)
            .service(health_check)
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
    .await
    .expect("Failed to run server");
}
