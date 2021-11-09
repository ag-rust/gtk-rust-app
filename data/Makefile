BASE_DIR=$(realpath .)/..

BIN_NAME=$(shell sed -n 's/name = "\(.*\)"/\1/p' $(BASE_DIR)/Cargo.toml)
APP_ID=$(shell sed -n 's/id = "\(.*\)"/\1/p' $(BASE_DIR)/Cargo.toml)

TARGET_DIR=$(BASE_DIR)/target
OUT_DIR=$(BASE_DIR)/out
FLATPAK_BUILD_DIR=$(TARGET_DIR)/flatpak-build
FLATPAK_TEMP_DIR=$(BASE_DIR)/target/flatpak-temp

FLATPAK_BIN_DIR=/app/bin
FLATPAK_SHARE_DIR=/app/share

.PHONY: install-gsettings uninstall-gsettings vendor unvendor build clean flatpak-install 

# Dev targets
.install-gsettings:
	sudo install -D $(OUT_DIR)/${APP_ID}.gschema.xml /usr/share/glib-2.0/schemas/
	sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

.uninstall-gsettings:
	sudo rm /usr/share/glib-2.0/schemas/${APP_ID}.gschema.xml 
	sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

# Targets to build a flatpak app
.vendor:
	@mkdir -p .cargo
	@cargo vendor $(TARGET_DIR)/vendor > $(BASE_DIR).cargo/config.toml

.unvendor:
	@rm -rf .cargo

flat:
	@mkdir -p $(FLATPAK_TEMP_DIR)
	@mkdir -p $(FLATPAK_TEMP_DIR)/target/release
	@cp $(TARGET_DIR)/release/$(BIN_NAME) $(FLATPAK_TEMP_DIR)/target/release/
	@cp -r $(OUT_DIR) $(FLATPAK_TEMP_DIR)/
	@cp -r $(BASE_DIR)/src $(FLATPAK_TEMP_DIR)/
	@cp -r $(BASE_DIR)/Cargo.toml $(FLATPAK_TEMP_DIR)/
	@cp -r $(OUT_DIR)/Makefile $(FLATPAK_TEMP_DIR)/out/
	@mkdir -p $(FLATPAK_BUILD_DIR)/state/build/$(BIN_NAME)-1
	@flatpak-builder \
		--repo=$(FLATPAK_BUILD_DIR)/repo \
		$(FLATPAK_BUILD_DIR)/host \
		$(OUT_DIR)/data/$(APP_ID).host.yml \
		--force-clean \
		--keep-build-dirs \
		--state-dir=$(FLATPAK_BUILD_DIR)/state
	@flatpak build-bundle $(FLATPAK_BUILD_DIR)/repo $(TARGET_DIR)/$(BIN_NAME).flatpak $(APP_ID)
	@rm -rf $(FLATPAK_TEMP_DIR)
	@rm -rf $(FLATPAK_BUILD_DIR)/state

.clean:
	@rm -rf $(OUT_DIR)
	@rm -rf target

# Targets called within flatpak build
.flatpak-install:
	@# Install binary
	mkdir -p $(FLATPAK_BIN_DIR)
	install $(TARGET_DIR)/release/$(BIN_NAME) $(FLATPAK_BIN_DIR)/$(APP_ID)

	@# Install icons
	mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/scalable/apps
	mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/64x64/apps
	mkdir -p $(FLATPAK_SHARE_DIR)/icons/hicolor/128x128/apps
	install -m 644 $(OUT_DIR)/data/$(APP_ID).svg $(FLATPAK_SHARE_DIR)/icons/hicolor/scalable/apps/$(APP_ID).svg
	install -m 644 $(OUT_DIR)/data/$(APP_ID).64.png $(FLATPAK_SHARE_DIR)/icons/hicolor/64x64/apps/$(APP_ID).png
	install -m 644 $(OUT_DIR)/data/$(APP_ID).128.png $(FLATPAK_SHARE_DIR)/icons/hicolor/128x128/apps/$(APP_ID).png
	touch $(FLATPAK_SHARE_DIR)/icons/hicolor/

	@# Force icon cache to refresh
	mkdir -p $(FLATPAK_SHARE_DIR)/applications/
	mkdir -p $(FLATPAK_SHARE_DIR)/metainfo/
	
	install -m 644 $(OUT_DIR)/data/$(APP_ID).desktop $(FLATPAK_SHARE_DIR)/applications/$(APP_ID).desktop
	install -m 644 $(OUT_DIR)/data/$(APP_ID).appdata.xml $(FLATPAK_SHARE_DIR)/metainfo/$(APP_ID).appdata.xml
