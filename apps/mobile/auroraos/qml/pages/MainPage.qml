import QtQuick 2.0
import Sailfish.Silica 1.0
import QtGraphicalEffects 1.0
import SharedRust 1.0

Page {
    objectName: "mainPage"
    allowedOrientations: Orientation.All

    SharedRust {
        id: shared
    }

    PageHeader {
        objectName: "pageHeader"
        title: qsTr("Demo Rust")
    }

    Column {
        id: content
        width: parent.width - Theme.paddingLarge * 2
        anchors.centerIn: parent
        spacing: Theme.paddingLarge * 2

        Text {
            width: parent.width
            text: qsTr("Aurora OS + Rust")
            color: Theme.primaryColor
            wrapMode: Text.WordWrap
            font.pixelSize: Theme.fontSizeExtraLarge
            horizontalAlignment: Qt.AlignCenter
        }

        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: Theme.paddingLarge

            Image {
                width: 99
                height: 120
                source: Qt.resolvedUrl("../icons/aurora.svg")
            }

            Image {
                id: testImage

                width: 120
                height: 120
                source: Qt.resolvedUrl("../icons/rust.svg")

                ColorOverlay {
                    anchors.fill: testImage
                    source: testImage
                    color: "#B7410E"
                }
            }
        }

        Button {
            anchors.horizontalCenter: parent.horizontalCenter
            text: shared.state
            onClicked: shared.performanceTest()
        }

        Text {
            width: parent.width
            text: qsTr("Click to test the performance of the\nRust C library.")
            color: Theme.primaryColor
            wrapMode: Text.WordWrap
            font.pixelSize: Theme.fontSizeSmall
            horizontalAlignment: Qt.AlignCenter
        }
    }
}
