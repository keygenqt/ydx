#include "core_ydx.h"

#include <QDebug>
#include <QtConcurrent>


CoreYDX::CoreYDX(QObject* parent) : QObject(parent)
{}

void CoreYDX::performanceTest() {
    QtConcurrent::run([this](){
        auto time = shared_performance_test();
        QString value = QString::number(time);
        m_state = "The test was performed: " + value + "ms";
        emit setState(m_state);
    });
    m_state = "Loading...";
    emit setState(m_state);
}
