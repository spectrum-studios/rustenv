FROM rust:latest

WORKDIR /usr/src/app

COPY . .

# apt-get
RUN apt-get update
RUN apt-get install -y \
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
    locales \
    openssl \
    pkg-config \
    sudo \
    wget \
    zsh

# locale
RUN sed -i 's/^# *\(en_US.UTF-8\)/\1/' /etc/locale.gen
RUN locale-gen
RUN echo "export LC_ALL=en_US.UTF-8" >> ~/.bashrc
RUN echo "export LANG=en_US.UTF-8" >> ~/.bashrc
RUN echo "export LANGUAGE=en_US.UTF-8" >> ~/.bashrc

# zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
RUN chown -R $USER_UID:$USER_GID /root/.oh-my-zsh /root

# rustup
RUN rustup install nightly
RUN rustup component add rustfmt
RUN rustup component add rustfmt --toolchain nightly
RUN rustup component add clippy
RUN rustup component add clippy --toolchain nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

# cargo
RUN cargo install trunk
RUN cargo install tauri-cli
