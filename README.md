# rust_audio_dsp

A small **real-time audio DSP module** written in Rust and exported via **C FFI** — to be used inside audio plugin frameworks like **JUCE**, **VST3**, or **CLAP**.

## What does it do?

Implements a simple **low-shelf EQ filter** using the [`biquad`](https://crates.io/crates/biquad) crate.

You can:

✅ Create and configure an EQ filter  
✅ Process audio buffers through the filter  
✅ Destroy the filter instance  

## Why?

- Matches typical **audio SDK work**  
- Demonstrates **Rust in real-time audio**  
- Safe Rust core, FFI boundary  
- Can be linked into C++ plugin projects (JUCE, etc.)

## Build

```bash
cargo build --release
