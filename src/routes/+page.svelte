<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { register, unregister } from "@tauri-apps/plugin-global-shortcut";

  let isRecording = $state(false);
  let recordingStatus = $state("");
  let error = $state("");
  let lastKeyPress = $state(0);
  let isInCooldown = $state(false);

  onMount(async () => {
    try {
      isRecording = await invoke("ffmpeg_status");
      if (isRecording) {
        recordingStatus = "Recording...";
      }

      // Register global shortcut for Control+Shift+R with debouncing
      await register("Control+Shift+R", () => {
        // Prevent multiple triggers within 500ms
        const now = Date.now();
        if (now - lastKeyPress < 500) return;
        lastKeyPress = now;

        // Prevent starting a new recording during cooldown
        if (isInCooldown) return;

        // Use a non-async function for the toggle to prevent hold-down behavior
        if (isRecording) {
          stopRecording();
        } else {
          startRecording();
        }
      });
    } catch (e) {
      error = `Error: ${e}`;
    }
  });

  onDestroy(async () => {
    try {
      await unregister("Control+Shift+R");
    } catch (e) {
      console.error("Failed to unregister shortcut:", e);
    }
  });

  async function startRecording() {
    try {
      error = "";
      await invoke("start_ffmpeg");
      isRecording = true;
      recordingStatus = "Recording...";
    } catch (e) {
      error = `Failed to start recording: ${e}`;
    }
  }

  async function stopRecording() {
    try {
      error = "";
      await invoke("stop_ffmpeg");
      isRecording = false;
      recordingStatus = "Recording stopped";
      
      // Enter cooldown period after stopping
      isInCooldown = true;
      setTimeout(() => {
        isInCooldown = false;
      }, 1000); // 1 second cooldown
    } catch (e) {
      error = `Failed to stop recording: ${e}`;
      isRecording = true;
      recordingStatus = "Recording...";
    }
  }
</script>

<main class="container">
  <h1>Screen Recorder</h1>

  <div class="controls">
    <button 
      class="record-btn" 
      onclick={startRecording} 
      disabled={isRecording}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
        <circle cx="12" cy="12" r="6"></circle>
      </svg>
      Record
    </button>

    <button 
      class="stop-btn" 
      onclick={stopRecording} 
      disabled={!isRecording}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
        <rect x="7" y="7" width="10" height="10"></rect>
      </svg>
      Stop
    </button>
  </div>

  {#if recordingStatus}
    <p class="status">{recordingStatus}</p>
  {/if}
  
  {#if error}
    <p class="error">{error}</p>
  {/if}
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #ffffff;
  background-color: #5B798E;
  
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  height: 100vh;
}

h1 {
  text-align: center;
  color: white;
  margin-bottom: 2rem;
}

.controls {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.8em 1.5em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: white;
  cursor: pointer;
  transition: all 0.25s;
}

button svg {
  width: 20px;
  height: 20px;
}

.record-btn {
  background-color: #e53935;
}

.record-btn:hover {
  background-color: #c62828;
}

.stop-btn {
  background-color: #282828;
}

.stop-btn:hover {
  background-color: #212121;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.status {
  font-size: 1rem;
  color: white;
  margin-top: 1rem;
}

.error {
  font-size: 1rem;
  color: #ff9e9e;
  margin-top: 1rem;
  background-color: rgba(220, 53, 69, 0.2);
  padding: 0.5rem 1rem;
  border-radius: 4px;
  max-width: 80%;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #ffffff;
    background-color: #2F4452;
  }
}
</style>
