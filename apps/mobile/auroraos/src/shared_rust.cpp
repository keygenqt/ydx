#include "shared_rust.h"

#include <QDebug>
#include <QtConcurrent>


SharedRust::SharedRust(QObject* parent) : QObject(parent)
{}

void SharedRust::performanceTest() {
    QtConcurrent::run([this](){
        auto time = shared_performance_test();
        QString value = QString::number(time);
        m_state = "The test was performed: " + value + "ms";
        emit setState(m_state);
    });
    m_state = "Loading...";
    emit setState(m_state);
}
