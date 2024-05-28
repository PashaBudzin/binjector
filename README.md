# simple base 16 colors injector
use ${base0X} to inject colors into files specified in config.toml

# example

```toml
[colors]
base00 = "FFFFFF"
base01 = "FFFFFF"
base02 = "FFFFFF"
base03 = "FFFFFF"
base04 = "FFFFFF"
base05 = "FFFFFF"
base06 = "FFFFFF"
base07 = "FFFFFF"
base08 = "FFFFFF"
base09 = "FFFFFF"
base0a = "FFFFFF"
base0b = "FFFFFF"
base0c = "FFFFFF"
base0d = "FFFFFF"
base0e = "FFFFFF"
base0f = "FFFFFF"

[paths]
"~/base16/from1.txt" = "~/base16/to.txt"
```
then build it using 

```sh
binjector build # looks for config.toml in current directory
```

# installation
just compile it
```sh
cargo build --release && sudo cp target/release/binjector /usr/local/bin/
```
