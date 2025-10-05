// Définition des actions pour les raccourcis clavier
relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(pub OpenFileAction, WindowActionGroup, "open-file");
relm4::new_stateless_action!(pub OpenFolderAction, WindowActionGroup, "open-folder");
relm4::new_stateless_action!(pub ConvertAction, WindowActionGroup, "convert");
