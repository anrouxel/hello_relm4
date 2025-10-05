use adw::prelude::*;
use adw::HeaderBar;
use gtk::Button;
use relm4::prelude::*;

/// Messages émis par le composant Header
#[derive(Debug)]
pub enum HeaderMsg {
    OpenFile,
    OpenFolder,
    Convert,
    Preferences,
    About,
}

/// Messages de sortie du composant Header vers le parent
#[derive(Debug)]
pub enum HeaderOutput {
    OpenFile,
    OpenFolder,
    Convert,
    Preferences,
    About,
}

/// Composant représentant la barre d'en-tête avec les boutons d'action
pub struct Header {}

#[relm4::component(pub)]
impl SimpleComponent for Header {
    type Init = ();
    type Input = HeaderMsg;
    type Output = HeaderOutput;

    view! {
        #[root]
        HeaderBar {
            pack_start = &Button {
                set_label: "Convertir",
                set_tooltip_text: Some("Convertir (Ctrl+R)"),
                connect_clicked => HeaderMsg::Convert,
            },

            pack_start = &Button {
                set_icon_name: "document-open-symbolic",
                set_tooltip_text: Some("Choisir un fichier audio (Ctrl+O)"),
                connect_clicked => HeaderMsg::OpenFile,
            },

            pack_start = &Button {
                set_icon_name: "folder-symbolic",
                set_tooltip_text: Some("Choisir un dossier (Ctrl+F)"),
                connect_clicked => HeaderMsg::OpenFolder,
            },

            pack_end = &Button {
                set_icon_name: "help-about-symbolic",
                set_tooltip_text: Some("À propos"),
                connect_clicked => HeaderMsg::About,
            },

            pack_end = &Button {
                set_icon_name: "preferences-system-symbolic",
                set_tooltip_text: Some("Préférences"),
                connect_clicked => HeaderMsg::Preferences,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            HeaderMsg::OpenFile => {
                let _ = sender.output(HeaderOutput::OpenFile);
            }
            HeaderMsg::OpenFolder => {
                let _ = sender.output(HeaderOutput::OpenFolder);
            }
            HeaderMsg::Convert => {
                let _ = sender.output(HeaderOutput::Convert);
            }
            HeaderMsg::Preferences => {
                let _ = sender.output(HeaderOutput::Preferences);
            }
            HeaderMsg::About => {
                let _ = sender.output(HeaderOutput::About);
            }
        }
    }
}
