pub use ffi::QLayoutItem;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qlayoutitem.h");

        /// Base class for layout items.
        type QLayoutItem;
    }
}
