use std::pin::Pin;

use crate::WidgetPtr;

use cxx_qt::QObject;
pub use ffi::QTimer;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtCore/QTimer>);

        /// Timer that emits `timeout()` at a specified interval.
        #[qobject]
        type QTimer;

        /// Starts the timer with the given interval in milliseconds.
        #[cxx_name = "start"]
        fn start_with_interval(self: Pin<&mut QTimer>, msec: i32);

        /// Starts the timer using the current interval.
        fn start(self: Pin<&mut QTimer>);

        /// Stops the timer.
        fn stop(self: Pin<&mut QTimer>);

        /// Returns whether the timer is active.
        #[cxx_name = "isActive"]
        fn is_active(self: &QTimer) -> bool;

        /// Sets the interval in milliseconds.
        #[cxx_name = "setInterval"]
        fn set_interval(self: Pin<&mut QTimer>, msec: i32);

        /// Returns the interval in milliseconds.
        fn interval(self: &QTimer) -> i32;

        /// Sets whether the timer is single-shot.
        #[cxx_name = "setSingleShot"]
        fn set_single_shot(self: Pin<&mut QTimer>, single_shot: bool);

        /// Returns whether the timer is single-shot.
        #[cxx_name = "isSingleShot"]
        fn is_single_shot(self: &QTimer) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_timer() -> UniquePtr<QTimer>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_timer_with_parent(parent: *mut QObject) -> *mut QTimer;
    }
}

impl ffi::QTimer {
    /// Creates a new timer without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_timer().into()
    }

    /// Creates a new timer with a parent object.
    pub fn new_with_parent(parent: Pin<&mut QObject>) -> WidgetPtr<Self> {
        unsafe { ffi::new_timer_with_parent(parent.get_unchecked_mut()).into() }
    }
}
