#include "ydx_core.h"

#include <QDebug>
#include <QtConcurrent>


YdxCore::YdxCore(QObject* parent) : QObject(parent)
{}

void YdxCore::performanceTest() {
    QtConcurrent::run([this](){
        auto time = ydx_auroraos_performance_test();
        QString value = QString::number(time);
        m_state = "The test was performed: " + value + "ms";
        emit setState(m_state);
    });
    m_state = "Loading...";
    emit setState(m_state);
}
