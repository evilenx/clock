#!/usr/bin/env bash
set -e
command -v cargo >/dev/null || { echo "cargo not found"; exit 1; }
TEMP_DIR=$(mktemp -d)
git clone --depth 1 https://github.com/evilenx/clock.git "$TEMP_DIR"
cd "$TEMP_DIR"
cargo build --release
mkdir -p ~/.config/clock

if [ ! -f ~/.config/clock/config.toml ]; then
    cat > ~/.config/clock/config.toml << EOF
[settings]
font_size = 80
padding = 20.0
auto_resize = true
EOF
fi

mkdir -p ~/.cargo/bin
cp target/release/clock ~/.cargo/bin/clock
rm -rf "$TEMP_DIR"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bash_profile
source ~/.bash_profile
