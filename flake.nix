{
  description = "LQXP Client Tauri development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-tauri
            gobject-introspection
            nodejs
            pkg-config
            rustc
            rustup
            wrapGAppsHook4
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            glib-networking
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            openssl
            pango
            webkitgtk_4_1
            xdotool
          ];

          shellHook = ''
            export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
            export WEBKIT_DISABLE_DMABUF_RENDERER=1
          '';
        };
      });
}
