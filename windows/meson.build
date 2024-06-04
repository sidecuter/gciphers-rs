win_conf = configuration_data()
win_conf.set('APP_ID_FIXED', app_id)
win_conf.set('REPO_PATH', meson.project_source_root())
win_conf.set_quoted('APP_ID', app_id)
win_conf.set_quoted('VERSION', meson.project_version())

custom_target('copy icon',
  input : '@0@.ico'.format(app_id),
  output :  '@0@.ico'.format(app_id),
  command : ['cp', '@INPUT@', '@OUTPUT@'],
  install : false,
  build_by_default : true
)

configure_file(
  input: './version.rc.in',
  output: 'version.rc',
  configuration: win_conf,
  install: false,
)

configure_file(
  input: './gciphers-rs.rc.in',
  output: 'gciphers-rs.rc',
  configuration: win_conf,
  install: false,
)

windows = import('windows')
gciphers_sources += windows.compile_resources(
    join_paths(meson.current_build_dir(), 'gciphers-rs.rc'), 
    join_paths(meson.current_build_dir(), 'version.rc'),
)

meson.add_install_script('post_install.py')
