FROM registry.gitlab.com/loers/gtk-rust-app/aarch64-0:main
RUN cargo install cargo-outdated --locked && \
    cargo install cargo-bump
