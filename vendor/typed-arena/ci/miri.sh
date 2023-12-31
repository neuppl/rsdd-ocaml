set -ex

#MIRI_NIGHTLY=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri)
MIRI_NIGHTLY=nightly-2019-09-11
echo "Installing latest nightly with Miri: $MIRI_NIGHTLY"
rustup default "$MIRI_NIGHTLY"

cargo clean
rustup component add miri
cargo miri setup

cargo miri test
