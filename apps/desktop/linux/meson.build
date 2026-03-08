# Define our project
project(
  'com.keygenqt.ydx',
  ['c', 'rust'],
  version: '0.1.0',
  license: 'AGPL-3.0-or-later',
  meson_version: '>= 1.0.0',
  default_options: [
    'warning_level=2',
    'werror=false',
    'c_std=c11',
  ],
)

# Application ID
app_id = meson.project_name()

# Import the modules
i18n = import('i18n')
gnome = import('gnome')

# Define base directories
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
libdir = prefix / get_option('libdir')

# Configuration for template files
conf = configuration_data()
conf.set('app_id', app_id)
conf.set('PACKAGE_VERSION', meson.project_version())
conf.set('prefix', prefix)
conf.set('bindir', bindir)
conf.set('datadir', datadir)
conf.set('libdir', libdir)
conf.set('GJS', find_program('gjs').full_path())
conf.set('resource_path', '/' + app_id.replace('.', '/'))

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

# Build summary
summary({
  'Application ID': app_id,
  'Prefix': prefix,
  'Binary dir': bindir,
  'Data dir': datadir,
  'Library dir': libdir,
  'GJS': conf.get('GJS'),
  'License': 'AGPL-3.0-or-later',
}, section: 'Directories')
