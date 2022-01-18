BASE_DIR=$(realpath ../..)

# The name of the rust binary
BIN_NAME=$(shell sed -n 's/name = "\(.*\)"/\1/p' $(BASE_DIR)/Cargo.toml | head -n1)
# The app id (e.g. org.example.MyApp)
APP_ID=$(shell sed -n 's/id = "\(.*\)"/\1/p' $(BASE_DIR)/Cargo.toml | head -n1)

# The rust target directory
TARGET_DIR=$(BASE_DIR)/target
# The gtk-rust-app out directory
GEN_DIR=$(BASE_DIR)/target/gra-gen

# The binary directory inside the flatpak container
FLATPAK_BIN_DIR=/app/bin
# The share directory inside the flatpak container
FLATPAK_SHARE_DIR=/app/share

.PHONY: .flatpak-install

# Build a release binary and install everything in the flaAPP_IDtpak container
.flatpak-install:
	@# Install binary
	@cargo build --release --features libadwaita-1 
	@mkdir -p $(FLATPAK_BIN_DIR)
	@install $(TARGET_DIR)/release/$(BIN_NAME) $(FLATPAK_BIN_DIR)/$(APP_ID)

	@# Install settings
	@mkdir -p $(FLATPAK_SHARE_DIR)/glib-2.0/schemas/
	@install -m 644 -D $(GEN_DIR)/${APP_ID}.gschema.xml $(FLATPAK_SHARE_DIR)/glib-2.0/schemas/
	@glib-compile-schemas $(FLATPAK_SHARE_DIR)/glib-2.0/schemas/

	@# Install icons
	@mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/scalable/apps
	@mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/64x64/apps
	@mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/128x128/apps
	@install -m 644 $(GEN_DIR)/data/$(APP_ID).svg $(FLATPAK_SHARE_DIR)/icons/hicolor/scalable/apps/$(APP_ID).svg
	@install -m 644 $(GEN_DIR)/data/$(APP_ID).64.png $(FLATPAK_SHARE_DIR)/icons/hicolor/64x64/apps/$(APP_ID).png
	@install -m 644 $(GEN_DIR)/data/$(APP_ID).128.png $(FLATPAK_SHARE_DIR)/icons/hicolor/128x128/apps/$(APP_ID).png
	@touch $(FLATPAK_SHARE_DIR)/icons/hicolor/

	@# Force icon cache to refresh
	@mkdir -p $(FLATPAK_SHARE_DIR)/applications/
	@mkdir -p $(FLATPAK_SHARE_DIR)/metainfo/

	@# Install appdata.xml and .desktop file
	@install -m 644 $(GEN_DIR)/data/$(APP_ID).desktop $(FLATPAK_SHARE_DIR)/applications/$(APP_ID).desktop
	@install -m 644 $(GEN_DIR)/data/$(APP_ID).appdata.xml $(FLATPAK_SHARE_DIR)/metainfo/$(APP_ID).appdata.xml
