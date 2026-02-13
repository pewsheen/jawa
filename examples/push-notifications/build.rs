use cxx_qt_build::CxxQtBuilder;

fn main() {
    let _ = CxxQtBuilder::new()
        .qt_module("Widgets")
        .qt_module("WebEngineCore")
        .qrc("data/data.qrc")
        .file("src/main.rs")
        .file("src/notification_popup.rs")
        .build();
}
