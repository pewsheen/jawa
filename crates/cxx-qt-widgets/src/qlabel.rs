use crate::WidgetPtr;

pub use ffi::QLabel;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qlabel.h");
        type QWidget = crate::QWidget;
        type QPixmap = crate::QPixmap;

        /// Display text or an image.
        #[qobject]
        #[base = QWidget]
        type QLabel;

        /// Set the text shown on the label.
        #[cxx_name = "setText"]
        fn set_text(self: Pin<&mut QLabel>, text: &QString);

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QLabel>);

        #[cxx_name = "setPixmap"]
        fn set_pixmap(self: Pin<&mut QLabel>, pixmap: &QPixmap);

        fn height(self: &QLabel) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_label() -> UniquePtr<QLabel>;
    }
}

impl ffi::QLabel {
    /// Creates a new label without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_label().into()
    }
}
