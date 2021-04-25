xargo rustc --release --target i686-pc-windows-msvc -- --verbose $@
ls -ltr target/i686-pc-windows-msvc/release/

# Troubleshooting
#
# Nightly: rustup default nightly
