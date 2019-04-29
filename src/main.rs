mod toolbar;

extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Adjustment,
    Application, 
    ApplicationWindow, 
    ContainerExt, 
    GtkWindowExt, 
    Image, 
    ImageExt,
    Scale, 
    ScaleExt,
    ToolButtonExt, 
    WidgetExt
};
use gtk::Orientation::{
    Horizontal,
    Vertical
};
use std::env;
use toolbar::MusicToolbar;

struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let cover = Image::new();
        cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App { 
            adjustment,
            cover,
            toolbar, 
            window 
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
