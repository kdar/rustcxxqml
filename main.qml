import QtQuick 2.4
import QtQuick.Controls 1.2
import QtQuick.Window 2.2

ApplicationWindow {
  id: win
  title: qsTr("Demo rustcxx and QT's QML")
  width: 300
  height: 200

  signal qmlSignal(string msg)

  menuBar: MenuBar {
    Menu {
      title: qsTr("File")
      MenuItem {
        text: qsTr("&Open")
        onTriggered: win.qmlSignal("Clicked Menu \"Open\" from QML!")
      }
      MenuItem {
        text: qsTr("Exit")
        onTriggered: Qt.quit();
      }
    }
  }
  Button {
    text: qsTr("Click and check console!")
    anchors.horizontalCenter: parent.horizontalCenter
    anchors.verticalCenter: parent.verticalCenter
    onClicked: win.qmlSignal("Clicked button from QML!")
  }
}
