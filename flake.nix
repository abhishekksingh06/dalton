{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        p = import nixpkgs { inherit system; };
        llvm = p.llvmPackages_latest;
        mymake = p.writeShellScriptBin "mk" ''
          if [ -f "$1.c" ]; then
            i="$1.c"
            c=$CC
          else
            i="$1.cpp"
            c=$CXX
          fi
          o=$1
          shift
          $c -ggdb $i -o $o -lm -Wall $@
        '';
      in
      {
        devShell = p.mkShell {
          packages = with p; [
            gnumake
            cmake
            bear
            llvm.lldb
            llvm.llvm
            gdb
            clang-tools
            llvm.clang
            llvm.libcxx
            cppcheck
            llvm.libllvm
            valgrind
            mymake
            glm
            SDL2
            SDL2_gfx
          ];

          shellHook = ''
            export CXX="clang++"
            export CC="clang"
            cat > compile_flags.txt << EOF
            -stdlib=libc++
            -I${llvm.libcxx.dev}/include/c++/v1
            EOF
          '';

          name = "dalton";
        };
      }
    );
}
