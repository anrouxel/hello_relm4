use std::fs;
use std::path::PathBuf;

/// Vérifie si une extension correspond à un fichier audio supporté
pub fn is_audio_extension(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "opus"
    )
}

/// Scanne un dossier et retourne tous les fichiers audio trouvés
pub fn scan_audio_files(folder: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut audio_files = Vec::new();
    
    let entries = fs::read_dir(folder)?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_audio_extension(ext) {
                    audio_files.push(path);
                }
            }
        }
    }
    
    Ok(audio_files)
}
