//! Simple drag and drop example
//!
//! Ported over from example code:
//! https://developer.gnome.org/gtkmm-tutorial/stable/sec-dnd-example.html.en

extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Configure button as drag source for text
    let button = gtk::Button::new_with_label("Drag here");
    let targets = vec![gtk::TargetEntry::new("STRING", gtk::TargetFlags::empty(), 0),
                       gtk::TargetEntry::new("text/plain", gtk::TargetFlags::empty(), 0)];
    button.drag_source_set(gdk::MODIFIER_MASK, &targets, gdk::ACTION_COPY);
    button.connect_drag_data_get(|_, _, s, _, _| {
        let data = "I'm data!";
        s.set_text(data, data.len() as i32);
    });

    // Configure label as drag destination to receive text
    let label = gtk::Label::new("Drop here");
    label.drag_dest_set(gtk::DEST_DEFAULT_ALL,
                        &targets,
                        gdk::ACTION_COPY);
    label.connect_drag_data_received(|w, _, _, _, s, _, _| {
        w.set_text(&s.get_text().unwrap());
    });

    // Stack the button and label horizontally
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&button, true, true, 0);
    hbox.pack_start(&label, true, true, 0);

    // Finish populating the window and display everything
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.add(&hbox);
    window.show_all();

    // GTK & main window boilerplate
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
