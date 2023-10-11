# breedenter-rust
A rust version of breedenter, automatically start browser http://192.168.1.1/ when breed is entered.

![](https://img.shields.io/badge/license-MIT-000000.svg)
![](https://img.shields.io/badge/language-rust-brightgreen)
[![Build](https://github.com/wwng2333/breed-enter-rust/actions/workflows/build-gnu.yaml/badge.svg)](https://github.com/wwng2333/breed-enter-rust/actions/workflows/build-gnu.yaml)
[![Build](https://github.com/wwng2333/breed-enter-rust/actions/workflows/build-msvc.yaml/badge.svg)](https://github.com/wwng2333/breed-enter-rust/actions/workflows/build-msvc.yaml)

[点我进入下载页面](https://github.com/wwng2333/breed-enter-rust/releases)
# Compile with GNU toolchain
```bash
git clone https://github.com/wwng2333/breed-enter-rust.git
cd breed-enter-rust
cargo build --release
```
# Compile with MSVC toolchain
```
cargo build --target=x86_64-pc-windows-msvc --release
   Compiling windows_x86_64_msvc v0.42.1
   Compiling windows-sys v0.42.0
   Compiling open v3.2.0
   Compiling breedenter-rust v0.1.5 (D:\Projects\rust\breedenter-rust)
    Finished release [optimized] target(s) in 4.28s
```
# Usage
Set your PC IP to `192.168.1.2/255.255.255.0`.

Download it, and execute it.
```
[root@crazy breed-enter-rust]# cargo run --package breedenter-rust
   Compiling breedenter-rust v0.1.5 (D:\Projects\rust\breedenter-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target\debug\breedenter-rust.exe`
Sending abort command to breed per 500ms.
Received pong from breed, starting browser.
Press any key to continue...
```

# License
MIT
