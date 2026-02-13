#pragma once

#include <QTimer>

#include <utility>

#include "rust/cxx.h"

namespace rust::cxxqtlib1 {
void single_shot_trampoline(::rust::u64 callback_id) noexcept;
} // namespace rust::cxxqtlib1

template <typename Duration>
inline void singleShot(Duration interval, const QObject* context, rust::Fn<void()> functor) {
	QTimer::singleShot(
			interval,
			context,
			[functor = std::move(functor)]() mutable {
				functor();
			});
}

template <typename Duration>
inline void singleShot(Duration interval, const QObject* context, ::rust::u64 callback_id) {
	QTimer::singleShot(
			interval,
			context,
			[callback_id]() mutable {
				rust::cxxqtlib1::single_shot_trampoline(callback_id);
			});
}

