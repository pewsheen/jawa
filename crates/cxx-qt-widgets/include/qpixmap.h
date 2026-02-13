#pragma once

#include <QPixmap>

#include "rust/cxx.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QPixmap, {
  ::std::size_t a1;
  ::std::size_t a2;
  ::std::size_t a3;
});

inline QPixmap (*qpixmapFromImage)(const QImage&, Qt::ImageConversionFlags) = QPixmap::fromImage;


namespace rust {

template<>
struct IsRelocatable<QPixmap> : ::std::true_type
{};

} // namespace rust
