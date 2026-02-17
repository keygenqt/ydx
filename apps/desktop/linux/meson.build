# Define our project
project(
  'demo-rust',
  ['c', 'rust'],
  version: '0.1.0',
  license: ['Apache 2.0'],
  meson_version: '>= 1.0.0',
  default_options: ['warning_level=2', 'werror=false'],
)

# Import the modules
i18n = import('i18n')
gnome = import('gnome')

# Load instructions from subdirectories
subdir('data')
subdir('src')
subdir('po')

# Post installation tasks
gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
