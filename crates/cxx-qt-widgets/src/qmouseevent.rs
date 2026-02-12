pub use ffi::QMouseEvent;

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    extern "C++" {
        include!(<QtGui/QMouseEvent>);
        type MouseButton = crate::MouseButton;
    }

    unsafe extern "C++Qt" {

        type QSinglePointEvent = crate::QSinglePointEvent;

        /// Mouse event with a single pointer.
        #[qobject]
        #[base = QSinglePointEvent]
        type QMouseEvent;

        fn button(self: &QMouseEvent) -> MouseButton;
    }

    impl UniquePtr<QMouseEvent> {}
}
