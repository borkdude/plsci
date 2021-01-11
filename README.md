# pl/sci

## Build

1. Run `script/compile-libplsci`. This requires [lein](https://leiningen.org/) and `GRAALVM_HOME` to be set to an installation of GraalVM 20.3.0 Java 11. Currently this build script contains some macOS-specific things like `.dylib` and `install_name_tool`. This will be made more portable later on.

## Run

1. Run `cargo pgx run pg13`. Then try it out:

```
psql (13.0)
Type "help" for help.

plsci=# create extension plsci;
CREATE EXTENSION

plsci=# select * from plsci('(+ 1 2 3)');
 plsci
-------
 6
(1 row)
```

## License

Copyright © 2019-2020 Michiel Borkent

Distributed under the EPL License. See LICENSE.
