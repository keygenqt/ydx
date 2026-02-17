/**
 * SPDX-FileCopyrightText: Copyright 2025 Open Mobile Platform LLC <community@omp.ru>
 * SPDX-License-Identifier: BSD-3-Clause
 */
#ifndef SHARED_KMP_H
#define SHARED_KMP_H

#include <QObject>
#include <QtQml>

#include <../../shared/rust/libs/linuxX64/libshared_api.h>

class SharedRust : public QObject
{
    Q_OBJECT

public:
    explicit SharedRust(QObject* parent = nullptr);

    Q_PROPERTY(QString state MEMBER m_state NOTIFY setState)
    Q_INVOKABLE void performanceTest();

signals:
    void setState(QString);

private:
    QString m_state = "Run performance test";
};

#endif // SHARED_KMP_H
