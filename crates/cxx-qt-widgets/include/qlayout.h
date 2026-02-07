#pragma once

#include <QLayout>
#include <QSize>
#include <QWidget>

#include <memory>

#include "rust/cxx.h"

class RustQLayout : public QLayout {
 public:
  explicit RustQLayout(QWidget* parent = nullptr) : QLayout(parent) {}

  void addItem(QLayoutItem* /*item*/) override {}

  QSize sizeHint() const override { return QSize(0, 0); }

  QLayoutItem* itemAt(int /*index*/) const override { return nullptr; }

  QLayoutItem* takeAt(int /*index*/) override { return nullptr; }

  int count() const override { return 0; }
};

namespace rust::cxxqtlib1 {
inline std::unique_ptr<QLayout> new_layout() {
  return std::make_unique<RustQLayout>(nullptr);
}

inline QLayout* new_layout_with_parent(QWidget* parent) {
  return new RustQLayout(parent);
}
} // namespace rust::cxxqtlib1
