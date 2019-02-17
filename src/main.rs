extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};
use std::env;

fn main() {
    let application = Application::new("com.github.rbe-rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");
        window.show();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
