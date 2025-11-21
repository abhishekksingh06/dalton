{
  description = "Dalton a Modern Programming language";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      stdenv = pkgs.llvmPackages_latest.stdenv;
      gccForLibs = stdenv.cc.cc;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          bashInteractive
          clang-tools
          python3
          ninja
          cmake
          llvmPackages_latest.llvm
          git
          curl
        ];

        inherit stdenv;

        CPLUS_INCLUDE_PATH = "${pkgs.llvmPackages_latest.libcxx.dev}/include/c++/v1";
        C_INCLUDE_PATH = "${stdenv.cc.libc.dev}/include";

        NIX_LDFLAGS = "-L${gccForLibs}/lib/gcc/${stdenv.targetPlatform.config}/${gccForLibs.version}";
        CFLAGS = "-B${gccForLibs}/lib/gcc/${stdenv.targetPlatform.config}/${gccForLibs.version} -B${stdenv.cc.libc}/lib";

        CMAKE_FLAGS = [
          "-DGCC_INSTALL_PREFIX=${gccForLibs}"
          "-DC_INCLUDE_DIRS=${stdenv.cc.libc.dev}/include"
          "-GNinja"
          "-DCMAKE_BUILD_TYPE=Release"
          "-DCMAKE_INSTALL_PREFIX=../inst"
          "-DLLVM_INSTALL_TOOLCHAIN_ONLY=ON"
          "-DLLVM_ENABLE_PROJECTS=clang"
          "-DLLVM_ENABLE_RUNTIMES=libcxx;libcxxabi"
          "-DLLVM_TARGETS_TO_BUILD=host"
        ];

        shellHook = ''
          echo "Welcome to the LLVM development shell!"
          echo "C++ include path: $CPLUS_INCLUDE_PATH"
          echo "C include path: $C_INCLUDE_PATH"
        '';
      };
    };
}
