# Manifest documentation

## Section [app]
This section contains metadata for your app. It is used to generate the `*.appdata.xml`, `*.desktop` and manifest.yml (`*.yml`) files in `target/flatpak`.

### id

The flatpak app id.
This is internal app name and execution command of your app when built with flatpak.

#### Example:
`id = "org.example.AwsomeApp"`

#### Used for:
- File names of app icons, .desktop file, appdata.xml and manifest.yml
- The field `app-id` in flatpaks manifest.yml
- `IconName` and `ExecName` fields in `*.desktop`
- `id` field in `*.appdata.xml`

### summary

A short summary of your app.

#### Example: 
`summary = "An awesome app."`

#### Used for:
- `summary` field in `*.appdata.xml`
- The comment field in `*.desktop`

### description

A description for your app. May contain multiple sentences.

#### Example: 
`description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."`

#### Used for:
- `description` field in `*.appdata.xml`

#### Limitations: 
- Currently only english and only one paragraph is supported

### categories

A list of category tags. Valid values are listed [here](https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry).

#### Example: 
`categories = [ "Audio", "System", "Utility" ]`

#### Used for:
- `Categories` field in `*.desktop`
- `categories` field in `*.appdata.xml`


## Section [settings]

This section will generate a GNOME settings definition file: `target/<app-id>.gschema.xml`. The file needs to be installed in GNOME to allow the app to read and write the settings.

The settings section may contain arbitrary key-value pairs where the key is the setting key and the value its initial value.

### Example
```toml
[settings]
window-width = 600
window-height = 600
```

## Section [actions]

This section generates global app actions (GTK Actions). It may contain arbitrary keys. The values must be objects with the following fields:
```
type = <gtk type string. E.g. "s">
accelerators = <array of gtk accelerator strings. E.g. ["<Ctrl>A", "<primary>P"]>
```

> Note: The action type "s" (string) may be used to send serialized `Action` structs.