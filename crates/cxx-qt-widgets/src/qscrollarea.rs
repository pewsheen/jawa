use std::pin::Pin;

use crate::{QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

pub use ffi::QScrollArea;

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        type ScrollBarPolicy = crate::ScrollBarPolicy;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qscrollarea.h");
        type QWidget = crate::QWidget;

        /// A scrolling view onto another widget.
        #[qobject]
        #[base = QWidget]
        type QScrollArea;

        /// Sets the widget displayed in the scroll area.
        #[cxx_name = "setWidget"]
        unsafe fn set_widget_raw(self: Pin<&mut QScrollArea>, widget: *mut QWidget);

        /// Returns the currently displayed widget.
        fn widget(self: &QScrollArea) -> *mut QWidget;

        /// Controls whether the scroll area resizes the child widget.
        #[cxx_name = "setWidgetResizable"]
        fn set_widget_resizable(self: Pin<&mut QScrollArea>, resizable: bool);

        #[cxx_name = "setHorizontalScrollBarPolicy"]
        fn set_horizontal_scroll_bar_policy(self: Pin<&mut QScrollArea>, policy: ScrollBarPolicy);

        #[cxx_name = "setVerticalScrollBarPolicy"]
        fn set_vertical_scroll_bar_policy(self: Pin<&mut QScrollArea>, policy: ScrollBarPolicy);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_scroll_area() -> UniquePtr<QScrollArea>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_scroll_area_with_parent(parent: *mut QWidget) -> *mut QScrollArea;
    }
}

impl QScrollArea {
    /// Creates a new scroll area without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_scroll_area().into()
    }

    /// Creates a new scroll area with a parent.
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe { ffi::new_scroll_area_with_parent(parent.upcast_pin().get_unchecked_mut()).into() }
    }

    /// Sets the widget displayed in the scroll area, transferring ownership.
    pub fn set_widget<T: Upcast<QWidget> + UniquePtrTarget>(
        self: Pin<&mut QScrollArea>,
        widget: &mut WidgetPtr<T>,
    ) {
        widget.release();
        unsafe { self.set_widget_raw(widget.pin_mut().upcast_pin().get_unchecked_mut()) }
    }

    /// Returns the currently displayed widget if one has been set.
    pub fn child_widget(&self) -> Option<WidgetPtr<QWidget>> {
        let widget = self.widget();
        if widget.is_null() {
            None
        } else {
            Some(widget.into())
        }
    }
}
