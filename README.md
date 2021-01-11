# pl/sci

## Build

1. Run `script/compile-libplsci`. This requires [lein](https://leiningen.org/) and `GRAALVM_HOME` to be set to an installation of GraalVM 20.3.0 Java 11. Currently this build script contains some macOS-specific things like `.dylib` and `install_name_tool`. This will be made more portable later on.

2. Run `cargo pgx run pg13`.
