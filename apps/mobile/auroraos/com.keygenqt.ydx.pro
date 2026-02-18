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
    $$PWD/../../../core/build/auroraos/armv7-unknown-linux-gnueabihf/ydx_auroraos.h \
    src/ydx_core.h \

SOURCES += \
    src/main.cpp \
    src/ydx_core.cpp \

DISTFILES += \
    rpm/com.keygenqt.ydx.spec \

AURORAAPP_ICONS = 86x86 108x108 128x128 172x172

TRANSLATIONS += \
    translations/com.keygenqt.ydx.ts \
    translations/com.keygenqt.ydx-ru.ts \

# get target name rust
contains(QMAKE_TARGET.arch, armv7l): {
    RUST_TARGET = "armv7-unknown-linux-gnueabihf"
}
contains(QMAKE_TARGET.arch, aarch64): {
    RUST_TARGET = "aarch64-unknown-linux-gnu"
}
contains(QMAKE_TARGET.arch, x86_64): {
    RUST_TARGET = "x86_64-unknown-linux-gnu"
}

# add rust static lib
INCLUDEPATH += $$PWD/../../../core/build/auroraos/$${RUST_TARGET}
LIBS += -L$$PWD/../../../core/build/auroraos/$${RUST_TARGET} -lydx_auroraos

# add lib to rpm
CONFIG += ydx_auroraos
ydx_auroraos.files = $$PWD/../../../core/build/auroraos/$${RUST_TARGET}
ydx_auroraos.path = /usr/share/$${TARGET}
INSTALLS += ydx_auroraos
