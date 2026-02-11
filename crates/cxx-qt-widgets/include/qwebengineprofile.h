#pragma once

#include <QWebEngineProfile>
#include <QWebEngineNotification>
#include <utility>

#include "rust/cxx.h"
#include "cxx-qt-lib/qstring.h"

using PersistentCookiesPolicy = QWebEngineProfile::PersistentCookiesPolicy;

inline void setNotificationPresenter(
	QWebEngineProfile& profile,
	rust::Fn<void(std::unique_ptr<QWebEngineNotification>)> presenter)
{
	profile.setNotificationPresenter(
		[presenter = std::move(presenter)](std::unique_ptr<QWebEngineNotification> notification) mutable {
			presenter(std::move(notification));
		});
}

