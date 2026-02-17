#include <QtQuick>
#include <auroraapp.h>

#include "core_ydx.h"

int main(int argc, char *argv[])
{
    QScopedPointer<QGuiApplication> application(Aurora::Application::application(argc, argv));
    application->setOrganizationName(QStringLiteral("com.keygenqt"));
    application->setApplicationName(QStringLiteral("ydx"));

    qmlRegisterType<CoreYDX>("CoreYDX", 1, 0, "CoreYDX");

    QScopedPointer<QQuickView> view(Aurora::Application::createView());
    view->setSource(Aurora::Application::pathTo(QStringLiteral("qml/ydx.qml")));
    view->show();

    return application->exec();
}
