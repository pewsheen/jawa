pub use ffi::QPixmap;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!(<QtGui/QPixmap>);
        type QImage = crate::QImage;
        type QPixmap;

        #[cxx_name = "isNull"]
        fn is_null(self: &QPixmap) -> bool;

        fn width(self: &QPixmap) -> i32;
        fn height(self: &QPixmap) -> i32;

        // TODO: create header
        // #[cxx_name = "fromImage"]
        // fn from_image(image: &QImage) -> QPixmap;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_pixmap() -> UniquePtr<QPixmap>;
    }
}

impl ffi::QPixmap {
    /// Creates an empty pixmap.
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::new_pixmap()
    }
}
