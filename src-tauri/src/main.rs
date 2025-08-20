// Main entry point for the Tauri-based offline desktop assistant
use tauri::{
    command, AppHandle, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result as SqliteResult};
use llama_cpp::{Model, Session};
use piper_tts::Piper;
use whisper_cpp::Whisper;
use miniaudio::Device;

// App state shared across Tauri commands
#[derive(Clone)]
struct AppState {
    llm_session: Arc<Mutex<Session>>,
    tts: Arc<Mutex<Piper>>,
    stt: Arc<Mutex<Option<Whisper>>>,
    db: Arc<Mutex<Connection>>,
    config: Arc<Mutex<Config>>,
}

// Configuration struct for serde
#[derive(Serialize, Deserialize, Clone)]
struct Config {
    llm: LlmConfig,
    tts: TtsConfig,
    stt: SttConfig,
}

#[derive(Serialize, Deserialize, Clone)]
struct LlmConfig {
    path: String,
    context
