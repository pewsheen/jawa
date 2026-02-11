use std::{pin::Pin, ptr::null_mut};

use cxx_qt_widgets::{
    QBoxLayout, QHBoxLayout, QLabel, QLayout, QVBoxLayout, QWidget, WidgetPtr, WindowFlags,
    WindowType, casting::Upcast, QMouseEvent
};

#[cxx_qt::bridge]
pub mod qobject {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qwidget.h");
        type WindowFlags = cxx_qt_widgets::WindowFlags;

    }

    unsafe extern "C++" {
        include!(<memory>);
        type RustQWidget = cxx_qt_widgets::RustQWidget;
        type QMouseEvent = cxx_qt_widgets::QMouseEvent;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = RustQWidget]
        type NotificationPopup = super::NotificationPopupRust;

        #[cxx_override]
        #[cxx_name = "mouseReleaseEvent"]
        fn mouse_release_event(self: Pin<&mut Self>, event: *mut QMouseEvent);
    }
}


#[derive(Default)]
pub struct NotificationPopupRust;

impl qobject::NotificationPopup {
    pub fn mouse_release_event(self: Pin<&mut Self>, event: *mut QMouseEvent) {
        println!("Mouse released on notification popup!");
    }
}

pub struct NotificationPopup {
    this: WidgetPtr<QHBoxLayout>,
    icon: WidgetPtr<QLabel>,
    title: WidgetPtr<QLabel>,
    message: WidgetPtr<QLabel>,
}

impl NotificationPopup {
    pub fn new() -> Self {
        // let mut this = QWidget::new();// TODO: Fix Qwidget constructor to accept parent and window flags
        // let mut widget: Pin<&mut QWidget> = this.pin_mut().upcast_pin();
        // widget.as_mut().set_window_flags(WindowType::ToolTip.into());

        let mut this = QHBoxLayout::new();
        let mut root_layout: Pin<&mut QBoxLayout> = this.pin_mut().upcast_pin();

        let mut icon = QLabel::new();
        // root_layout.as_mut().add_widget(&mut icon);

        // let mut body_layout = QVBoxLayout::new();
        // root_layout.as_mut().add_layout(&mut body_layout);

        let mut title = QLabel::new();
        let mut message = QLabel::new();

        Self {
            this,
            icon,
            title,
            message,
        }
    }
}
