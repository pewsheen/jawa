#pragma once

#include <QNetworkCookie>

#include <memory>
#include "rust/cxx.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkCookie, {
  ::std::size_t a1;
});

namespace rust {

template<>
struct IsRelocatable<QNetworkCookie> : ::std::true_type
{};

} // namespace rust