FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    file \
    git \
    gnupg2 \
    jq \
    libayatana-appindicator3-dev \
    libgtk-3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    locales \
    openssl \
    pkg-config \
    sudo \
    wget \
    zsh

RUN sed -i 's/^# *\(en_US.UTF-8\)/\1/' /etc/locale.gen && \
    locale-gen && \
    echo "export LC_ALL=en_US.UTF-8" >> /etc/profile && \
    echo "export LANG=en_US.UTF-8" >> /etc/profile && \
    echo "export LANGUAGE=en_US.UTF-8" >> /etc/profile

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" && \
    chown -R root:root /root/.oh-my-zsh

RUN rustup install nightly && \
    rustup component add rustfmt clippy && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

RUN cargo install trunk && \
    cargo install tauri-cli
