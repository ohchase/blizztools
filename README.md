# blizztools

Allows you to interact with blizzard cdn through cli.

Useful for ci/cd workflows dependent on client binaries.

Supports
- wow_retail
- wow_classic
- wow_classic_era
- anything else just implement the enum entry...

Some keywords:
TACT, CDN, BLTE, Install Manifest, Download Manifest, CE Table, EncodingKey, ContentKey

## help
```console
Usage: blizztools <COMMAND>

Commands:
  version   Versions command to query tact for a product version
  cdn       Cdn command to query tact for cdns available for a product
  download  Command that will download a selected file from a version's install
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
````

## download quickstart

### step 1, check the version

check the version of which ever product your downloading 
```console
cargo run version wow-classic

INFO blizztools: [
    VersionDefinition {
        region: "us",
        build_config: 268a7d2d4bd28cad7c3779a1f5d0a11d,
        cdn_config: cf4afeeb86e392e4623f7969c89243f2,
        key_ring: None,
        build_id: "52237",
        version_name: "3.4.3.52237",
        product_config: fac7680539cd51bc0a791a88ade3da21,
    },
    ...
]
```

### step 2, download it

download which ever product you want
```console
cargo run download wow-classic ./output

INFO blizztools: latest version: 3.4.3.52237
INFO blizztools: output dir: "./output/wow_classic/3.4.3.52237"
INFO blizztools: selected cdn: blzddist1-a.akamaihd.net/tpr/wow
INFO blizztools: beginning download of WowClassic.exe
INFO blizztools: successfully downloaded WowClassic.exe size: 49479816
INFO blizztools: beginning download of WowClassic-arm64.exe
INFO blizztools: successfully downloaded WowClassic-arm64.exe size: 42883208
````

### step 3, profit ????

the binaries will be downloaded into your target output directory under {product}/{version} hierarchy
```tree
output
├── wow
│   ├── 10.2.5.53040
│   │   ├── Wow-ARM64.exe
│   │   └── Wow.exe
│   └── 10.2.5.53441
│       ├── Wow-ARM64.exe
│       └── Wow.exe
├── wow_classic
│   └── 3.4.3.52237
│       ├── WowClassic-arm64.exe
│       └── WowClassic.exe
└── wow_classic_era
    └── 1.15.0.52610
        ├── WowClassic-arm64.exe
        └── WowClassic.exe

8 directories, 8 files
```
