# STT/TTS

## Overview
STT/TTS is a web application designed to convert speech to text (STT) and text to speech (TTS). This application leverages modern web technologies to provide a seamless user experience for both functionalities.

## Frontend
The frontend of the application is built using the following technologies:
- **Svelte**: A JavaScript framework for building user interfaces.

## Backend
The backend of the application is built using the following technologies:
- **Rust**: A systems programming language for building fast and reliable software.

## Text to Speech (TTS)
For the text to speech functionality, the application uses:
- **piper-rs**: A Rust library for text-to-speech synthesis.

## Speech to Text (STT)
For the speech to text functionality, the application uses:
- **simple-transcribe-rs**: A Rust library that uses OpenAI's Whisper models for transcription.

## Installation
To install and run the application locally, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/Web-App-Test.git
    ```

2. Navigate to the project directory:
    ```bash
    cd Web-App-Test
    ```

3. Install the frontend dependencies:
    ```bash
    cd svelte_frontend
    npm install
    ```

4. Build the frontend files:
    ```bash
    npm run build
    ```

5. Install the backend dependencies:
    ```bash
    cd ../rust_backend
    cargo build
    ```

6. Start the backend application:
    ```bash
    cargo run
    ```

7. Open your web browser and navigate to `http://localhost:8080` to use the application.

## Usage
Once the application is running, you can access it in your web browser at `http://localhost:8080`. Use the provided interface to convert speech to text and text to speech.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

