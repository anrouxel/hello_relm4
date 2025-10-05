pub mod audio_file_row;
pub mod file_list;
pub mod header;

// Ré-export des composants publics
pub use file_list::{FileList, FileListMsg, FileListOutput};
pub use header::{Header, HeaderOutput};

// AudioFileRow est public mais n'est pas ré-exporté car il est interne à file_list
