# OVL Extractor 3.0 (Rust Edition)

This is a Rust reimplementation of the RCT3 tool OVL Extractor, originally written in C++. This version leverages Rust's memory safety guarantees and robust error handling to provide a more reliable and secure tool for working with RCT3 file formats.

## Improvements over 2.0
- Memory safety guaranteed by Rust's ownership system
- Comprehensive error handling using Result types
- Modern GUI using egui framework
- Cross-platform support
- Zero memory leaks by design

## Features
- Extract and analyze RCT3 OVL files
- View file sections and contents
- Safe binary data handling
- User-friendly interface

## Building
Requires Rust 2021 edition or later.

```bash
cargo build --release
```

## Usage
Run the executable and use the GUI to open and extract OVL files.

## Note
This tool is designed for experimenting with RCT3 file formats. While the original C++ version had memory leaks and error handling issues, this Rust implementation addresses these concerns through Rust's safety features.