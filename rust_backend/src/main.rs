use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;

mod webcam; // Ensure you have a `webcam.rs` file in the `src` directory.

// Example API route
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World from Rust!")
}

// Use the `find_available_webcams` function from the `webcam` module.
async fn get_webcams() -> impl Responder {
    let webcams = webcam::find_available_webcams(); // Call the function from the module
    HttpResponse::Ok().json(webcams)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // Enable CORS for API endpoints
            .wrap(Cors::default().allow_any_origin())
            // API routes
            .route("/api/hello", web::get().to(hello_world))
            .route("/api/webcams", web::get().to(get_webcams))
            // Serve the Svelte `dist` folder
            .service(Files::new("/", "../svelte_frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
