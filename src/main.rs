use gtk4::glib::ExitCode;
use gtk4::{prelude::*, Align, Entry, Overlay};
use gtk4::{Application, ApplicationWindow};
use webkit6::prelude::WebViewExt;
use webkit6::WebView;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("org.gtk.browse")
        .build();

    app.connect_activate(app_window);

    app.run()
}

fn app_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Google Render")
        .default_width(800)
        .default_height(600)
        .build();

    let webview = WebView::new();
    webview.load_uri("https://www.google.com");

    let overlay = Overlay::new();

    overlay.set_child(Some(&webview));

    let menu = Entry::new();

    menu.set_placeholder_text(Some("Enter URL"));
    menu.set_visible(false);

    overlay.add_overlay(&menu);
    menu.set_halign(Align::Center);
    menu.set_valign(Align::Center);

    let webview_clone = webview.clone();
    menu.connect_activate(move |url| {
        let mut url_text = url.text().to_string();

        if !url_text.starts_with("http://") && !url_text.starts_with("https://") {
            url_text = format!("https://{}", url_text);
        }

        if !url_text.is_empty() {
            webview_clone.load_uri(&url_text);

            url.set_text("");
            url.set_placeholder_text(Some("Enter URL"));
        }
    });

    window.set_child(Some(&overlay));

    window.present();
}
