// STT using whisper.cpp
// Assume whisper_cpp_rs crate

use whisper_cpp::Whisper;

pub struct SttEngine {
    whisper: Whisper,
}

pub fn init_stt(config: &super::SttConfig) -> Result<SttEngine, String> {
    let whisper = Whisper::new(&config.model).map_err(|e| e.to_string())?;
    Ok(SttEngine { whisper })
}

// Add transcribe method
