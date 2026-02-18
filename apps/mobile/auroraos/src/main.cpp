#include <QtQuick>
#include <auroraapp.h>

#include "ydx_core.h"

int main(int argc, char *argv[])
{
    QScopedPointer<QGuiApplication> application(Aurora::Application::application(argc, argv));
    application->setOrganizationName(QStringLiteral("com.keygenqt"));
    application->setApplicationName(QStringLiteral("ydx"));

    qmlRegisterType<YdxCore>("YdxCore", 1, 0, "YdxCore");

    QScopedPointer<QQuickView> view(Aurora::Application::createView());
    view->setSource(Aurora::Application::pathTo(QStringLiteral("qml/ydx.qml")));
    view->show();

    return application->exec();
}
