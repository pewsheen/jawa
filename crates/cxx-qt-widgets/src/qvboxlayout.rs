use std::pin::Pin;

use crate::{QWidget, WidgetPtr};

pub use ffi::QVBoxLayout;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtWidgets/QVBoxLayout>);
        type QWidget = crate::QWidget;
        type QBoxLayout = crate::QBoxLayout;

        /// Vertical box layout for arranging child widgets in a column.
        #[qobject]
        #[base = QBoxLayout]
        type QVBoxLayout;

        /// Sets the spacing between items in the layout.
        #[cxx_name = "setSpacing"]
        fn set_spacing(self: Pin<&mut QVBoxLayout>, spacing: i32);

        /// Sets the contents margins of the layout.
        #[cxx_name = "setContentsMargins"]
        fn set_contents_margins(
            self: Pin<&mut QVBoxLayout>,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_vbox_layout() -> UniquePtr<QVBoxLayout>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_vbox_layout_with_parent(parent: *mut QWidget) -> *mut QVBoxLayout;
    }
}

impl ffi::QVBoxLayout {
    /// Creates a new layout without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_vbox_layout().into()
    }

    /// Creates a new layout with a parent widget.
    pub fn new_with_parent(parent: Pin<&mut QWidget>) -> WidgetPtr<Self> {
        unsafe { ffi::new_vbox_layout_with_parent(parent.get_unchecked_mut()).into() }
    }
}
