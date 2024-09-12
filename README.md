# ![logo](https://github.com/rmk-firmware/rmk/blob/master/docs/public/logo-64x64.png?raw=true) RMK Firmware

RMK is a rust keyboard firmware built for ease.  
The main selling points are:
  - You can configure your keyboard directly through the keyboard's flash drive.
  - It’s fast and reliable, as it's built in Rust.
  - Adding your own keyboard is as simple as creating a single configuration file.
  - It supports a wide variety of chipsets through [embassy](https://github.com/embassy-rs/embassy).
    - all of the STM32 family, the nRF52, nRF53, nRF91, Raspberry Pi RP2040, ESP32 and the WCH 32-bit RISC-V(CH32V) series.


Here is an example keymap:  
```rmk
layer 0
____________________________________________________________________________________________________
press      | esc    q      w      e      r      t      y      u      i      o      p      =
shifted    | `      ---    ---    ---    ---    ---    ---    ---    ---    ---    ---    "::"
____________________________________________________________________________________________________
press      | tab    a      s      d      f      g      h      j      k      l             ent
shifted    | ---    ---    ---    ---    ---    ---    ---    ---    ---    ---           bspc
____________________________________________________________________________________________________
press      | lsft   z      x      c      v      b      n      m      ,             .      del
shifted    | ---    ---    ---    ---    ---    ---    ---    ---    !             ?      ---
____________________________________________________________________________________________________
press      | lctl   lgui   lalt          space         space                /      -      _
shifted    | ---    ---    ---           ---           ---                  \      +      _
held       | ---    ---    ---           ml(1)         ml(2)                ---    ---    ---
mac        | lgui   lctl   ---           ---           ---                  ---    ---    ---
```


## Prerequisits
install rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

install probe-rs
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

  
To get started, check out the 📖 [Docs](https://rmk-firmware.github.io/rmk).

If you need help, we have a friendly [discord server](https://discord.gg/SrESTtBKV5) for you.


## License

RMK is licensed under either of your choice

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
