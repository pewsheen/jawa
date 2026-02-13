mod notification_popup;

use std::pin::Pin;

use crate::notification_popup::NotificationPopup;
use cxx_qt::Threading;
use cxx_qt_widgets::{
    PermissionType, QApplication, QUrl, QWebEngineProfile,
    QWebEngineView, QWidget, casting::Upcast,
};

fn main() {
    let mut app = QApplication::new();

    let mut view = QWebEngineView::new();
    view.page().pin_mut()
        .on_permission_requested(|_page, permission| {
            if permission.permission_type() != PermissionType::Notifications {
                println!("Unsupported permission type requested: {:?}", permission);
                return;
            }
            println!("Permission requested: {:?}", permission);
            permission.grant();
        })
        .release();

    let mut profile = view.page().profile();
    let mut profile: Pin<&mut QWebEngineProfile> = profile.pin_mut();
    view.pin_mut().load(&QUrl::from("http://localhost:5001"));
    let mut widget: Pin<&mut QWidget> = view.pin_mut().upcast_pin();
    let popup = NotificationPopup::new(widget.as_mut());
    let this = popup.qt_thread();

    profile.as_mut().set_push_service_enabled(true);
    profile
        .as_mut()
        .set_notification_presenter(move |notification| {
            let _ = this.queue(|p| p.present(notification));
        });

    widget.as_mut().resize(800, 600);
    widget.as_mut().show();

    app.pin_mut().exec();
}
