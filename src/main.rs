extern crate gtk;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::{HeaderBar, Builder};
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("バカ!!! >_<");
    }
    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let w_handle_o = Rc::new(RefCell::new(window));
    let w_handle_1 = w_handle_o.clone();
    let w_handle_2 = w_handle_o.clone();
    let w_handle_3 = w_handle_o.clone();
    w_handle_1.borrow_mut().set_default_size(600, 400);
    
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_title(Some("GTK is awesome"));
    header_bar.set_subtitle(Some("The title is telling the truth!"));
    header_bar.set_show_close_button(true);
    w_handle_1.borrow_mut().set_titlebar(Some(&header_bar));
    
    let button_a = gtk::Button::new_from_icon_name("gtk-refresh", 0);
    let button_b = gtk::Button::new_from_icon_name("gtk-copy", 0);
    button_a.set_always_show_image(true);
    button_b.set_always_show_image(true);
    header_bar.add(&button_a);
    header_bar.add(&button_b);
    
    button_a.connect_clicked(move |_| {
        let builder = Builder::new_from_file("headerbars.glade");
        let new_header: HeaderBar = builder.get_object("extraHeader1").unwrap();
        w_handle_2.borrow_mut().set_titlebar(Some(&new_header));
    });
    button_b.connect_clicked(move |_| {
        let builder = Builder::new_from_file("headerbars.glade");
        let new_header: HeaderBar = builder.get_object("extraHeader2").unwrap();
        w_handle_3.borrow_mut().set_titlebar(Some(&new_header));
    });
    
    w_handle_1.borrow_mut().connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    w_handle_1.borrow_mut().show_all();
    gtk::main();
}
