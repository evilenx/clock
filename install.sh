#!/usr/bin/env bash
set -e

# Detect user's shell and select appropriate profile file
detect_profile() {
    if [ -n "$ZSH_VERSION" ]; then
        echo "$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bash_profile" ]; then
            echo "$HOME/.bash_profile"
        else
            echo "$HOME/.bashrc"
        fi
    else
        # Default fallback
        echo "$HOME/.profile"
    fi
}

PROFILE_FILE=$(detect_profile)

# Check and install cargo/rustup if missing
if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found. Installing Rust toolchain via rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$PROFILE_FILE"
fi

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

# Ensure the cargo bin path is available for this session
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi

echo "clock was installed successfully."
echo "Restart your terminal or run 'source $PROFILE_FILE' to update your PATH."

