use tauri::{command, AppHandle, State};
use crate::AppState;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Sender, Receiver};

// Structs for API responses
#[derive(Serialize)]
struct AppInfo {
    version: String,
    // Add more
}

#[derive(Serialize)]
struct ModelInfo {
    kind: String,
    path: String,
    // Size, etc.
}

#[command]
pub fn init_app(state: State<AppState>) -> Result<AppInfo, String> {
    Ok(AppInfo { version: "1.0.0".to_string() })
}

#[command]
pub fn list_models(kind: String, state: State<AppState>) -> Result<Vec<ModelInfo>, String> {
    // Implement listing models from models/ folder
    Ok(vec![])
}

#[command]
pub fn set_model(kind: String, path: String, state: State<AppState>) -> Result<(), String> {
    // Update config and reload engine
    Ok(())
}

#[command]
pub async fn generate(prompt: String, chat_id: String, settings: Option<serde_json::Value>, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // Use llm to generate, stream to UI via emit
    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(32);

    // Spawn task for generation
    tauri::async_runtime::spawn(async move {
        // Generate with llm
        // For each chunk, tx.send(chunk)
    });

    // Receive and emit
    while let Some(chunk) = rx.recv().await {
        app.emit_all("text_chunk", chunk).unwrap();
    }

    Ok(())
}

#[command]
pub async fn speak(text: String, voice_id: String, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // Similar, stream progress
    Ok(())
}

#[command]
pub async fn transcribe(action: String, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // start or stop
    Ok(())
}

#[command]
pub fn save_message(chat_id: String, role: String, text: String, state: State<AppState>) -> Result<(), String> {
    // Use storage
    Ok(())
}

#[command]
pub fn search_kb(query: String, state: State<AppState>) -> Result<Vec<Snippet>, String> {
    // Use rag
    Ok(vec![])
}

#[command]
pub fn toggle_offline(mode: bool, state: State<AppState>) -> Result<(), String> {
    // Always offline, perhaps noop
    Ok(())
}

// Define Snippet struct etc.
#[derive(Serialize)]
struct Snippet {
    content: String,
    score: f32,
}
