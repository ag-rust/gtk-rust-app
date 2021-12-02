clean:
	rm -rf out && rm -rf target/flatpak-temp && rm -rf target/flatpak-build && rm -f Cargo.lock && make build-aux

flatpak:
	make -C out .prepare-flatpak
	make -C out flatpak

flatpak-arm64:
	make -C out .prepare-flatpak
	make -C out flatpak-arm64

release:
	echo release

install-gsettings:
	make -C out .install-gsettings

uninstall-gsettings:
	make -C out .uninstall-gsettings

build-aux:
	rm -rf out
	cargo build --bin build-aux --features build-aux --no-default-features
	./target/debug/build-aux
	@echo Done

setup:
	wget https://nightly.gnome.org/gnome-nightly.flatpakrepo -P target/
	flatpak remote-add --if-not-exists gnome-nightly target/gnome-nightly.flatpakrepo
	rm target/gnome-nightly.flatpakrepo
	flatpak install org.gnome.Sdk//master -y
	flatpak install org.gnome.Sdk/aarch64/master -y
	flatpak install org.gnome.Platform//master -y
	flatpak install org.gnome.Platform/aarch64/master -y
	flatpak install org.freedesktop.Sdk.Extension.rust-stable//21.08 -y
	flatpak install org.freedesktop.Sdk.Extension.rust-stable/aarch64/21.08 -y