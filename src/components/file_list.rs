use gtk::prelude::*;
use gtk::{ListBox, SelectionMode, ScrolledWindow};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use std::path::PathBuf;

use super::audio_file_row::AudioFileRow;

/// Messages gérés par le composant FileList
#[derive(Debug)]
pub enum FileListMsg {
    AddFile(PathBuf),
    AddFiles(Vec<PathBuf>),
}

/// Sortie du composant FileList vers le composant parent
#[derive(Debug)]
pub enum FileListOutput {
    FilesChanged(usize),
}

/// Composant gérant la liste des fichiers audio
pub struct FileList {
    audio_files: FactoryVecDeque<AudioFileRow>,
}

#[relm4::component(pub)]
impl SimpleComponent for FileList {
    type Init = ();
    type Input = FileListMsg;
    type Output = FileListOutput;

    view! {
        #[root]
        ScrolledWindow {
            set_vexpand: true,
            set_hexpand: true,

            #[local_ref]
            file_list -> ListBox {
                set_margin_all: 16,
                set_selection_mode: SelectionMode::None,
                add_css_class: "boxed-list",
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let audio_files = FactoryVecDeque::builder()
            .launch(ListBox::default())
            .detach();

        let model = FileList { audio_files };

        let file_list = model.audio_files.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            FileListMsg::AddFile(path) => {
                eprintln!("Ajout du fichier : {}", path.display());
                self.audio_files.guard().push_back(path);
                let _ = sender.output(FileListOutput::FilesChanged(self.audio_files.len()));
            }
            FileListMsg::AddFiles(paths) => {
                eprintln!("Ajout de {} fichiers", paths.len());
                let mut guard = self.audio_files.guard();
                for path in paths {
                    guard.push_back(path);
                }
                drop(guard);
                let _ = sender.output(FileListOutput::FilesChanged(self.audio_files.len()));
            }
        }
    }
}
