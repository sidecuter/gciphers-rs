project('gciphers-rs', ['rust', 'c'], 
          version: '1.0.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

app_id = meson.project_name()

gnome = import('gnome')

if host_machine.system() == 'windows'
  subdir('windows')
endif

subdir('data')
subdir('src')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
