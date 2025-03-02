@echo on

rustup install 1.70.0 || exit /b 1
cargo +1.70.0 build --release || exit /b 1
