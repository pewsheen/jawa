use cxx::UniquePtr;

pub use ffi::QNetworkCookie;

#[cxx_qt::bridge]
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
        type QNetworkCookie;

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

    unsafe extern "C++" {
        include!("cxx-qt-widgets/qnetworkcookie.h");

        #[doc(hidden)]
        #[cxx_name = "qnetworkcookieNew"]
        fn new_qnetworkcookie() -> UniquePtr<QNetworkCookie>;

        #[doc(hidden)]
        #[cxx_name = "qnetworkcookieNewWithNameValue"]
        fn new_qnetworkcookie_with_name_value(
            name: &QByteArray,
            value: &QByteArray,
        ) -> UniquePtr<QNetworkCookie>;
    }
}

impl ffi::QNetworkCookie {
    /// Creates a new empty cookie.
    pub fn new() -> UniquePtr<Self> {
        ffi::new_qnetworkcookie()
    }

    /// Creates a new cookie with a name and value.
    pub fn new_with_name_value(
        name: &cxx_qt_lib::QByteArray,
        value: &cxx_qt_lib::QByteArray,
    ) -> UniquePtr<Self> {
        ffi::new_qnetworkcookie_with_name_value(name, value)
    }
}
