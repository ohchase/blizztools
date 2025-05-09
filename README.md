# blizztools

>[!NOTE]
> The CDN is Blizzard's property, I have no association.
> Be respectful to their property when using this application and do not abuse

>[!WARNING]
> The application currently redownload's the install and encoding manifest for every individual download. This is very inefficient, please do not download many files through cli to be respectful to blizzard's cdn; just target the client binary for instance if that achieves your use case.
> Pull requests open if anybody wants to cache the install and encoding manifest.

Allows you to interact with blizzard cdn through cli.

Useful for ci/cd workflows dependent on client binaries.

WowDev wiki was used as the primary resource to develop this https://wowdev.wiki/TACT

Some keywords:
TACT, CDN, BLTE, Install Manifest, Download Manifest, CE Table, EncodingKey, ContentKey

## supported products
```
❯ cargo run version --help 
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target\debug\blizztools.exe version --help`
Versions command to query tact for a product version

Usage: blizztools.exe version <PRODUCT>

Arguments:
  <PRODUCT>
          Possible values:
          - diablo3:                Diablo 3 Retail
          - diablo3-ptr:            Diablo 3 Test
          - diablo4:                Diablo IV Retail, Fenris
          - diablo4-beta:           Diablo IV Beta , Fenris Beta
          - hearthstone:            Hearthstone Retail
          - hearthstone-tournament: Hearthstone Chournament
          - overwatch:              Overwatch Retail, Prometheus
          - overwatch-test:         Overwatch Test, Prometheus Test
          - warcraft3:              Warcraft III
          - wow:                    World of Warcraft Retail
          - wow-beta:               World of Warcraft Alpha/Beta
          - wow-classic:            World of Warcraft Classic (BCC)
          - wow-classic-beta:       World of Warcraft Classic (BCC) Beta
          - wow-classic-ptr:        World of Warcraft Classic (BCC) Test
          - wow-classic-era:        World of Warcraft Classic (Vanilla)
          - wow-classic-era-beta:   World of Warcraft Classic (Vanilla) Beta
          - wow-classic-era-ptr:    World of Warcraft Classic (Vanilla) Test
```

## help
```console
cli toolset for interacting with blizzard cdn

Usage: blizztools.exe <COMMAND>

Commands:
  version           Versions command to query tact for a product version
  cdn               Cdn command to query tact for cdns available for a product
  install-manifest  Command that will download the encoding and install manifest for a product
  download          Command that will download a selected file from a version's install
  help              Print this message or the help of the given subcommand(s)

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

### step 2, check the products install manifest

take note of which ever product CKey aka hash you want
```console
cargo run install-manifest wow-classic 

 INFO blizztools: latest version: 3.4.3.53788
 INFO blizztools: selected cdn: blzddist1-a.akamaihd.net/tpr/wow
 ...
 INFO blizztools: Name: BlizzardError.exe , CKey: 42a3179ed13971d3ef826b7be3871f73
 INFO blizztools: Name: Utils\WowVoiceProxy.exe , CKey: a8654e416b1c79f57570db1949b28bfe
 INFO blizztools: Name: Utils\BlizzardBrowser.exe , CKey: 210d88356b2867e9ad0d153933005acd
 INFO blizztools: Name: WowClassic.exe , CKey: 3bdf94e861f99559347cc9c576f0e236
 INFO blizztools: Name: UTILS\LIBCEF.DLL , CKey: b73a483b89d52bc4ec62c8392afeedfb
 INFO blizztools: Name: Utils\vivoxsdk.dll , CKey: 8f95106684dd7e99cd9999a50535459f
 INFO blizztools: Name: dxilconv7.dll , CKey: cf0ae82deafd5f795c654156aa79d493
 INFO blizztools: Name: d3d12.dll , CKey: 5c40ee95f29dad945fa3b630f103072e
 INFO blizztools: Name: Utils\chrome_elf.dll , CKey: 3ab59116ed74fd220498a6371a16e50f
 INFO blizztools: Name: Utils\swiftshader\libEGL.dll , CKey: ecaad2412ac28b7bfeffbaf18a4d41a0
 INFO blizztools: Name: Utils\swiftshader\libGLESv2.dll , CKey: 391df7b4b0811e32b63ddc282c072fb2
 INFO blizztools: Name: Utils\libEGL.dll , CKey: 15242f3fd6cd11d8aad782f7560acaad
 INFO blizztools: Name: Utils\d3dcompiler_47.dll , CKey: 222d020bd33c90170a8296adc1b7036a
 INFO blizztools: Name: Utils\libGLESv2.dll , CKey: 1bb1640968387e7d5027f864bcc98394
 INFO blizztools: Name: Utils\WowWindowsExceptionHandler.dll , CKey: 91def0128099945ffb353f2d20799245
 INFO blizztools: Name: Utils\WindowsExceptionHandler.dll , CKey: 4783be624c8abef10a4c70836aea5c8f
 INFO blizztools: Name: Utils\snapshot_blob.bin , CKey: e2e1f1b0bdcb9246897dc3e13add37b9
 ...
```

### step 3, download artifacts you desire

```console
cargo run download wow-classic 3bdf94e861f99559347cc9c576f0e236 ./target/output

 INFO blizztools: latest version: 3.4.3.53788
 INFO blizztools: output dir: "./target/output\\wow_classic\\3.4.3.53788"
 INFO blizztools: selected cdn: blzddist1-a.akamaihd.net/tpr/wow
 INFO blizztools: beginning download of content key: 3bdf94e861f99559347cc9c576f0e236
 INFO blizztools: successfully downloaded content key: 3bdf94e861... with size: 49655432
```

the binaries will be downloaded into your target output directory under {product}/{version}/{c_key} hierarchy
```tree
./target/output
├── wow
│   ├── 10.2.5.53040
│   │   ├── c_key_0000000
│   │   └── c_key_0000001
│   └── 10.2.5.53441
│       ├── c_key_0000000
│       └── c_key_0000001
├── wow_classic
│   └── 3.4.3.53788
│       ├── 3bdf94e861f99559347cc9c576f0e236
│       └── c_key_0000001
├── fenris
│   └── 1.3.5.52293
│       ├── 457a5b5cb0f86c5ff45fee9addbc6c4c
└── wow_classic_era
    └── 1.15.0.52610
        ├── c_key_0000000
        └── c_key_0000001

8 directories, 8 files
```
