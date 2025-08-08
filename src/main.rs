use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{OpenOptions, File, create_dir_all};
use std::io::Write;
use std::sync::Mutex;
use actix_multipart::{Multipart, Field};
use futures_util::stream::StreamExt;
use chrono::Utc;
use actix_files::Files;
use std::env;

#[derive(Serialize, Deserialize, Clone)]
struct UploadMetadata {
    filename: String,
    user: String,
    timestamp: String,
}

struct AppState {
    uploads: Mutex<Vec<UploadMetadata>>,
}



async fn upload_file(
    mut payload: Multipart,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut filename = String::new();
    let mut total_size = 0u64;
    const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB limit

    while let Some(item) = payload.next().await {
        let mut field: Field = match item {
            Ok(field) => field,
            Err(e) => return HttpResponse::BadRequest().body(format!("Failed to read multipart field: {}", e)),
        };

        // Handle content disposition (returns &ContentDisposition)
        let content_disposition = field.content_disposition();
        if let Some(name) = content_disposition.get_filename() {
            // Sanitize filename: remove path separators and special chars
            let sanitized_name = name
                .replace('/', "_")
                .replace('\\', "_")
                .replace("..", "_")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
                .collect::<String>();
            
            if sanitized_name.is_empty() {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid filename",
                    "message": "Filename contains only invalid characters"
                }));
            }
            
            filename = sanitized_name;
            let filepath = format!("uploads/{}", filename);
            
            // Ensure uploads directory exists (including for uploads.json)
            if let Err(e) = create_dir_all("uploads") {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to create uploads directory",
                    "message": e.to_string()
                }));
            }
            
            let mut file = match File::create(&filepath) {
                Ok(file) => file,
                Err(e) => return HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to create file",
                    "message": e.to_string()
                })),
            };
            
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => return HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to read chunk",
                        "message": e.to_string()
                    })),
                };
                
                total_size += data.len() as u64;
                if total_size > MAX_FILE_SIZE {
                    return HttpResponse::PayloadTooLarge().json(json!({
                        "error": "File too large",
                        "message": format!("File size exceeds {}MB limit", MAX_FILE_SIZE / 1024 / 1024)
                    }));
                }
                
                if let Err(e) = file.write_all(&data) {
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to write file",
                        "message": e.to_string()
                    }));
                }
            }
        } else {
            return HttpResponse::BadRequest().json(json!({
                "error": "Missing filename",
                "message": "No filename provided in upload"
            }));
        }
    }

    if filename.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "No file uploaded",
            "message": "No file was provided in the request"
        }));
    }

    let metadata = UploadMetadata {
        filename,
        user: "anonymous".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    let mut uploads = data.uploads.lock().unwrap();
    uploads.push(metadata.clone());
    let file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("uploads/uploads.json") {
            Ok(file) => file,
            Err(e) => return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to open uploads.json",
                "message": e.to_string()
            })),
        };
    if let Err(e) = serde_json::to_writer(file, &*uploads) {
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to write to uploads.json",
            "message": e.to_string()
        }));
    }

    HttpResponse::Ok().json(json!({ "status": "success", "filename": metadata.filename }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uploads = web::Data::new(AppState {
        uploads: Mutex::new(vec![]),
    });

    if let Ok(file) = File::open("uploads/uploads.json") {
        if let Ok(data) = serde_json::from_reader(file) {
            *uploads.uploads.lock().unwrap() = data;
        }
    }

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    println!("Starting server on {}", bind_addr);
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(uploads.clone())
            .route("/upload", web::post().to(upload_file))
            .service(Files::new("/", "frontend").index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await
}