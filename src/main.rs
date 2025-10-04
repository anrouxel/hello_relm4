use gtk::prelude::*;
use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use gtk::{Box as GtkBox, Orientation, Button, Label};
use relm4::RelmWidgetExt;

use std::rc::Rc;
use std::cell::RefCell;

fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Compteur simple")
        .default_width(300)
        .default_height(100)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    let inc_button = Button::with_label("Increment");
    let dec_button = Button::with_label("Decrement");

    let label = Label::new(Some("Counter: 0"));
    label.set_margin_all(5);

    vbox.append(&inc_button);
    vbox.append(&dec_button);
    vbox.append(&label);

    window.set_content(Some(&vbox));

    let counter = Rc::new(RefCell::new(0_u8));

    // Bouton increment
    {
        let label_clone = label.clone();
        let counter_clone = Rc::clone(&counter);
        inc_button.connect_clicked(move |_| {
            // emprunt mutable unique
            {
                let mut cnt = counter_clone.borrow_mut();
                *cnt = cnt.wrapping_add(1);
                // on peut lire immédiatement depuis cnt
                let val = *cnt;
                // on relâche le borrow explictement
                drop(cnt);
                // maintenant on peut utiliser label
                label_clone.set_label(&format!("Counter: {}", val));
            }
        });
    }

    // Bouton decrement
    {
        let label_clone = label.clone();
        let counter_clone = Rc::clone(&counter);
        dec_button.connect_clicked(move |_| {
            {
                let mut cnt = counter_clone.borrow_mut();
                *cnt = cnt.wrapping_sub(1);
                let val = *cnt;
                drop(cnt);
                label_clone.set_label(&format!("Counter: {}", val));
            }
        });
    }

    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.simple_adw")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
