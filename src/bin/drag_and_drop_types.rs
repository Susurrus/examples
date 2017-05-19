//! https://python-gtk-3-tutorial.readthedocs.io/en/latest/drag_and_drop.html
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

use std::cell::RefCell;

use gtk::prelude::*;

static TARGET_ENTRY_TEXT: u32 = 0;
static TARGET_ENTRY_PIXBUF: u32 = 1;
static COLUMN_TEXT: i32 = 1;
static COLUMN_PIXBUF: i32 = 1;

// declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<(gtk::Label, gtk::IconView)>> = RefCell::new(None)
);

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 6);
    vbox.pack_start(&hbox, true, true, 0);

    let icon_view = gtk::IconView::new();
    icon_view.set_text_column(COLUMN_TEXT);
    icon_view.set_pixbuf_column(COLUMN_PIXBUF);
    hbox.pack_start(&icon_view, true, true, 0);

    let label = gtk::Label::new("Drop something on me!");
    label.drag_dest_set(gtk::DEST_DEFAULT_ALL,
                        &[],
                        gdk::ACTION_COPY);
    label.connect_drag_data_received(|_, _, _, _, data, info, _| {
        println!("drag-data-received");
        if info == TARGET_ENTRY_TEXT {
            let text = data.get_text().unwrap();
            println!("Received text '{}'", text);
        } else if info == TARGET_ENTRY_PIXBUF {
            let pixbuf = data.get_pixbuf().unwrap();
            let width = pixbuf.get_width();
            let height = pixbuf.get_height();
            println!("Received pixbuf with width {}px and height {}px",
                     width,
                     height);
        }
    });
    hbox.pack_start(&label, true, true, 0);

    let model = gtk::ListStore::new(&[gtk::Type::String,
                                      gdk_pixbuf::Pixbuf::static_type()]);
    //let pixbuf = gtk::IconTheme.get_default().load_icon("image-missing", 16, 0);
    //model.append(["Item 1", pixbuf]);
    //let pixbuf = gtk::IconTheme.get_default().load_icon("help-about", 16, 0);
    //model.append(["Item 2", pixbuf]);
    //let pixbuf = gtk::IconTheme.get_default().load_icon("edit-copy", 16, 0);
    //model.append(["Item 3", pixbuf]);
    //icon_view.set_model(Some(&model));
    //icon_view.enable_model_drag_source(gdk::BUTTON1_MASK,
    //                                   &[],
    //                                   gdk::ACTION_MOVE);
    icon_view.connect_drag_data_get(|w, _, data, info, _| {
        let items = w.get_selected_items();
        let ref selected_path = items[0];
        let model = w.get_model().unwrap();
        let selected_iter = model.get_iter(&selected_path).unwrap();
        if info == TARGET_ENTRY_TEXT {
            let text = model
                           .get_value(&selected_iter, COLUMN_TEXT)
                           .get::<String>()
                           .unwrap();
            data.set_text(&text, text.len() as i32);
        } else if info == TARGET_ENTRY_PIXBUF {
            let pixbuf = model
                             .get_value(&selected_iter, COLUMN_TEXT)
                             .get::<gdk_pixbuf::Pixbuf>()
                             .unwrap();
            data.set_pixbuf(&pixbuf);
        }
    });

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    vbox.pack_start(&button_box, true, false, 0);

    let image_button = gtk::RadioButton::new_with_label_from_widget(None, "Images");
    button_box.pack_start(&image_button, true, false, 0);
    image_button.connect_toggled(|_| {
        println!("image_button - toggled");
        let targets = gtk::TargetList::new(&[]);
        targets.add_image_targets(TARGET_ENTRY_PIXBUF, true);
        GLOBAL.with(|global| {
            if let Some((ref label, ref icon_view)) = *global.borrow() {
                label.drag_dest_set_target_list(Some(&targets));
                icon_view.drag_source_set_target_list(Some(&targets));
            }
        });
    });

    let text_button = gtk::RadioButton::new_with_label_from_widget(Some(&image_button), "Text");
    button_box.pack_start(&text_button, true, false, 0);
    text_button.connect_toggled(|_| {
        println!("text_button - toggled");
        GLOBAL.with(|global| {
            if let Some((ref label, ref icon_view)) = *global.borrow() {
                label.drag_dest_set_target_list(None);
                icon_view.drag_source_set_target_list(None);
                label.drag_dest_add_text_targets();
                icon_view.drag_source_add_text_targets();
            }
        });
    });

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Drag and Drop Example with Multiple Drop Types");
    window.add(&vbox);
    window.show_all();

    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((label, icon_view))
    });

    // GTK & main window boilerplate
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
