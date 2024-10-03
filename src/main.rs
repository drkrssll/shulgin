use gdk4::glib::ExitCode;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Dialog, Entry, Orientation};
use webkit6::prelude::WebViewExt;
use webkit6::WebView;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("com.example.webview")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Google Web Render")
        .default_width(800)
        .default_height(600)
        .build();

    let webview = WebView::new();
    webview.load_uri("https://www.google.com");

    let overlay = gtk4::Overlay::new();
    overlay.set_child(Some(&webview));

    let toggle_button = Button::with_label("Toggle Menu");
    overlay.add_overlay(&toggle_button);
    toggle_button.set_halign(gtk4::Align::End);
    toggle_button.set_valign(gtk4::Align::End);

    let dialog = Dialog::new();
    dialog.set_transient_for(Some(&window));
    dialog.set_title(Some("Enter URL"));

    let content_area = dialog.content_area();
    let dialog_box = Box::new(Orientation::Vertical, 5);
    content_area.append(&dialog_box);

    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some(""));
    dialog_box.append(&url_entry);

    url_entry.connect_activate({
        let dialog = dialog.clone();
        let webview = webview.clone();
        move |entry| {
            let url = entry.text();
            if !url.is_empty() {
                let url_to_load = if url.starts_with("http://") {
                    url.to_string()
                } else {
                    format!("https://{}", url)
                };
                webview.load_uri(&url_to_load);
            }

            entry.set_text("");
            dialog.hide();
        }
    });

    toggle_button.connect_clicked({
        let dialog = dialog.clone();
        move |_| {
            if !dialog.is_visible() {
                dialog.show()
            } else {
                dialog.hide()
            }
        }
    });

    window.set_child(Some(&overlay));
    window.present();
}
