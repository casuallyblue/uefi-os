{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system}.overrideToolchain fenix.packages.${system}.complete.toolchain;
        src = craneLib.path ./.;

        commonBuildInputs = with pkgs; [
          openssl
          pkg-config
          # Add additional build inputs here
        ] ++ lib.optionals pkgs.stdenv.isDarwin
          (with pkgs; with pkgs.darwin; [
            # Additional darwin specific inputs can be set here
            libiconv
            Security
          ]);


        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;

          buildInputs = commonBuildInputs;
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        my-crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages = {
          default = my-crate;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = my-crate;
        };

        devShells.default = pkgs.mkShell {
          # Extra inputs can be added here
          nativeBuildInputs =
            commonBuildInputs ++
            (with pkgs;
            [
              cargo
              fenix.packages.${system}.complete.toolchain
	            qemu
            ] ++ lib.optionals pkgs.stdenv.isDarwin (with darwin.apple_sdk; [
              frameworks.SystemConfiguration
              (pkgs.writeShellScriptBin "qemu-system-x86_64-uefi" (''
        	      qemu-system-x86_64 \
		              -drive if=pflash,format=raw,unit=0,file=${pkgs.qemu}/share/qemu/edk2-x86_64-code.fd,readonly=on \
          	      "$@"
          	  ''))
            ]) ++ lib.optionals pkgs.stdenv.isLinux ([
              (pkgs.writeShellScriptBin "qemu-system-x86_64-uefi" (''
        	      qemu-system-x86_64 \
                  --enable-kvm \
                  --cpu host \
		              -drive if=pflash,format=raw,unit=0,file=${pkgs.qemu}/share/qemu/edk2-x86_64-code.fd,readonly=on \
          	      "$@"
          	  ''))            
            ]));
        };
      });
}
