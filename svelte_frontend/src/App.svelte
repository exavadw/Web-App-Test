<script>
  import "./app.css";

  let mode = "text-to-speech"; // Default mode
  let recordingTime = 0; // Tracks the recorded time
  let isRecording = false; // State to track if recording is ongoing
  let isDarkMode = true; // Default to dark mode
  document.documentElement.setAttribute("data-theme", "dark");
  let recordedAudioBlob = null; // Holds the recorded audio file
  let transcription = ""; // Stores the transcription result

  let mediaRecorder; // MediaRecorder instance
  let audioChunks = []; // Array to store recorded audio data

  // Toggle the mode
  const toggleTheme = () => {
      isDarkMode = !isDarkMode;
      const theme = isDarkMode ? "dark" : "light";
      document.documentElement.setAttribute("data-theme", theme);
  };
  const convertToSpeech = async () => {
      const text = document.querySelector("textarea").value;

      try {
          const response = await fetch("http://127.0.0.1:8080/api/tts", {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ text }),
          });

          if (response.ok) {
              const audioBlob = await response.blob();
              const audioUrl = URL.createObjectURL(audioBlob);
              const audio = new Audio(audioUrl);
              audio.play();
          } else {
              alert("Error converting text to speech!");
          }
      } catch (error) {
          console.error("Error:", error);
      }
  };
  /*
  const startRecording = async () => {
      if (!isRecording) {
          isRecording = true;
          recordingTime = 0;
          audioChunks = []; // Clear previous recordings

          try {
              const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
              mediaRecorder = new MediaRecorder(stream);

              mediaRecorder.ondataavailable = (event) => {
                  audioChunks.push(event.data);
              };

              mediaRecorder.onstop = () => {
                  recordedAudioBlob = new Blob(audioChunks, { type: "audio/wav" });
                  console.log("Recording stopped, audio blob created");
              };

              mediaRecorder.start();
              console.log("Recording started");

              // Update recording time
              const interval = setInterval(() => {
                  if (recordingTime >= 15 || !isRecording) {
                      clearInterval(interval);
                      stopRecording();
                  } else {
                      recordingTime++;
                  }
              }, 1000);
          } catch (error) {
              console.error("Error accessing microphone:", error);
          }
      }
  };

  const stopRecording = () => {
      if (isRecording && mediaRecorder) {
          isRecording = false;
          mediaRecorder.stop();
      }
  };

  const uploadAudioForTranscription = async () => {
      if (!recordedAudioBlob) {
          alert("No audio recorded!");
          return;
      }

      const formData = new FormData();
      formData.append("file", recordedAudioBlob, "audio.wav");

      try {
          const response = await fetch("http://127.0.0.1:8080/api/stt", {
              method: "POST",
              body: formData,
          });

          if (response.ok) {
              const data = await response.json();
              transcription = data.transcription;
              console.log("Transcription:", transcription);
          } else {
              console.error("Error during transcription:", response.statusText);
          }
      } catch (error) {
          console.error("Error:", error);
      }
  };*/
</script>

<main>
  <header>
    <button on:click={toggleTheme}>
        Switch to {isDarkMode ? "Light" : "Dark"} Mode
    </button>
</header>
  <h1>Text-to-Speech and Speech-to-Text</h1>
  
  <div class="radio-buttons">
      <label>
          <input type="radio" name="mode" value="text-to-speech" bind:group={mode} />
          Text-to-Speech
      </label>
      <label>
          <input type="radio" name="mode" value="speech-to-text" bind:group={mode} />
          Speech-to-Text
      </label>
  </div>

  {#if mode === "text-to-speech"}
      <div class="text-to-speech">
        <button on:click={convertToSpeech}>🔊 Convert to Speech</button>
          <textarea placeholder="Enter text to convert to speech"></textarea>
      </div>
  {/if}

  {#if mode === "speech-to-text"}
      <div class="speech-to-text">
          <button>
              {isRecording ? "Stop" : "🎤 Start Recording"}
          </button>
          <button disabled={!recordedAudioBlob}>
              Upload for Transcription
          </button>
          <p>{transcription ? `Transcription: ${transcription}` : "Transcription will appear here."}</p>
          {#if isRecording}
              <p>{recordingTime}/15 seconds recorded</p>
          {/if}
      </div>
  {/if}
</main>