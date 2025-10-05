mod actions;
mod components;
mod models;
mod utils;

use adw::prelude::*;
use adw::{AboutDialog, ApplicationWindow};
use gtk::{gio, FileDialog, FileFilter, Orientation};
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::prelude::*;
use std::path::PathBuf;

use actions::{ConvertAction, OpenFileAction, OpenFolderAction, WindowActionGroup};
use components::{FileList, FileListMsg, FileListOutput, Header, HeaderOutput};
use utils::scan_audio_files;

/// Messages gérés par l'application principale
#[derive(Debug)]
enum AppMsg {
    OpenFile,
    OpenFolder,
    Convert,
    Preferences,
    About,
    AddFile(PathBuf),
    AddFolder(PathBuf),
    FileListChanged(usize),
}

/// Structure principale de l'application
struct App {
    header: Controller<Header>,
    file_list: Controller<FileList>,
    window: ApplicationWindow,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_default_width: 700,
            set_default_height: 420,

            gtk::Box {
                set_orientation: Orientation::Vertical,

                #[local_ref]
                header_widget -> adw::HeaderBar {},

                #[local_ref]
                file_list_widget -> gtk::ScrolledWindow {},
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Initialisation du composant Header
        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                HeaderOutput::OpenFile => AppMsg::OpenFile,
                HeaderOutput::OpenFolder => AppMsg::OpenFolder,
                HeaderOutput::Convert => AppMsg::Convert,
                HeaderOutput::Preferences => AppMsg::Preferences,
                HeaderOutput::About => AppMsg::About,
            });

        // Initialisation du composant FileList
        let file_list = FileList::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                FileListOutput::FilesChanged(count) => AppMsg::FileListChanged(count),
            });

        let window = root.clone();
        let model = App {
            header,
            file_list,
            window,
        };

        let header_widget = model.header.widget();
        let file_list_widget = model.file_list.widget();
        let widgets = view_output!();

        // Configuration des actions et raccourcis clavier
        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let sender_clone = sender.clone();
        let open_file_action: RelmAction<OpenFileAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::OpenFile);
        });
        actions.add_action(open_file_action);

        let sender_clone = sender.clone();
        let open_folder_action: RelmAction<OpenFolderAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::OpenFolder);
        });
        actions.add_action(open_folder_action);

        let sender_clone = sender.clone();
        let convert_action: RelmAction<ConvertAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::Convert);
        });
        actions.add_action(convert_action);

        actions.register_for_widget(&widgets.main_window);

        // Configuration des raccourcis clavier
        let app = relm4::main_application();
        app.set_accels_for_action("win.open-file", &["<Ctrl>O"]);
        app.set_accels_for_action("win.open-folder", &["<Ctrl>F"]);
        app.set_accels_for_action("win.convert", &["<Ctrl>R"]);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenFile => {
                self.open_file_dialog(_sender.clone());
            }
            AppMsg::OpenFolder => {
                self.open_folder_dialog(_sender.clone());
            }
            AppMsg::Convert => {
                eprintln!("Conversion lancée");
                // Ici, vous pouvez accéder aux fichiers via self.file_list
                // et implémenter la logique de conversion
            }
            AppMsg::Preferences => {
                eprintln!("Préférences cliqué");
            }
            AppMsg::About => {
                self.show_about_dialog();
            }
            AppMsg::AddFile(path) => {
                eprintln!("Fichier audio choisi : {}", path.display());
                self.file_list.emit(FileListMsg::AddFile(path));
            }
            AppMsg::AddFolder(folder) => {
                eprintln!("Dossier choisi : {}", folder.display());
                match scan_audio_files(&folder) {
                    Ok(files) => {
                        self.file_list.emit(FileListMsg::AddFiles(files));
                    }
                    Err(e) => {
                        eprintln!("Impossible de lire le dossier: {} - {}", folder.display(), e);
                    }
                }
            }
            AppMsg::FileListChanged(count) => {
                eprintln!("Nombre de fichiers dans la liste : {}", count);
            }
        }
    }
}

impl App {
    /// Affiche le dialogue de sélection de fichier
    fn open_file_dialog(&self, sender: ComponentSender<Self>) {
        let dialog = FileDialog::new();
        dialog.set_title("Ouvrir un fichier audio");
        
        let audio_filter = FileFilter::new();
        audio_filter.set_name(Some("Fichiers audio"));
        audio_filter.add_mime_type("audio/*");
        
        let filters = gio::ListStore::new::<FileFilter>();
        filters.append(&audio_filter);
        dialog.set_filters(Some(&filters));
        
        let window: gtk::Window = self.window.clone().upcast();
        dialog.open(
            Some(&window),
            gio::Cancellable::NONE,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        sender.input(AppMsg::AddFile(path));
                    }
                }
            },
        );
    }

    /// Affiche le dialogue de sélection de dossier
    fn open_folder_dialog(&self, sender: ComponentSender<Self>) {
        let dialog = FileDialog::new();
        dialog.set_title("Choisir un dossier");
        
        let window: gtk::Window = self.window.clone().upcast();
        dialog.select_folder(
            Some(&window),
            gio::Cancellable::NONE,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        sender.input(AppMsg::AddFolder(path));
                    }
                }
            },
        );
    }

    /// Affiche le dialogue "À propos"
    fn show_about_dialog(&self) {
        let about = AboutDialog::new();
        about.set_application_name("First App");
        about.set_version("0.1");
        about.set_comments("Application de conversion audio — exemple avec Relm4");
        about.set_developers(&["Développeur"]);
        
        about.present(Some(&self.window));
    }
}

fn main() {
    let app = RelmApp::new("com.example.FirstAdwaitaApp");
    app.run::<App>(());
}
