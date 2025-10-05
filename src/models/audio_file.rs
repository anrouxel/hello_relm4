use std::path::PathBuf;

/// Modèle représentant un fichier audio
#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
}

impl AudioFile {
    /// Crée une nouvelle instance d'AudioFile
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Retourne le nom du fichier
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.path.display().to_string())
    }
}
