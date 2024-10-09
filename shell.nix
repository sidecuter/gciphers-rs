{pkgs ? import <nixpkgs> {}}: pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gcc
    libadwaita
    glib
    pango
    gdk-pixbuf
    gtk4
    pkg-config
    dbus
    meson
    desktop-file-utils
    ninja
  ];
}