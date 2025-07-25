project('bign-handheld-thumbnailer', 'rust',
  version: '1.1.3',
  license: 'GPL-2.0-or-later',
  meson_version: '>= 0.64.0')

addon_id = 'dev.mateusrodcosta.bign_handheld_thumbnailer'

gnome = import('gnome')

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
thumbnailers_dir = prefix / get_option('datadir') / 'thumbnailers'
metainfo_dir = prefix / get_option('datadir') / 'metainfo'

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

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env', cargo_env,
    cargo_bin, 'build',
    cargo_opt, '&&', 'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)

configure_file(input : 'data/' + meson.project_name() + '.thumbnailer.in',
               output : meson.project_name() + '.thumbnailer',
               configuration : {'bindir' : bindir},
               install_dir : thumbnailers_dir)

install_data(
    'data/mime/bign-handheld-thumbnailer.xml',
    install_dir: get_option('datadir') / 'mime/packages',
)

if get_option('update_mime_database')
  gnome.post_install(
      update_mime_database: true
  )
endif

if get_option('generate_metainfo')
  configure_file(input : 'data/' + meson.project_name() + '.metainfo.xml.in',
                output : addon_id + '.metainfo.xml',
                configuration : {'addon_id' : addon_id},
                install_dir : metainfo_dir)
endif
