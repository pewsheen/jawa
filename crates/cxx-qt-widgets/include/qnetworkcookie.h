#pragma once

#include <QNetworkCookie>

#include <memory>

#include "cxx-qt-lib/qbytearray.h"

inline std::unique_ptr<QNetworkCookie> qnetworkcookieNew()
{
  return std::make_unique<QNetworkCookie>();
}

inline std::unique_ptr<QNetworkCookie> qnetworkcookieNewWithNameValue(const QByteArray& name,
                                                                       const QByteArray& value)
{
  return std::make_unique<QNetworkCookie>(name, value);
}
