// Audio I/O using miniaudio

use miniaudio::{Device, DeviceConfig, Format, Channels};

pub fn play_audio(buffer: Vec<u8>) -> Result<(), String> {
    // Configure and play
    Ok(())
}

pub fn capture_audio() -> Result<Vec<u8>, String> {
    // Capture
    Ok(vec![])
}
