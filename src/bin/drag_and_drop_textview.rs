//! More complex drag and drop example
//!
//! Displays the text of a file in a TextView widget after it has been dragged
//! and dropped onto it.
//! http://www.kksou.com/php-gtk2/sample-codes/display-a-text-file-in-GtkTextView-with-drag-and-drop.php
extern crate gdk;
extern crate gtk;
extern crate url;

use std::fs::File;
use std::io::Read;

use gtk::prelude::*;
use url::Url;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Give a nice text description for the user
    let label = gtk::Label::new("Drag text file onto the TextView below.");

    // Set up a scrollable text view as our drag target
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(None, None);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);
    let targets = vec![gtk::TargetEntry::new("text/uri-list", gtk::TargetFlags::empty(), 0)];
    text_view.drag_dest_set(gtk::DEST_DEFAULT_ALL, &targets, gdk::ACTION_COPY);
    text_view.connect_drag_data_received(|w, _, _, _, d, _, _| {
        // Populate the text view with the contents of the file if only one was received
        if d.get_uris().len() == 1 {
            let filename = d.get_uris()[0].split("\n").nth(0).unwrap().to_string();
            let file_path = Url::parse(&filename).unwrap().to_file_path().unwrap();
            let mut file = File::open(file_path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            w.get_buffer().unwrap().set_text(&String::from_utf8(buf).unwrap());
        }
        // There's no good way to handle multiple files in this example, so just alert the user
        else {
            let toplevel = w.get_toplevel().unwrap();
            let window = toplevel.downcast::<gtk::Window>();
            let window = window.as_ref().unwrap();
            let s = "Only 1 text file can be dragged onto this widget".to_owned();
            let dialog = gtk::MessageDialog::new(Some(window),
                                                 gtk::DIALOG_DESTROY_WITH_PARENT,
                                                 gtk::MessageType::Error,
                                                 gtk::ButtonsType::Close,
                                                 &s);
            dialog.run();
            dialog.destroy();
        }
    });

    // Pack widgets into the window and display everything
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&scrolled_text_view, true, true, 0);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Drag and Drop Example with a TextView");
    window.add(&vbox);
    window.show_all();

    // GTK & main window boilerplate
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
