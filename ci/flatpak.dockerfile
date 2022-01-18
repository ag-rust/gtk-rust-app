FROM registry.gitlab.com/loers/gtk-rust-app/x86-64:main
RUN pacman -Syu wget flatpak flatpak-builder --noconfirm && \
	wget https://nightly.gnome.org/gnome-nightly.flatpakrepo -P target/ && \
    flatpak remote-add --if-not-exists gnome-nightly target/gnome-nightly.flatpakrepo && \
	rm target/gnome-nightly.flatpakrepo && \
	flatpak install org.gnome.Sdk//master -y && \
	flatpak install org.gnome.Platform//master -y && \
	flatpak install org.freedesktop.Sdk.Extension.rust-stable//21.08 -y
