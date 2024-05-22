{
  description = "Rust nightly development shell with subxt";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
      pkgs.mkShell {
        buildInputs = with pkgs; [
          protobuf
          pkg-config
          glib
          openssl
          clang
          (
            rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override {
                extensions = ["rust-src"];
              })
          )
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [pkgs.openssl];
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
      };
  };
}
