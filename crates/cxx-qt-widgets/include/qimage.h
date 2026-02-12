#pragma once

#include <QImage>

#include "rust/cxx.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QImage, {
  ::std::size_t a1;
  ::std::size_t a2;
  ::std::size_t a3;
});

namespace rust {

template<>
struct IsRelocatable<QImage> : ::std::true_type
{};

} // namespace rust
