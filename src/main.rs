use adw::prelude::*;
use relm4::prelude::*;
use relm4::gtk;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::factory::FactoryVecDeque;
use adw::{ActionRow, AboutDialog, ApplicationWindow};
use gtk::{gio, FileDialog, FileFilter, ListBox, SelectionMode, Orientation};
use std::fs;
use std::path::PathBuf;

// Actions pour les raccourcis clavier
relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(OpenFileAction, WindowActionGroup, "open-file");
relm4::new_stateless_action!(OpenFolderAction, WindowActionGroup, "open-folder");
relm4::new_stateless_action!(ConvertAction, WindowActionGroup, "convert");

fn is_audio_extension(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "opus"
    )
}

#[derive(Debug, Clone)]
struct AudioFile {
    path: PathBuf,
}

#[derive(Debug)]
enum AudioFileMsg {
    Activate,
}

#[relm4::factory]
impl FactoryComponent for AudioFile {
    type Init = PathBuf;
    type Input = AudioFileMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = ListBox;

    view! {
        #[root]
        ActionRow {
            set_activatable: true,
            #[watch]
            set_title: &self.path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| self.path.display().to_string()),
            
            connect_activated[sender] => move |_| {
                sender.input(AudioFileMsg::Activate);
            }
        }
    }

    fn init_model(path: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        AudioFile { path }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            AudioFileMsg::Activate => {
                eprintln!("Ligne activée : {}", self.path.display());
            }
        }
    }
}

#[derive(Debug)]
enum AppMsg {
    OpenFile,
    OpenFolder,
    Convert,
    Preferences,
    About,
    AddFile(PathBuf),
    AddFolder(PathBuf),
}

struct App {
    audio_files: FactoryVecDeque<AudioFile>,
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

                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_label: "Convertir",
                        set_tooltip_text: Some("Convertir (Ctrl+R)"),
                        connect_clicked => AppMsg::Convert,
                    },

                    pack_start = &gtk::Button {
                        set_icon_name: "document-open-symbolic",
                        set_tooltip_text: Some("Choisir un fichier audio (Ctrl+O)"),
                        connect_clicked => AppMsg::OpenFile,
                    },

                    pack_start = &gtk::Button {
                        set_icon_name: "folder-symbolic",
                        set_tooltip_text: Some("Choisir un dossier (Ctrl+F)"),
                        connect_clicked => AppMsg::OpenFolder,
                    },

                    pack_end = &gtk::Button {
                        set_icon_name: "help-about-symbolic",
                        set_tooltip_text: Some("À propos"),
                        connect_clicked => AppMsg::About,
                    },

                    pack_end = &gtk::Button {
                        set_icon_name: "preferences-system-symbolic",
                        set_tooltip_text: Some("Préférences"),
                        connect_clicked => AppMsg::Preferences,
                    },
                },

                gtk::ScrolledWindow {
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
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let audio_files = FactoryVecDeque::builder()
            .launch(ListBox::default())
            .detach();

        let window = root.clone();
        let model = App { audio_files, window };

        let file_list = model.audio_files.widget();
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
                let sender_clone = _sender.clone();
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
                                sender_clone.input(AppMsg::AddFile(path));
                            }
                        }
                    },
                );
            }
            AppMsg::OpenFolder => {
                let sender_clone = _sender.clone();
                let dialog = FileDialog::new();
                dialog.set_title("Choisir un dossier");
                
                let window: gtk::Window = self.window.clone().upcast();
                dialog.select_folder(
                    Some(&window),
                    gio::Cancellable::NONE,
                    move |result| {
                        if let Ok(file) = result {
                            if let Some(path) = file.path() {
                                sender_clone.input(AppMsg::AddFolder(path));
                            }
                        }
                    },
                );
            }
            AppMsg::Convert => {
                eprintln!("Conversion lancée pour {} fichiers", self.audio_files.len());
                for (idx, file) in self.audio_files.iter().enumerate() {
                    eprintln!("  {} - {}", idx + 1, file.path.display());
                }
            }
            AppMsg::Preferences => {
                eprintln!("Préférences cliqué");
            }
            AppMsg::About => {
                let about = AboutDialog::new();
                about.set_application_name("First App");
                about.set_version("0.1");
                about.set_comments("Application de conversion audio — exemple avec Relm4");
                about.set_developers(&["Développeur"]);
                
                about.present(Some(&self.window));
            }
            AppMsg::AddFile(path) => {
                eprintln!("Fichier audio choisi : {}", path.display());
                self.audio_files.guard().push_back(path);
            }
            AppMsg::AddFolder(folder) => {
                eprintln!("Dossier choisi : {}", folder.display());
                if let Ok(entries) = fs::read_dir(&folder) {
                    let mut guard = self.audio_files.guard();
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.is_file() {
                            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                                if is_audio_extension(ext) {
                                    guard.push_back(p);
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("Impossible de lire le dossier: {}", folder.display());
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.example.FirstAdwaitaApp");
    app.run::<App>(());
}
