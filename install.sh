#!/usr/bin/env bash
set -e
command -v cargo >/dev/null || { echo "cargo not found"; exit 1; }
TEMP_DIR=$(mktemp -d)
git clone --depth 1 https://github.com/evilenx/clock.git "$TEMP_DIR"
cd "$TEMP_DIR"
cargo build --release
mkdir -p ~/.config/clock
[ -f ~/.config/clock/config.yml ] || echo "font_size: 80" > ~/.config/clock/config.yml
mkdir -p ~/.cargo/bin
cp target/release/clock ~/.cargo/bin/clock
rm -rf "$TEMP_DIR"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bash_profile
source ~/.bash_profile



