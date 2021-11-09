# GTK Rust App Template

This is a template for GTK4 Rust apps.

## Features

- gettext included in cargo build via build script
- logging via env_logger
- cross compilation with flatpak wrapped in makefile script
- auto versioning based on conventional_commits wrapped in makefile script

## Requirements

Cross compilation requires qemu-user-static

```
apt -y install qemu-user-static
```

```
pacman -S qemu-user-static
```