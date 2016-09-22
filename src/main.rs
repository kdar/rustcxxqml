#![feature(plugin)]
#![plugin(rustcxx_plugin)]

#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
#[link(name = "Qt5Quick")]
#[link(name = "Qt5Qml")]
extern "C" {}

use std::ffi::CStr;
use std::str;

cxx_inline! {
  #include <QtGui/QGuiApplication>
  #include <QtWidgets/QPushButton>
  #include <QtQml/QQmlApplicationEngine>
  #include <QtQuick/QQuickWindow>
  #include "wobjectdefs.h"
  #include "wobjectimpl.h"

  void *operator new(std::size_t n) {
    void *const p = std::malloc(n);
    return p;
  }

  void operator delete(void *p, std::size_t) {
    std::free(p);
  }

  class MyClass: public QObject {
    W_OBJECT(MyClass)
  public:
    void cppSlot(const QString &msg) {
      const int8_t *s = reinterpret_cast<const int8_t *>(msg.toUtf8().constData());
      rust![(s: *const i8) {
        let c_str = CStr::from_ptr(s);
        let bytes = c_str.to_bytes();
        println!("signal: {}", str::from_utf8(bytes).unwrap());
      }];
    }
    W_SLOT(cppSlot)
  };

  W_OBJECT_IMPL(MyClass)
}

fn main() {
  std::process::exit(unsafe {
    cxx![() -> i32{
  		int argc = 0;
  		char arg[] = "rustcxxqml";
  		char * argv [] = {arg, 0};

  		QGuiApplication app (argc, argv);
	    QQmlApplicationEngine engine;
	    engine.load(QUrl("main.qml"));
	    QObject *topLevel = engine.rootObjects().value(0);
      MyClass myClass;
      QObject::connect(topLevel, SIGNAL(qmlSignal(QString)),
                        &myClass, SLOT(cppSlot(QString)));
	    QQuickWindow *window = qobject_cast<QQuickWindow *>(topLevel);
	    window->show();
  		app.exec()
    }]
  });
}
