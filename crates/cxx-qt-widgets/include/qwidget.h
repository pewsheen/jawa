#pragma once

#include <QObject>
#include <QWidget>

#include <memory>

#include "rust/cxx.h"

class RustQWidget : public QWidget {
  Q_OBJECT
  public:
   explicit RustQWidget(QObject* parent = nullptr)
       : QWidget(qobject_cast<QWidget*>(parent)) {}
};

namespace rust::cxxqtlib1 {
inline std::unique_ptr<QWidget> new_widget() {
  return std::make_unique<RustQWidget>(nullptr);
}

inline QWidget* new_widget_with_parent(QWidget* parent) {
  return new RustQWidget(parent);
}
} // namespace rust::cxxqtlib1
