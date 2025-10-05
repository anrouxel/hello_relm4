use adw::prelude::*;
use adw::ActionRow;
use gtk::ListBox;
use relm4::prelude::*;
use std::path::PathBuf;

/// Messages gérés par le composant AudioFileRow
#[derive(Debug)]
pub enum AudioFileRowMsg {
    Activate,
}

/// Composant représentant une ligne de fichier audio dans la liste
#[derive(Debug, Clone)]
pub struct AudioFileRow {
    path: PathBuf,
}

#[relm4::factory(pub)]
impl FactoryComponent for AudioFileRow {
    type Init = PathBuf;
    type Input = AudioFileRowMsg;
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
                sender.input(AudioFileRowMsg::Activate);
            }
        }
    }

    fn init_model(path: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        AudioFileRow { path }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            AudioFileRowMsg::Activate => {
                eprintln!("Ligne activée : {}", self.path.display());
            }
        }
    }
}
