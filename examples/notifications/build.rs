use cxx_qt_build::CxxQtBuilder;

fn main() {
    let mut builder = CxxQtBuilder::new()
        .qt_module("Widgets")
        .qt_module("WebEngineCore")
        .qrc("data/data.qrc")
        .qrc_resources(["data/index.html"])
        .file("src/main.rs")
        .build();
}
