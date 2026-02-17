TARGET = com.keygenqt.ydx

# set target name from spec file
QMAKE_TARGET.arch = $$(QMAKE_TARGET)
cache(QMAKE_TARGET.arch, set)

QT += concurrent

CONFIG += \
    auroraapp \
    auroraapp_i18n \

PKGCONFIG += \

HEADERS += \
    # default path header libs (linuxX64 == linuxArm64)
    $$PWD/../../../core/libs/linuxX64/libcore.h \
    src/core_ydx.h \

SOURCES += \
    src/main.cpp \
    src/core_ydx.cpp \

DISTFILES += \
    rpm/com.keygenqt.ydx.spec \

AURORAAPP_ICONS = 86x86 108x108 128x128 172x172

TRANSLATIONS += \
    translations/com.keygenqt.ydx.ts \
    translations/com.keygenqt.ydx-ru.ts \

# get target name rust
contains(QMAKE_TARGET.arch, armv7l): {
    error("Unsupported architecture armv7l")
}
contains(QMAKE_TARGET.arch, aarch64): {
    RUST_TARGET = "linuxArm64"
}
contains(QMAKE_TARGET.arch, x86_64): {
    RUST_TARGET = "linuxX64"
}

# add rust static lib
INCLUDEPATH += $$PWD/../../../core/libs/$${RUST_TARGET}
LIBS += -L$$PWD/../../../core/libs/$${RUST_TARGET} -lcore

# add lib to rpm
CONFIG += shared
shared.files = $$PWD/../../../core/libs/$${RUST_TARGET}
shared.path = /usr/share/$${TARGET}
INSTALLS += shared
