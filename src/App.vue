<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

enum KeyboardEvent {
  StartRecord = "StartRecord",
  StopRecord = "StopRecord",
  PlayRecord = "PlayRecord"
}

const isRecording = ref(false);
const isCountingDown = ref(false);
const countdown = ref(5);
const unlistenFunctions = ref<UnlistenFn[]>([]);

let countdownInterval: number | undefined = undefined;

onMounted(async () => {
  isRecording.value = await invoke("keyboard_status");
  await registerListener();
});

onUnmounted(() => {
  unregisterListener();
});

/**
 * 播放錄製內容，包含倒數計時
 */
function playRecord(value: number = 5) {

  if (isRecording.value || isCountingDown.value) return;

  isCountingDown.value = true;
  countdown.value = value;

  countdownInterval = setInterval(() => {
    countdown.value -= 1;
    if (countdown.value <= 0) {
      clearInterval(countdownInterval);
      isCountingDown.value = false;
      invoke("play_record");
    }
  }, 1000);
}

/**
 * 切換錄製狀態
 */
async function toggleRecording() {
  if (isRecording.value) { isRecording.value = await invoke("stop_record"); return; }
  isRecording.value = await invoke("start_record");
}

/**
 * 註冊指令相關事件監聽 (用變數記錄下來)
 */
async function registerListener() {
  unlistenFunctions.value.push(await handleCommandStartRecord());
  unlistenFunctions.value.push(await handleCommandStopRecord());
  unlistenFunctions.value.push(await handleCommandPlayRecord());
}

/**
 * 解除指令相關事件監聽 (執行記錄的unlisten())
 */
function unregisterListener() {
  unlistenFunctions.value.forEach((handle: () => void) => { handle(); });
}

/**
 * 處理tauri傳來的StartRecord訊息 (emit)
 */
async function handleCommandStartRecord() {

  return await listen<string>(KeyboardEvent.StartRecord, (_: any) => {
    toggleRecording();
  });
}

/**
 * 處理tauri傳來的StopRecord訊息 (emit)
 */
async function handleCommandStopRecord() {

  return await listen<string>(KeyboardEvent.StopRecord, (_: any) => {
    toggleRecording();
  });
}

/**
 * 處理tauri傳來的PlayRecord訊息 (emit)
 */
async function handleCommandPlayRecord() {

  return await listen<string>(KeyboardEvent.PlayRecord, (_: any) => {
    playRecord(0);
  });
}

</script>

<template>
  <main class="container">
    <div class="row">
      <button class="round-button" :class="{ 'is-recording': isRecording }" @click="toggleRecording">
        {{ isRecording ? "Stop" : "Start" }}
      </button>
      <button class="round-button" @click="playRecord" :disabled="isRecording || isCountingDown" :class="{ 'is-countdown': isCountingDown }">
        {{ isCountingDown ? countdown : "Play" }}
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
  width: 5em; /* Enforce fixed size */
  height: 5em; /* Enforce fixed size */
  border-radius: 50%;
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
.round-button.is-recording,
.round-button.is-countdown {
  background-color: #f44336; /* Red */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  transform: translateY(0);
}

.round-button.is-recording:hover,
.round-button.is-countdown:hover {
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
  .round-button.is-recording,
  .round-button.is-countdown {
    background-color: #c62828; /* Darker Red */
  }
}

</style>