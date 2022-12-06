fn main() {
    use gtk::prelude::*;
    let app = adw::Application::builder().build();
    app.connect_activate(|app| {
        adw::ApplicationWindow::builder()
            .default_width(800)
            .default_height(600)
            .application(app)
            .build()
            .present()
    });
    app.run();
}
