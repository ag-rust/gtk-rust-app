FROM manjaroarm/manjaro-aarch64-base
RUN pacman -Syu base-devel gtk4 libadwaita --noconfirm && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    source $HOME/.cargo/env && \
    cargo install cocogitto --locked && \
    cargo install cargo-outdated
RUN source $HOME/.cargo/env
