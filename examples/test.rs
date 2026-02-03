use std::pin::Pin;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::QUrl;
use cxx_qt_lib_extras::QApplication;

use jawa::{QMainWindow, QWebEngineView, QWidget};

fn main() {
    let mut app = QApplication::new();
    let mut window = QMainWindow::new();

    {
        let widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
        let mut view = QWebEngineView::new_with_parent(widget);
        view.pin_mut()
            .load(&QUrl::from("https://html5test.teamdev.com"));
        window.pin_mut().set_central_widget(&mut view);
    }

    let mut widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
    widget.as_mut().resize(800, 600);
    widget.as_mut().show();

    app.pin_mut().exec();
}
