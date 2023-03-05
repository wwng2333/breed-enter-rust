set "RUSTFLAGS=-D warnings"
set "RUSTFMT_CI=1"

:: Print version information
rustc -Vv || exit /b 1
cargo -V || exit /b 1

:: Build
cargo build --release || exit /b 1
dir || exit /b 1
tree || exit /b 1
ls D:\a\breed-enter-rust\breed-enter-rust\target\x86_64-pc-windows-msvc\release || exit /b 1