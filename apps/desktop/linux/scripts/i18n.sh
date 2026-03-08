#!/bin/bash

######################
## Update translations
######################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

FOLDER='build-i18n'

echo "Generating POTFILES..."

# Clean and create POTFILES (заменяем старый файл)
rm -f po/POTFILES po/POTFILES.in
touch po/POTFILES

# Find all translatable files
find data -type f -name "*.ui" | sort >> po/POTFILES
find data -type f -name "*.desktop.in" | sort >> po/POTFILES
find data -type f -name "*.metainfo.xml.in" | sort >> po/POTFILES
find src -type f -name "*.js" | sort >> po/POTFILES

# Create symlink or copy for compatibility
cp po/POTFILES po/POTFILES.in

echo "Checking languages..."

# Initialize missing .po files from LINGUAS
if [ -f po/LINGUAS ]; then
    # Фильтруем пустые строки и комментарии
    grep -v '^#' po/LINGUAS | grep -v '^$' | while read lang; do
        if [ ! -z "$lang" ] && [ ! -e "po/$lang.po" ]; then
            echo "Creating po/$lang.po"
            msginit --locale="$lang" --input=po/com.keygenqt.ydx.pot --output=po/"$lang.po" --no-translator 2>/dev/null || \
            cat > "po/$lang.po" <<CONTENT_TYPE
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\\n"
"Plural-Forms: nplurals=INTEGER; plural=EXPRESSION;\\n"

CONTENT_TYPE
        fi
    done
fi

echo "Setting up build directory..."

# Setup meson
meson setup $FOLDER > /dev/null 2>&1

echo "Generating POT template..."

# Generate POT file
meson compile -C $FOLDER com.keygenqt.ydx-pot

echo "Updating PO files..."

# Update PO files
meson compile -C $FOLDER com.keygenqt.ydx-update-po

# Cleanup
rm -rf $FOLDER

echo "Done! Translations updated."
