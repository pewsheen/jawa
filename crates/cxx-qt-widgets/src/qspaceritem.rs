use cxx::UniquePtr;

pub use ffi::QSpacerItem;

use crate::WidgetPtr;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtWidgets/QSpacerItem>);
        include!("cxx-qt-widgets/qsizepolicy.h");
        type Policy = crate::Policy;
        type QLayoutItem = crate::QLayoutItem;

        /// A layout spacer item with a size policy.
        #[qobject]
        #[base = QLayoutItem]
        type QSpacerItem;

        /// Changes the spacer's size and size policies.
        #[cxx_name = "changeSize"]
        fn change_size(
            self: Pin<&mut QSpacerItem>,
            width: i32,
            height: i32,
            horizontal_policy: Policy,
            vertical_policy: Policy,
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_spacer_item(
            width: i32,
            height: i32,
            horizontal_policy: Policy,
            vertical_policy: Policy,
        ) -> UniquePtr<QSpacerItem>;
    }
}

impl ffi::QSpacerItem {
    /// Creates a new spacer item.
    pub fn new(
        width: i32,
        height: i32,
        horizontal_policy: crate::Policy,
        vertical_policy: crate::Policy,
    ) -> WidgetPtr<Self> {
        ffi::new_spacer_item(width, height, horizontal_policy, vertical_policy).into()
    }
}
