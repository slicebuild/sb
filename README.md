### Installing on Windows
curl-rust crate is broken for Windows now. To compile it msys is required.
First of all try to compile it with default Rust installation. If it failed then perform following steps:
* Go to https://github.com/rust-lang/rust and perform actions required to install with msys2
* Compile rustc and install
* Download sources of cmake.
* Compile cmake and install in mingw shell. Do not forget about --prefix=/usr/local/bin
* Compile cargo and install
* After that this project will be compiled
