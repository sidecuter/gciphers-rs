pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

gnome.compile_resources('gciphers-rs',
  'gciphers-rs.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'release'
  cargo_opt += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif


if host_machine.system() == 'windows'
  cmd = [
    'compile.sh', meson.project_build_root()  / 'cargo-home', 
    cargo_bin, 'build', cargo_opt,
    meson.project_build_root() / 'src' / rust_target / meson.project_name(),
    meson.project_build_root() / meson.project_name()
  ]
  rust_exec = meson.project_name() + '.exe'
else
  cmd = [
    'env', cargo_env,
    cargo_bin, 'build',
    cargo_opt, '&&', 'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
  rust_exec = meson.project_name()
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: rust_exec,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: cmd
)
