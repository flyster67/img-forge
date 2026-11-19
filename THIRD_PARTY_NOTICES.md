# Third-Party Software Notices & Licenses

This document acknowledges the third-party open-source components and libraries incorporated into `img-forge` and its pre-compiled binary distributions.

## Primary Dependencies & Licensing Overview

| Package | Version | License | Upstream Repository |
| :--- | :--- | :--- | :--- |
| `zenwebp` | 0.4.6 | AGPL-3.0-only OR Commercial | https://github.com/imazen/zenwebp |
| `ravif` | 0.13.0 | BSD-3-Clause | https://github.com/kornelski/cavif-rs |
| `image` | 0.25.10 | MIT OR Apache-2.0 | https://github.com/image-rs/image |
| `clap` | 4.6.8 | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| `rayon` | 1.12.0 | MIT OR Apache-2.0 | https://github.com/rayon-rs/rayon |
| `notify` | 8.2.2 | CC0-1.0 OR Apache-2.0 | https://github.com/notify-rs/notify |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_json` | 1.0.154 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| `toml` | 0.9.12 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| `walkdir` | 2.5.0 | Unlicense OR MIT | https://github.com/BurntSushi/walkdir |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 | https://github.com/dtolnay/anyhow |

---

## Notable Notices and Attributions

### 1. zenwebp
- **License**: AGPL-3.0-only OR LicenseRef-Imazen-Commercial
- **Copyright**: Copyright (c) 2011-2024 Imazen LLC and contributors; upstream WebP decoding/encoding portions copyright (c) image-rs developers.
- **Notice**: In accordance with the GNU Affero General Public License version 3, the complete source code for `img-forge` is available under the GNU AGPL-3.0.

### 2. ravif & avif-serialize
- **License**: BSD-3-Clause
- **Copyright**:
  - `ravif`: Copyright (c) 2020 Kornel Szabo
  - `avif-serialize`: Copyright (c) 2020 Cloudflare, Inc.
- **Redistribution Notice**:
  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the copyright notices, this list of conditions and the disclaimer are retained.

### 3. rav1e & v_frame
- **License**: BSD-2-Clause
- **Copyright**: Copyright (c) the rav1e contributors
- **Redistribution Notice**:
  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the copyright notices and disclaimer are retained.

### 4. MIT and Apache-2.0 Components
The majority of utility crates (including `image`, `clap`, `rayon`, `serde`, `anyhow`, and `toml`) are dual-licensed under MIT and Apache-2.0.
- For MIT-licensed software: Permission is hereby granted, free of charge, to deal in the Software without restriction, subject to retaining the respective copyright notices.
- For Apache-2.0 licensed software: Licensed under the Apache License, Version 2.0.

---

## Complete Dependency Manifest

The following table enumerates all direct and transitive dependencies resolved in `Cargo.lock`:

| Component | Version | License | Source |
| :--- | :--- | :--- | :--- |
| `adler2` | `2.0.1` | 0BSD OR MIT OR Apache-2.0 | [https://github.com/oyvindln/adler2](https://github.com/oyvindln/adler2) |
| `aligned` | `0.4.3` | MIT OR Apache-2.0 | [https://github.com/rust-embedded-community/aligned](https://github.com/rust-embedded-community/aligned) |
| `aligned-vec` | `0.6.4` | MIT | [https://github.com/sarah-ek/aligned-vec/](https://github.com/sarah-ek/aligned-vec/) |
| `anstream` | `1.0.0` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `anstyle` | `1.0.14` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `anstyle-parse` | `1.0.0` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `anstyle-query` | `1.1.5` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `anstyle-wincon` | `3.0.11` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `anyhow` | `1.0.104` | MIT OR Apache-2.0 | [https://github.com/dtolnay/anyhow](https://github.com/dtolnay/anyhow) |
| `arbitrary` | `1.5.0` | MIT OR Apache-2.0 | [https://github.com/rust-fuzz/arbitrary/](https://github.com/rust-fuzz/arbitrary/) |
| `archmage` | `0.9.31` | MIT OR Apache-2.0 | [https://github.com/imazen/archmage](https://github.com/imazen/archmage) |
| `archmage-macros` | `0.9.31` | MIT OR Apache-2.0 | [https://github.com/imazen/archmage](https://github.com/imazen/archmage) |
| `arg_enum_proc_macro` | `0.3.4` | MIT | [https://github.com/lu-zero/arg_enum_proc_macro](https://github.com/lu-zero/arg_enum_proc_macro) |
| `arrayvec` | `0.7.8` | MIT OR Apache-2.0 | [https://github.com/bluss/arrayvec](https://github.com/bluss/arrayvec) |
| `as-slice` | `0.2.1` | MIT OR Apache-2.0 | [https://github.com/japaric/as-slice](https://github.com/japaric/as-slice) |
| `autocfg` | `1.5.1` | Apache-2.0 OR MIT | [https://github.com/cuviper/autocfg](https://github.com/cuviper/autocfg) |
| `av-scenechange` | `0.14.1` | MIT | [https://github.com/rust-av/av-scenechange](https://github.com/rust-av/av-scenechange) |
| `av1-grain` | `0.2.5` | BSD-2-Clause | [https://github.com/rust-av/av1-grain](https://github.com/rust-av/av1-grain) |
| `avif-serialize` | `0.8.9` | BSD-3-Clause | [https://github.com/kornelski/avif-serialize](https://github.com/kornelski/avif-serialize) |
| `bitflags` | `2.13.2` | MIT OR Apache-2.0 | [https://github.com/bitflags/bitflags](https://github.com/bitflags/bitflags) |
| `bitstream-io` | `4.10.0` | MIT/Apache-2.0 | [https://github.com/tuffy/bitstream-io](https://github.com/tuffy/bitstream-io) |
| `built` | `0.8.1` | MIT | [https://github.com/lukaslueg/built](https://github.com/lukaslueg/built) |
| `bumpalo` | `3.20.4` | MIT OR Apache-2.0 | [https://github.com/fitzgen/bumpalo](https://github.com/fitzgen/bumpalo) |
| `bytemuck` | `1.25.2` | Zlib OR Apache-2.0 OR MIT | [https://github.com/Lokathor/bytemuck](https://github.com/Lokathor/bytemuck) |
| `bytemuck_derive` | `1.12.1` | Zlib OR Apache-2.0 OR MIT | [https://github.com/Lokathor/bytemuck](https://github.com/Lokathor/bytemuck) |
| `byteorder-lite` | `0.1.0` | Unlicense OR MIT | [https://github.com/image-rs/byteorder-lite](https://github.com/image-rs/byteorder-lite) |
| `cc` | `1.11.2` | MIT OR Apache-2.0 | [https://github.com/rust-lang/cc-rs](https://github.com/rust-lang/cc-rs) |
| `cfg-if` | `1.0.5` | MIT OR Apache-2.0 | [https://github.com/rust-lang/cfg-if](https://github.com/rust-lang/cfg-if) |
| `clap` | `4.6.8` | MIT OR Apache-2.0 | [https://github.com/clap-rs/clap](https://github.com/clap-rs/clap) |
| `clap_builder` | `4.6.7` | MIT OR Apache-2.0 | [https://github.com/clap-rs/clap](https://github.com/clap-rs/clap) |
| `clap_derive` | `4.6.8` | MIT OR Apache-2.0 | [https://github.com/clap-rs/clap](https://github.com/clap-rs/clap) |
| `clap_lex` | `1.1.2` | MIT OR Apache-2.0 | [https://github.com/clap-rs/clap](https://github.com/clap-rs/clap) |
| `colorchoice` | `1.0.5` | MIT OR Apache-2.0 | [https://github.com/rust-cli/anstyle.git](https://github.com/rust-cli/anstyle.git) |
| `crc32fast` | `1.5.5` | MIT OR Apache-2.0 | [https://github.com/srijs/rust-crc32fast](https://github.com/srijs/rust-crc32fast) |
| `crossbeam-deque` | `0.8.8` | MIT OR Apache-2.0 | [https://github.com/crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) |
| `crossbeam-epoch` | `0.9.21` | MIT OR Apache-2.0 | [https://github.com/crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) |
| `crossbeam-utils` | `0.8.23` | MIT OR Apache-2.0 | [https://github.com/crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) |
| `either` | `1.19.0` | MIT OR Apache-2.0 | [https://github.com/rayon-rs/either](https://github.com/rayon-rs/either) |
| `enough` | `0.4.5` | MIT OR Apache-2.0 | [https://github.com/imazen/enough](https://github.com/imazen/enough) |
| `equator` | `0.4.2` | MIT | [https://github.com/sarah-ek/equator/](https://github.com/sarah-ek/equator/) |
| `equator-macro` | `0.4.2` | MIT | [https://github.com/sarah-ek/equator/](https://github.com/sarah-ek/equator/) |
| `equivalent` | `1.0.3` | Apache-2.0 OR MIT | [https://github.com/indexmap-rs/equivalent](https://github.com/indexmap-rs/equivalent) |
| `fdeflate` | `0.3.7` | MIT OR Apache-2.0 | [https://github.com/image-rs/fdeflate](https://github.com/image-rs/fdeflate) |
| `find-msvc-tools` | `0.1.15` | MIT OR Apache-2.0 | [https://github.com/rust-lang/cc-rs](https://github.com/rust-lang/cc-rs) |
| `flate2` | `1.1.11` | MIT OR Apache-2.0 | [https://github.com/rust-lang/flate2-rs](https://github.com/rust-lang/flate2-rs) |
| `foldhash` | `0.2.0` | Zlib | [https://github.com/orlp/foldhash](https://github.com/orlp/foldhash) |
| `garb` | `0.2.8` | MIT OR Apache-2.0 | [https://github.com/imazen/garb](https://github.com/imazen/garb) |
| `getrandom` | `0.3.4` | MIT OR Apache-2.0 | [https://github.com/rust-random/getrandom](https://github.com/rust-random/getrandom) |
| `getrandom` | `0.4.3` | MIT OR Apache-2.0 | [https://github.com/rust-random/getrandom](https://github.com/rust-random/getrandom) |
| `hashbrown` | `0.16.1` | MIT OR Apache-2.0 | [https://github.com/rust-lang/hashbrown](https://github.com/rust-lang/hashbrown) |
| `hashbrown` | `0.17.1` | MIT OR Apache-2.0 | [https://github.com/rust-lang/hashbrown](https://github.com/rust-lang/hashbrown) |
| `heck` | `0.5.0` | MIT OR Apache-2.0 | [https://github.com/withoutboats/heck](https://github.com/withoutboats/heck) |
| `image` | `0.25.10` | MIT OR Apache-2.0 | [https://github.com/image-rs/image](https://github.com/image-rs/image) |
| `imgref` | `1.12.3` | CC0-1.0 OR Apache-2.0 | [https://github.com/kornelski/imgref](https://github.com/kornelski/imgref) |
| `indexmap` | `2.14.2` | Apache-2.0 OR MIT | [https://github.com/indexmap-rs/indexmap](https://github.com/indexmap-rs/indexmap) |
| `inotify` | `0.11.5` | ISC | [https://github.com/hannobraun/inotify-rs](https://github.com/hannobraun/inotify-rs) |
| `inotify-sys` | `0.1.8` | ISC | [https://github.com/hannobraun/inotify-sys](https://github.com/hannobraun/inotify-sys) |
| `interpolate_name` | `0.2.4` | MIT | [https://github.com/lu-zero/interpolate_name](https://github.com/lu-zero/interpolate_name) |
| `is_terminal_polyfill` | `1.70.2` | MIT OR Apache-2.0 | [https://github.com/polyfill-rs/is_terminal_polyfill](https://github.com/polyfill-rs/is_terminal_polyfill) |
| `itertools` | `0.14.0` | MIT OR Apache-2.0 | [https://github.com/rust-itertools/itertools](https://github.com/rust-itertools/itertools) |
| `itoa` | `1.0.19` | MIT OR Apache-2.0 | [https://github.com/dtolnay/itoa](https://github.com/dtolnay/itoa) |
| `jobserver` | `0.1.35` | MIT OR Apache-2.0 | [https://github.com/rust-lang/jobserver-rs](https://github.com/rust-lang/jobserver-rs) |
| `kqueue` | `1.2.1` | MIT | [https://github.com/olislack/kqueue-rs](https://github.com/olislack/kqueue-rs) |
| `kqueue-sys` | `1.1.2` | MIT | [https://github.com/olislack/kqueue-rs](https://github.com/olislack/kqueue-rs) |
| `libc` | `0.2.191` | MIT OR Apache-2.0 | [https://github.com/rust-lang/libc](https://github.com/rust-lang/libc) |
| `libfuzzer-sys` | `0.4.13` | (MIT OR Apache-2.0) AND NCSA | [https://github.com/rust-fuzz/libfuzzer](https://github.com/rust-fuzz/libfuzzer) |
| `libm` | `0.2.16` | MIT | [https://github.com/rust-lang/compiler-builtins](https://github.com/rust-lang/compiler-builtins) |
| `linear-srgb` | `0.6.12` | MIT OR Apache-2.0 | [https://github.com/imazen/linear-srgb](https://github.com/imazen/linear-srgb) |
| `log` | `0.4.35` | MIT OR Apache-2.0 | [https://github.com/rust-lang/log](https://github.com/rust-lang/log) |
| `loop9` | `0.1.5` | MIT | [https://gitlab.com/kornelski/loop9.git](https://gitlab.com/kornelski/loop9.git) |
| `magetypes` | `0.9.31` | MIT OR Apache-2.0 | [https://github.com/imazen/archmage](https://github.com/imazen/archmage) |
| `maybe-rayon` | `0.1.1` | MIT | [https://github.com/shssoichiro/maybe-rayon](https://github.com/shssoichiro/maybe-rayon) |
| `memchr` | `2.8.3` | Unlicense OR MIT | [https://github.com/BurntSushi/memchr](https://github.com/BurntSushi/memchr) |
| `miniz_oxide` | `0.8.9` | MIT OR Zlib OR Apache-2.0 | [https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) |
| `miniz_oxide` | `0.9.1` | MIT OR Zlib OR Apache-2.0 | [https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) |
| `mio` | `1.2.5` | MIT | [https://github.com/tokio-rs/mio](https://github.com/tokio-rs/mio) |
| `moxcms` | `0.8.1` | BSD-3-Clause OR Apache-2.0 | [https://github.com/awxkee/moxcms.git](https://github.com/awxkee/moxcms.git) |
| `nasm-rs` | `0.3.2` | MIT OR Apache-2.0 | [https://github.com/medek/nasm-rs](https://github.com/medek/nasm-rs) |
| `new_debug_unreachable` | `1.0.6` | MIT | [https://github.com/mbrubeck/rust-debug-unreachable](https://github.com/mbrubeck/rust-debug-unreachable) |
| `no_std_io2` | `0.9.4` | Apache-2.0 OR MIT | [https://github.com/wcampbell0x2a/no-std-io2](https://github.com/wcampbell0x2a/no-std-io2) |
| `nom` | `8.0.0` | MIT | [https://github.com/rust-bakery/nom](https://github.com/rust-bakery/nom) |
| `noop_proc_macro` | `0.3.0` | MIT | [https://github.com/lu-zero/noop_proc_macro](https://github.com/lu-zero/noop_proc_macro) |
| `notify` | `8.2.2` | CC0-1.0 | [https://github.com/notify-rs/notify.git](https://github.com/notify-rs/notify.git) |
| `notify-types` | `2.3.1` | MIT OR Apache-2.0 | [https://github.com/notify-rs/notify.git](https://github.com/notify-rs/notify.git) |
| `num-bigint` | `0.4.8` | MIT OR Apache-2.0 | [https://github.com/rust-num/num-bigint](https://github.com/rust-num/num-bigint) |
| `num-derive` | `0.4.2` | MIT OR Apache-2.0 | [https://github.com/rust-num/num-derive](https://github.com/rust-num/num-derive) |
| `num-integer` | `0.1.47` | MIT OR Apache-2.0 | [https://github.com/rust-num/num-integer](https://github.com/rust-num/num-integer) |
| `num-rational` | `0.4.2` | MIT OR Apache-2.0 | [https://github.com/rust-num/num-rational](https://github.com/rust-num/num-rational) |
| `num-traits` | `0.2.19` | MIT OR Apache-2.0 | [https://github.com/rust-num/num-traits](https://github.com/rust-num/num-traits) |
| `objc2-core-foundation` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | [https://github.com/madsmtm/objc2](https://github.com/madsmtm/objc2) |
| `objc2-core-services` | `0.3.2` | Zlib OR Apache-2.0 OR MIT | [https://github.com/madsmtm/objc2](https://github.com/madsmtm/objc2) |
| `once_cell_polyfill` | `1.70.2` | MIT OR Apache-2.0 | [https://github.com/polyfill-rs/once_cell_polyfill](https://github.com/polyfill-rs/once_cell_polyfill) |
| `paste` | `1.0.15` | MIT OR Apache-2.0 | [https://github.com/dtolnay/paste](https://github.com/dtolnay/paste) |
| `pastey` | `0.1.1` | MIT OR Apache-2.0 | [https://github.com/as1100k/pastey](https://github.com/as1100k/pastey) |
| `png` | `0.18.1` | MIT OR Apache-2.0 | [https://github.com/image-rs/image-png](https://github.com/image-rs/image-png) |
| `ppv-lite86` | `0.2.21` | MIT OR Apache-2.0 | [https://github.com/cryptocorrosion/cryptocorrosion](https://github.com/cryptocorrosion/cryptocorrosion) |
| `proc-macro2` | `1.0.107` | MIT OR Apache-2.0 | [https://github.com/dtolnay/proc-macro2](https://github.com/dtolnay/proc-macro2) |
| `profiling` | `1.0.18` | MIT OR Apache-2.0 | [https://github.com/aclysma/profiling](https://github.com/aclysma/profiling) |
| `profiling-procmacros` | `1.0.18` | MIT OR Apache-2.0 | [https://github.com/aclysma/profiling](https://github.com/aclysma/profiling) |
| `pxfm` | `0.1.30` | BSD-3-Clause OR Apache-2.0 | [https://github.com/awxkee/pxfm](https://github.com/awxkee/pxfm) |
| `quick-error` | `2.0.1` | MIT/Apache-2.0 | [http://github.com/tailhook/quick-error](http://github.com/tailhook/quick-error) |
| `quote` | `1.0.47` | MIT OR Apache-2.0 | [https://github.com/dtolnay/quote](https://github.com/dtolnay/quote) |
| `r-efi` | `5.3.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | [https://github.com/r-efi/r-efi](https://github.com/r-efi/r-efi) |
| `r-efi` | `6.0.0` | MIT OR Apache-2.0 OR LGPL-2.1-or-later | [https://github.com/r-efi/r-efi](https://github.com/r-efi/r-efi) |
| `rand` | `0.9.5` | MIT OR Apache-2.0 | [https://github.com/rust-random/rand](https://github.com/rust-random/rand) |
| `rand_chacha` | `0.9.0` | MIT OR Apache-2.0 | [https://github.com/rust-random/rand](https://github.com/rust-random/rand) |
| `rand_core` | `0.9.5` | MIT OR Apache-2.0 | [https://github.com/rust-random/rand](https://github.com/rust-random/rand) |
| `rav1e` | `0.8.1` | BSD-2-Clause | [https://github.com/xiph/rav1e/](https://github.com/xiph/rav1e/) |
| `ravif` | `0.13.0` | BSD-3-Clause | [https://github.com/kornelski/cavif-rs](https://github.com/kornelski/cavif-rs) |
| `rayon` | `1.12.0` | MIT OR Apache-2.0 | [https://github.com/rayon-rs/rayon](https://github.com/rayon-rs/rayon) |
| `rayon-core` | `1.13.0` | MIT OR Apache-2.0 | [https://github.com/rayon-rs/rayon](https://github.com/rayon-rs/rayon) |
| `rgb` | `0.8.53` | MIT | [https://github.com/kornelski/rust-rgb](https://github.com/kornelski/rust-rgb) |
| `rustversion` | `1.0.23` | MIT OR Apache-2.0 | [https://github.com/dtolnay/rustversion](https://github.com/dtolnay/rustversion) |
| `safe_unaligned_simd` | `0.2.5` | MIT OR Apache-2.0 | [https://github.com/okaneco/safe_unaligned_simd](https://github.com/okaneco/safe_unaligned_simd) |
| `same-file` | `1.0.6` | Unlicense/MIT | [https://github.com/BurntSushi/same-file](https://github.com/BurntSushi/same-file) |
| `serde` | `1.0.229` | MIT OR Apache-2.0 | [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde) |
| `serde_core` | `1.0.229` | MIT OR Apache-2.0 | [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde) |
| `serde_derive` | `1.0.229` | MIT OR Apache-2.0 | [https://github.com/serde-rs/serde](https://github.com/serde-rs/serde) |
| `serde_json` | `1.0.154` | MIT OR Apache-2.0 | [https://github.com/serde-rs/json](https://github.com/serde-rs/json) |
| `serde_spanned` | `1.1.3` | MIT OR Apache-2.0 | [https://github.com/toml-rs/toml](https://github.com/toml-rs/toml) |
| `shlex` | `2.0.1` | MIT OR Apache-2.0 | [https://github.com/comex/rust-shlex](https://github.com/comex/rust-shlex) |
| `simd-adler32` | `0.3.10` | MIT | [https://github.com/mcountryman/simd-adler32](https://github.com/mcountryman/simd-adler32) |
| `simd_helpers` | `0.1.0` | MIT | [https://github.com/lu-zero/simd_helpers](https://github.com/lu-zero/simd_helpers) |
| `stable_deref_trait` | `1.2.1` | MIT OR Apache-2.0 | [https://github.com/storyyeller/stable_deref_trait](https://github.com/storyyeller/stable_deref_trait) |
| `strsim` | `0.11.1` | MIT | [https://github.com/rapidfuzz/strsim-rs](https://github.com/rapidfuzz/strsim-rs) |
| `syn` | `2.0.119` | MIT OR Apache-2.0 | [https://github.com/dtolnay/syn](https://github.com/dtolnay/syn) |
| `syn` | `3.0.10` | MIT OR Apache-2.0 | [https://github.com/dtolnay/syn](https://github.com/dtolnay/syn) |
| `thiserror` | `2.0.21` | MIT OR Apache-2.0 | [https://github.com/dtolnay/thiserror](https://github.com/dtolnay/thiserror) |
| `thiserror-impl` | `2.0.21` | MIT OR Apache-2.0 | [https://github.com/dtolnay/thiserror](https://github.com/dtolnay/thiserror) |
| `toml` | `0.9.12+spec-1.1.0` | MIT OR Apache-2.0 | [https://github.com/toml-rs/toml](https://github.com/toml-rs/toml) |
| `toml_datetime` | `0.7.5+spec-1.1.0` | MIT OR Apache-2.0 | [https://github.com/toml-rs/toml](https://github.com/toml-rs/toml) |
| `toml_parser` | `1.1.5+spec-1.1.0` | MIT OR Apache-2.0 | [https://github.com/toml-rs/toml](https://github.com/toml-rs/toml) |
| `toml_writer` | `1.2.0+spec-1.1.0` | MIT OR Apache-2.0 | [https://github.com/toml-rs/toml](https://github.com/toml-rs/toml) |
| `unicode-ident` | `1.0.27` | (MIT OR Apache-2.0) AND Unicode-3.0 | [https://github.com/dtolnay/unicode-ident](https://github.com/dtolnay/unicode-ident) |
| `utf8parse` | `0.2.2` | Apache-2.0 OR MIT | [https://github.com/alacritty/vte](https://github.com/alacritty/vte) |
| `v_frame` | `0.3.9` | BSD-2-Clause | [https://github.com/rust-av/v_frame](https://github.com/rust-av/v_frame) |
| `walkdir` | `2.5.0` | Unlicense/MIT | [https://github.com/BurntSushi/walkdir](https://github.com/BurntSushi/walkdir) |
| `wasi` | `0.11.1+wasi-snapshot-preview1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [https://github.com/bytecodealliance/wasi](https://github.com/bytecodealliance/wasi) |
| `wasip2` | `1.0.4+wasi-0.2.12` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [https://github.com/bytecodealliance/wasi-rs](https://github.com/bytecodealliance/wasi-rs) |
| `wasm-bindgen` | `0.2.130` | MIT OR Apache-2.0 | [https://github.com/wasm-bindgen/wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) |
| `wasm-bindgen-macro` | `0.2.130` | MIT OR Apache-2.0 | [https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro](https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro) |
| `wasm-bindgen-macro-support` | `0.2.130` | MIT OR Apache-2.0 | [https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support](https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support) |
| `wasm-bindgen-shared` | `0.2.130` | MIT OR Apache-2.0 | [https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared](https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared) |
| `whereat` | `0.1.5` | MIT OR Apache-2.0 | [https://github.com/lilith/whereat](https://github.com/lilith/whereat) |
| `winapi-util` | `0.1.11` | Unlicense OR MIT | [https://github.com/BurntSushi/winapi-util](https://github.com/BurntSushi/winapi-util) |
| `winarm-cpufeatures` | `0.1.2` | MIT OR Apache-2.0 | [https://github.com/imazen/winarm-cpufeatures](https://github.com/imazen/winarm-cpufeatures) |
| `windows-link` | `0.2.1` | MIT OR Apache-2.0 | [https://github.com/microsoft/windows-rs](https://github.com/microsoft/windows-rs) |
| `windows-sys` | `0.61.2` | MIT OR Apache-2.0 | [https://github.com/microsoft/windows-rs](https://github.com/microsoft/windows-rs) |
| `winnow` | `0.7.15` | MIT | [https://github.com/winnow-rs/winnow](https://github.com/winnow-rs/winnow) |
| `winnow` | `1.0.5` | MIT | [https://github.com/winnow-rs/winnow](https://github.com/winnow-rs/winnow) |
| `wit-bindgen` | `0.57.1` | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | [https://github.com/bytecodealliance/wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) |
| `y4m` | `0.8.0` | MIT | [https://github.com/image-rs/y4m.git](https://github.com/image-rs/y4m.git) |
| `zenpixels` | `0.2.16` | Apache-2.0 OR MIT | [https://github.com/imazen/zenpixels](https://github.com/imazen/zenpixels) |
| `zenwebp` | `0.4.6` | AGPL-3.0-only OR LicenseRef-Imazen-Commercial | [https://github.com/imazen/zenwebp](https://github.com/imazen/zenwebp) |
| `zenyuv` | `0.1.3` | MIT OR Apache-2.0 | [https://github.com/imazen/zenjpeg](https://github.com/imazen/zenjpeg) |
| `zerocopy` | `0.8.63` | BSD-2-Clause OR Apache-2.0 OR MIT | [https://github.com/google/zerocopy](https://github.com/google/zerocopy) |
| `zerocopy-derive` | `0.8.63` | BSD-2-Clause OR Apache-2.0 OR MIT | [https://github.com/google/zerocopy](https://github.com/google/zerocopy) |
| `zlib-rs` | `0.6.8` | Zlib | [https://github.com/trifectatechfoundation/zlib-rs](https://github.com/trifectatechfoundation/zlib-rs) |
| `zmij` | `1.0.23` | MIT | [https://github.com/dtolnay/zmij](https://github.com/dtolnay/zmij) |
| `zune-core` | `0.5.3` | MIT OR Apache-2.0 OR Zlib | [https://github.com/etemesi254/zune-image](https://github.com/etemesi254/zune-image) |
| `zune-jpeg` | `0.5.15` | MIT OR Apache-2.0 OR Zlib | [https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg](https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg) |
