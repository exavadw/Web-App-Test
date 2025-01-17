use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_cors::Cors;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write, Read};
use std::env;
use std::path::Path;
use piper_rs::synth::PiperSpeechSynthesizer;
use reqwest::Client;
use futures_util::StreamExt;
use ogg::reading::PacketReader;
use hound::{WavWriter, WavSpec, SampleFormat};
use opus::{Decoder, Channels}; // Import Opus Decoder and Channels enum
use simple_transcribe_rs::{model_handler, transcriber};

#[derive(Deserialize)]
struct TtsRequest {
    text: String,
}

#[derive(Deserialize)]
struct SttRequest {
    audio: Vec<u8>,
}

#[derive(Deserialize)]
struct DeepgramResponse {
    results: Vec<DeepgramResult>,
}

#[derive(Deserialize)]
struct DeepgramResult {
    alternatives: Vec<DeepgramAlternative>,
}

#[derive(Deserialize)]
struct DeepgramAlternative {
    transcript: String,
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
    let audio_data = match fs::read(&output_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading audio file: {}", err);
            return HttpResponse::InternalServerError().body("Failed to read audio file");
        }
    };

    // Return the audio data as the response
    HttpResponse::Ok()
        .content_type("audio/wav")
        .body(audio_data)
}

async fn save_audio(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    let file_path = "./speech-to-text.ogg";
    let file = File::create(file_path);
    
    if let Err(err) = file {
        return HttpResponse::InternalServerError().body(format!("Failed to create file: {}", err));
    }

    let mut file = BufWriter::new(file.unwrap());

    // Print the current codec from the Content-Type header
    if let Some(content_type) = req.headers().get("Content-Type") {
        println!("Content-Type: {}", content_type.to_str().unwrap());
    }

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => {
                if let Err(err) = file.write_all(&data) {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to write data to file: {}", err));
                }
            }
            Err(err) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error while reading payload: {}", err));
            }
        }
    }
/*
    // Print the codec used in the OGG file
    match find_ogg_codec(file_path) {
        Ok(codec) => println!("Codec used: {}", codec),
        Err(err) => eprintln!("Failed to find codec: {}", err),
    }*/

    convert_ogg_to_wav(file_path, "./speech-to-text.wav");
    HttpResponse::Ok().body("File saved successfully")
}

fn find_ogg_codec(ogg_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ogg_file = File::open(ogg_path)?;
    let mut reader = PacketReader::new(ogg_file);

    while let Some(packet) = reader.read_packet()? {
        if packet.data.starts_with(b"\x01vorbis") {
            return Ok("Vorbis".to_string());
        } else if packet.data.starts_with(b"OpusHead") {
            return Ok("Opus".to_string());
        }
    }

    Ok("Unknown".to_string())
}

fn convert_ogg_to_wav(ogg_path: &str, wav_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ogg_file = File::open(ogg_path)?;
    let mut reader = PacketReader::new(ogg_file);
    // Initialize Opus decoder
    let cchannels = Channels::Stereo;
    let mut opus_decoder = Decoder::new(48000, cchannels)?;

    // Create WAV writer
    let spec = WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let wav_file = File::create(wav_path)?;
    let mut wav_writer = WavWriter::new(wav_file, spec)?;

    let mut output_buffer = vec![0; 960 * 2]; // Buffer for decoded samples

    while let Some(packet) = reader.read_packet()? {
        if packet.data.starts_with(b"OpusTags") {
            continue;
        }

        if packet.data.starts_with(b"OpusHead") {
            continue;
        }

        let decoded_samples = opus_decoder.decode(&packet.data, &mut output_buffer, false)?;
        for sample in &output_buffer[..decoded_samples * 2] {
            wav_writer.write_sample(*sample)?;
        }
    }

    wav_writer.finalize()?;
    Ok(())
}

async fn speech_to_text() -> impl Responder {
    println!("function run");
    let model = model_handler::ModelHandler::new("base", "models/").await;
    println!("downloading model");
    let transcriber = transcriber::Transcriber::new(model);
    println!("transcriber made");
    let result = transcriber.transcribe("./speech-to-text.wav", None).unwrap();
    println!("transcription complete");
    let text = result.get_text();
    println!("text: {}", text);
    HttpResponse::Ok().json(text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Current working directory: {}", env::current_dir().unwrap().display());
    let svelte_dist_path = Path::new("/home/exavadw/Documents/GitHub/Web-App-Test/svelte_frontend/dist");
    // Resolve the absolute path
    let absolute_path = fs::canonicalize(&svelte_dist_path)
        .unwrap_or_else(|_| svelte_dist_path.to_path_buf());
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
            .route("/api/stt", web::post().to(speech_to_text))
            .route("/api/tts", web::post().to(text_to_speech))
            .route("/api/save-audio", web::post().to(save_audio))
            // Serve the Svelte `dist` folder
            .service(Files::new("/", absolute_path.clone()).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}