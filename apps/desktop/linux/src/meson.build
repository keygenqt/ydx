pkgdatadir = get_option('datadir') / app_id
gnome = import('gnome')

# Path to resource files in data directory
src_resources_xml = join_paths(meson.project_source_root(), 'data', app_id + '.src.gresource.xml')
data_resources_xml = join_paths(meson.project_source_root(), 'data', app_id + '.data.gresource.xml')

# Path to assets for source_dir
assets_dir = join_paths(meson.project_source_root(), 'data', 'assets')

# Compile source resources (JS files)
src_res = gnome.compile_resources(app_id + '.src',
  src_resources_xml,
  source_dir: meson.current_source_dir(),
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

# Compile data resources (UI, CSS, images)
data_res = gnome.compile_resources(app_id + '.data',
  data_resources_xml,
  source_dir: [
    meson.current_source_dir(),
    assets_dir,
  ],
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

# Configure and install the main executable using the global conf
configure_file(
  input: app_id + '.in',
  output: app_id,
  configuration: conf,
  install: true,
  install_dir: get_option('bindir')
)
