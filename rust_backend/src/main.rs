use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::env;
use std::path::Path;
use piper_rs::synth::PiperSpeechSynthesizer;

#[derive(Deserialize)]
struct TtsRequest {
    text: String,
}

async fn text_to_speech(request: web::Json<TtsRequest>) -> impl Responder {
    let config_path = "jenny.onnx.json"; // Path to your Piper config file
    let output_path = "output.wav"; // Output file path

    // Initialize Piper model
    let model = match piper_rs::from_config_path(Path::new(&config_path)) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Error loading model: {}", err);
            return HttpResponse::InternalServerError().body("Failed to load TTS model");
        }
    };

    // Create synthesizer
    let synth = match PiperSpeechSynthesizer::new(model) {
        Ok(synth) => synth,
        Err(err) => {
            eprintln!("Error initializing synthesizer: {}", err);
            return HttpResponse::InternalServerError().body("Failed to initialize TTS synthesizer");
        }
    };

    // Perform synthesis
    if let Err(err) = synth.synthesize_to_file(Path::new(&output_path), request.text.clone(), None) {
        eprintln!("Error during synthesis: {}", err);
        return HttpResponse::InternalServerError().body("Failed to generate speech");
    }

    // Read the generated audio file
    let mut audio_data = Vec::new();
    match File::open(output_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_end(&mut audio_data) {
                eprintln!("Error reading audio file: {}", err);
                return HttpResponse::InternalServerError().body("Failed to read generated audio");
            }
        }
        Err(err) => {
            eprintln!("Error opening audio file: {}", err);
            return HttpResponse::InternalServerError().body("Failed to open generated audio file");
        }
    }

    // Return the audio data as the response
    HttpResponse::Ok()
        .content_type("audio/wav")
        .body(audio_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Current working directory: {}", env::current_dir().unwrap().display());
    let svelte_dist_path = "/Users/henrybar-or/Documents/Web App Test/svelte_frontend/dist";
    // Resolve the absolute path
    let absolute_path = fs::canonicalize(svelte_dist_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(svelte_dist_path));
    println!("Serving static files from: {}", absolute_path.display());
    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // Enable CORS for API endpoints
            .wrap(Cors::default()
            .allow_any_origin() // Allow requests from any origin
            .allow_any_method() // Allow any HTTP method
            .allow_any_header() // Allow any header
        )
            // API routes
            //.route("/api/stt", web::post().to(speech_to_text))
            .route("/api/tts", web::post().to(text_to_speech))
            // Serve the Svelte `dist` folder
            .service(Files::new("/", absolute_path.clone()).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
