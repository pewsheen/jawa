use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-widgets/qimage.h");
        type QImage = super::QImage;

        #[cxx_name = "isNull"]
        fn is_null(self: &QImage) -> bool;

        fn width(self: &QImage) -> i32;
        fn height(self: &QImage) -> i32;
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct QImage {
    _a1: MaybeUninit<usize>,
    _a2: MaybeUninit<usize>,
    _a3: MaybeUninit<usize>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QImage {
    type Id = type_id!("QImage");
    type Kind = cxx::kind::Trivial;
}
