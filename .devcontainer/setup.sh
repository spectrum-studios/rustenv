# apt-get
apt-get update
apt-get install -y \
    build-essential \
    curl \
    file \
    git \
    gnupg2 \
    jq \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    openssl \
    pkg-config \
    sudo \
    wget \
    zsh

# zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
chown -R $USER_UID:$USER_GID /root/.oh-my-zsh /root

# rustup
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
rustup component add clippy --toolchain nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

# cargo
cargo install trunk
cargo install tauri-cli
