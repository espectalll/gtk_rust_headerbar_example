extern crate gtk;

use gtk::{HeaderBar, Window};
use gtk::traits::*;
use gtk::signal::*;

fn main() {
    if gtk::init().is_err() {
        println!("バカ!!! >_<");
    }
    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(600, 400);
    
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_title("GTK is awesome");
    header_bar.set_subtitle("The title is telling the truth!");
    header_bar.set_show_close_button(true);
    window.set_titlebar(&header_bar);
    
    let button_a = gtk::Button::new_from_icon_name("gtk-refresh", 0);
    let button_b = gtk::Button::new_from_icon_name("gtk-copy", 0);
    button_a.set_always_show_image(true);
    button_b.set_always_show_image(true);
    header_bar.add(&button_a);
    header_bar.add(&button_b);
    
    button_a.connect_clicked(|_| {
        let builder = Builder::new_from_file("headerbars.glade").unwrap();
        let new_header_bar: HeaderBar = builder.get_object("extraHeader1").unwrap();
        window.set_titlebar(&new_header_bar);
    });
    button_b.connect_clicked(|_| {
        println!("Heya! ;3");
    });
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    window.show_all();
    gtk::main();
}
