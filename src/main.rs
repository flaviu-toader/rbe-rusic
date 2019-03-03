mod toolbar;

extern crate gio;
extern crate gtk;


use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{Application, ApplicationWindow, GtkWindowExt, ContainerExt, ToolButtonExt, WidgetExt};
use std::env;
use toolbar::*;

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let toolbar = MusicToolbar::new();
        window.add(toolbar.toolbar());
        window.show_all();

        let app = App {
            toolbar,
            window,
        };

        app.connect_events();

        app
    }

    fn connect_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(toolbar::PLAY_STOCK.to_string()) {
                play_button.set_stock_id(toolbar::PAUSE_STOCK);
            } else {
                play_button.set_stock_id(toolbar::PAUSE_STOCK);
            }
        });
    }
}


fn main() {
    let application = Application::new("com.github.rbe-rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(move |application| {
        App::new(application);
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
