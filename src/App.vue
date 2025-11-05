<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";

const isRecording = ref(false);

onMounted(async () => {
  isRecording.value = await invoke("keyboard_status");
});

async function toggleRecording() {
  if (isRecording.value) { isRecording.value = await invoke("stop_record"); return; }
  isRecording.value = await invoke("start_record");
}
</script>

<template>
  <main class="container">
    <div class="row">
      <button class="round-button" :class="{ 'is-recording': isRecording }" @click="toggleRecording">
        {{ isRecording ? "Stop" : "Start" }}
      </button>
      <button class="round-button" @click="invoke('playback')" :disabled="isRecording">
        Play
      </button>
    </div>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

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
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
  gap: 20px; /* Added spacing between buttons */
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  outline: none;
  cursor: pointer;
}

.round-button {
  border-radius: 50%;
  padding: 1.5em;
  aspect-ratio: 1 / 1;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 1.2em;
  font-weight: bold;
  background-color: #4CAF50; /* Green */
  color: white;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  transition: all 0.3s ease;
}

.round-button:hover {
  background-color: #66bb6a; /* Lighter Green */
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.3);
  transform: translateY(-2px);
}

.round-button:active,
.round-button.is-recording {
  background-color: #f44336; /* Red */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  transform: translateY(0);
}

.round-button.is-recording:hover {
  background-color: #ef5350; /* Lighter Red */
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  .round-button {
    background-color: #2e7d32; /* Darker Green */
  }

  .round-button:hover {
    background-color: #4caf50; /* Green */
  }

  .round-button:active,
  .round-button.is-recording {
    background-color: #c62828; /* Darker Red */
  }
}

</style>