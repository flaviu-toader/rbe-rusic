mod playlist;
mod toolbar;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::FileChooserExt;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Adjustment, Application, ApplicationWindow, ContainerExt, DialogExt, FileChooserAction,
    FileChooserDialog, FileFilter, FileFilterExt, GtkWindowExt, Image, ImageExt, Scale, ScaleExt,
    ToolButtonExt, WidgetExt,
};
use playlist::Playlist;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use toolbar::MusicToolbar;

struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
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

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let cover = Image::new();
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
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

        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(toolbar::PLAY_STOCK.to_string()) {
                play_button.set_stock_id(toolbar::PAUSE_STOCK);
                toolbar::set_cover(&cover, &playlist);
            } else {
                play_button.set_stock_id(toolbar::PAUSE_STOCK);
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });
    }
}

fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(
        Some("Select an MP3 audio file"),
        Some(parent),
        FileChooserAction::Open,
    );
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", gtk_sys::GTK_RESPONSE_ACCEPT as i32);
    dialog.add_button("Open", gtk_sys::GTK_RESPONSE_CANCEL as i32);
    let result = dialog.run();
    if result == gtk_sys::GTK_RESPONSE_ACCEPT as i32 {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
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
