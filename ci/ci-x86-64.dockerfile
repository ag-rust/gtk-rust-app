FROM manjarolinux/base
RUN pacman -Syu base-devel gtk4 libadwaita --noconfirm && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    source $HOME/.cargo/env
ENV PATH="$HOME/.cargo/bin:$PATH"
