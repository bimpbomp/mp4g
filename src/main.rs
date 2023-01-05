use std::path::Path;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Video, gio};

const APP_ID: &str = "me.tobymorgan.mp4g";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let path = Path::new("shirt-vid.mp4");
    let file = gio::File::for_path(path);
    println!("File exists: {}", path.exists());

    let video = Video::builder()
        .file(&file)
        .width_request(500)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&video)
        .build();

    // Present window
    window.present();
}
