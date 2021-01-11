# pl/sci

## Build

### Requirements

- [lein](https://leiningen.org/)
- [GraalVM CE 20.3.0 Java 11](https://github.com/graalvm/graalvm-ce-builds/releases/tag/vm-20.3.0).
- [cargo](https://doc.rust-lang.org/stable/cargo/)

### Steps

1. Set `GRAALMV_HOME` to your GraalVM directory (the one that contains the `bin`
   directory).

2. Run `script/compile-libplsci`.  Currently this build script contains some
   macOS-specific things like `.dylib` and `install_name_tool`. This will be
   made more portable later on.

2. Run `cargo pgx run pg13`. Then try it out:

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

Copyright © 2021 Michiel Borkent

Distributed under the EPL License. See LICENSE.
