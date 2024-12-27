use serde::Serialize;
use opencv::prelude::*;
use opencv::videoio;

#[derive(Serialize)]
pub struct Webcam {
    pub id: i32,
    pub name: String,
}

pub fn find_available_webcams() -> Vec<Webcam> {
    let mut available_cameras = Vec::new();

    for device_id in 0..10 {
        if let Ok(mut camera) = videoio::VideoCapture::new(device_id, videoio::CAP_ANY) {
            if camera.is_opened().unwrap_or(false) {
                let name = camera.get_backend_name().unwrap_or_else(|_| "Unknown".to_string());
                available_cameras.push(Webcam {
                    id: device_id,
                    name: format!("Camera {}: {}", device_id, name),
                });
            }
        }
    }

    available_cameras
}
