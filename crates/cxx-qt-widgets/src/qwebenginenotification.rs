pub use ffi::QWebEngineNotification;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qurl.h");
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-widgets/qimage.h");
        type QImage = crate::QImage;

    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qwebenginenotification.h");

        /// Notification object emitted by the web engine.
        #[qobject]
        type QWebEngineNotification;

        /// Shows the notification.
        fn show(self: &QWebEngineNotification);

        /// Closes the notification.
        fn close(self: &QWebEngineNotification);

        /// Returns the notification title.
        fn title(self: &QWebEngineNotification) -> QString;

        /// Returns the notification message.
        fn message(self: &QWebEngineNotification) -> QString;

        /// Returns the origin of the notification.
        fn origin(self: &QWebEngineNotification) -> QUrl;

        /// Returns the notification icon.
        fn icon(self: &QWebEngineNotification) -> QImage;

        /// Emitted when the notification is closed.
        #[qsignal]
        fn closed(self: Pin<&mut QWebEngineNotification>);
    }

    impl UniquePtr<QWebEngineNotification> {}
}

unsafe impl Send for QWebEngineNotification {}
unsafe impl Sync for QWebEngineNotification {}
