# Change these variables according to your project name
URI_PREFIX=org.project
PKG_NAME=App
BIN_NAME=app

base_dir=$(realpath .)
flatpak_build_dir=$(base_dir)/.flatpak

.PHONY: clean prepare debug build_aarch64

build:
	@echo Building in $(flatpak_build_dir)/host
	@flatpak build \
		--share=network \
		--filesystem=$(flatpak_build_dir)/host \
		--env=PATH=$$PATH:/usr/lib/sdk/rust-stable/bin \
		--env=CARGO_HOME=/run/build/$(PKG_NAME)/cargo \
        --env=RUST_BACKTRACE=1 \
		$(flatpak_build_dir)/host \
		cargo build

debug:
	@flatpak build \
		--with-appdir \
		--allow=devel \
		--socket=fallback-x11 \
		--socket=wayland \
		--device=dri \
		--talk-name=org.a11y.Bus \
		--env=RUST_LOG=$(BIN_NAME)=debug \
		--env=G_MESSAGES_DEBUG=none \
		--env=RUST_BACKTRACE=1 \
		--talk-name='org.freedesktop.portal.*' \
		--talk-name=org.a11y.Bus \
		--env=XDG_SESSION_TYPE=x11 \
		--env=LANG=en_US.UTF-8 \
		--env=COLORTERM=truecolor \
		$(flatpak_build_dir)/host \
		target/debug/$(BIN_NAME)

build_aarch64:
	@echo Building in $(flatpak_build_dir)/aarch64
	@flatpak build \
		--share=network \
		--filesystem=$(flatpak_build_dir)/aarch64 \
		--env=PATH=$$PATH:/usr/lib/sdk/rust-stable/bin \
		--env=CARGO_HOME=/run/build/$(PKG_NAME)/cargo \
        --env=RUST_BACKTRACE=1 \
		--env=CARGO_TARGET_DIR=target/aarch64 \
		$(flatpak_build_dir)/aarch64 \
		ls
		# cargo build --release

clean:
	@cargo clean
	@rm -rf .flatpak
	@flatpak build-init \
		--sdk-extension=org.freedesktop.Sdk.Extension.rust-stable \
		$(flatpak_build_dir)/host \
		$(URI_PREFIX).$(PKG_NAME) \
		org.gnome.Sdk \
		org.gnome.Platform \
		40
	@flatpak build-init \
		--sdk-extension=org.freedesktop.Sdk.Extension.rust-stable \
		$(flatpak_build_dir)/aarch64 \
		$(URI_PREFIX).$(PKG_NAME) \
		org.gnome.Sdk/aarch64 \
		org.gnome.Platform/aarch64 \
		40

build_deps:
	flatpak-builder \
		--ccache \
		--force-clean \
		--disable-updates \
		--disable-download \
		--build-only \
		--keep-build-dirs \
		--state-dir=$(base_dir)/target/.flatpak-builder \
		--stop-at=$(BIN_NAME) \
		$(flatpak_build_dir)/.flatpak/host \
		$(base_dir)/data/$(URI_PREFIX).$(PKG_NAME).yml

prepare:
	flatpak install flathub org.gnome.Sdk//40
	flatpak install flathub org.gnome.Platform//40
	flatpak install flathub org.gnome.Sdk/aarch64/40
	flatpak install flathub org.gnome.Platform/aarch64/40
	flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//20.08
