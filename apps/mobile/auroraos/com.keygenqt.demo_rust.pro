TARGET = com.keygenqt.demo_rust

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
    $$PWD/../../shared/rust/libs/linuxX64/libshared_api.h \
    src/shared_rust.h \

SOURCES += \
    src/main.cpp \
    src/shared_rust.cpp \

DISTFILES += \
    rpm/com.keygenqt.demo_rust.spec \

AURORAAPP_ICONS = 86x86 108x108 128x128 172x172

TRANSLATIONS += \
    translations/com.keygenqt.demo_rust.ts \
    translations/com.keygenqt.demo_rust-ru.ts \

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
INCLUDEPATH += $$PWD/../../shared/rust/libs/$${RUST_TARGET}
LIBS += -L$$PWD/../../shared/rust/libs/$${RUST_TARGET} -lshared

# add lib to rpm
CONFIG += shared
shared.files = $$PWD/../../shared/rust/libs/$${RUST_TARGET}
shared.path = /usr/share/$${TARGET}
INSTALLS += shared
