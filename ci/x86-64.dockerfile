FROM manjarolinux/base
RUN pacman -Syu base-devel gtk4 libadwaita wget --noconfirm && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    source $HOME/.cargo/env && \
    cargo install cocogitto --locked && \
    cargo install cargo-outdated --locked && \
    cargo install cargo-bump --locked && \
    cargo install cargo-gra --locked
