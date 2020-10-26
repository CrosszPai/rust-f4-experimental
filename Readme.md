# Embedded Rust Experimental
STM32F411CEU6 (Blackpill) with embedded graphic + ST7735 display

## Required List
### target
`rustup target add thumbv7em-none-eabihf`

### cargo
- [cargo-flash](https://github.com/probe-rs/cargo-flash)

### programmer
- STLINK V2

## Flash
`cargo flash --chip STM32F411CE --release`

![Imgur](./ferris.gif)