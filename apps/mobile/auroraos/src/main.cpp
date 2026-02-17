#include <QtQuick>
#include <auroraapp.h>

#include "shared_rust.h"

int main(int argc, char *argv[])
{
    QScopedPointer<QGuiApplication> application(Aurora::Application::application(argc, argv));
    application->setOrganizationName(QStringLiteral("com.keygenqt"));
    application->setApplicationName(QStringLiteral("demo_rust"));

    qmlRegisterType<SharedRust>("SharedRust", 1, 0, "SharedRust");

    QScopedPointer<QQuickView> view(Aurora::Application::createView());
    view->setSource(Aurora::Application::pathTo(QStringLiteral("qml/demo_rust.qml")));
    view->show();

    return application->exec();
}
