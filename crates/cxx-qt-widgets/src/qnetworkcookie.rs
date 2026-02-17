use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        include!("cxx-qt-lib/qstring.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-widgets/qnetworkcookie.h");

        /// Represents an HTTP cookie.
        type QNetworkCookie = super::QNetworkCookie;

        /// Returns whether the cookie has the Secure attribute.
        #[cxx_name = "isSecure"]
        fn is_secure(self: &QNetworkCookie) -> bool;

        /// Sets whether the cookie has the Secure attribute.
        #[cxx_name = "setSecure"]
        fn set_secure(self: Pin<&mut QNetworkCookie>, enable: bool);

        /// Returns whether the cookie has the HttpOnly attribute.
        #[cxx_name = "isHttpOnly"]
        fn is_http_only(self: &QNetworkCookie) -> bool;

        /// Sets whether the cookie has the HttpOnly attribute.
        #[cxx_name = "setHttpOnly"]
        fn set_http_only(self: Pin<&mut QNetworkCookie>, enable: bool);

        /// Returns whether the cookie is a session cookie.
        #[cxx_name = "isSessionCookie"]
        fn is_session_cookie(self: &QNetworkCookie) -> bool;

        /// Returns the cookie domain.
        fn domain(self: &QNetworkCookie) -> QString;

        /// Sets the cookie domain.
        #[cxx_name = "setDomain"]
        fn set_domain(self: Pin<&mut QNetworkCookie>, domain: &QString);

        /// Returns the cookie path.
        fn path(self: &QNetworkCookie) -> QString;

        /// Sets the cookie path.
        #[cxx_name = "setPath"]
        fn set_path(self: Pin<&mut QNetworkCookie>, path: &QString);

        /// Returns the cookie name.
        fn name(self: &QNetworkCookie) -> QByteArray;

        /// Sets the cookie name.
        #[cxx_name = "setName"]
        fn set_name(self: Pin<&mut QNetworkCookie>, name: &QByteArray);

        /// Returns the cookie value.
        fn value(self: &QNetworkCookie) -> QByteArray;

        /// Sets the cookie value.
        #[cxx_name = "setValue"]
        fn set_value(self: Pin<&mut QNetworkCookie>, value: &QByteArray);

        /// Returns whether this cookie has the same identifier as another cookie.
        #[cxx_name = "hasSameIdentifier"]
        fn has_same_identifier(self: &QNetworkCookie, other: &QNetworkCookie) -> bool;
    }
}
#[derive(Clone, Debug)]
#[repr(C)]
pub struct QNetworkCookie {
    _a1: MaybeUninit<usize>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkCookie {
    type Id = type_id!("QNetworkCookie");
    type Kind = cxx::kind::Trivial;
}
