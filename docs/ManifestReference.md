# GRA Manifest Reference

This document explain the contents of the App.toml content from `gtk-rust-app`. It contains a full list of possible sections and fields. Every fields purpose is statet as well as which generated files contain the value and a link to the underlying specification (given there is some).

# Table of Contents
1. [Section [app]](#app)
    - [categories](#categories)
    - [content-rating](#content-rating)
    - [description](#description)
    - [flatpak-modules](#flatpak-modules)
    - [flatpak-runtime-version](#flatpak-runtime-version)
    - [generic-name](#generic-name)
    - [id](#id)
    - [metadata-license](#metadata-license)
    - [mimetype](#mimetype)
    - [permissions](#permissions)
    - [requires](#requires)
    - [recommends](#recommends)
    - [releases](#releases)
    - [screenshots](#screenshots)
    - [summary](#summary)
2. [Section [settings]](#section-settings)
3. [Section [actions]](#section-actions)


## Section [app] <a name="app"></a>

The app section contains all metadata of your app.

| Field | Description | Type | Links |
| --- | --- | --- | --- |
| `categories` <a name="categories"></a> | A list of categories your app belongs to.<br>Example:<br>`categories = ["GTK", "Development"]` | `Vec<String>` | [Freedesktop Menu spec](https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry)<br>Used in:<br>`*.appdata.xml`<br>`*.desktop` |
| `content-rating` <a name="content-rating"></a> | A list of objects to specify age rating for your app.<br>Example:<br>`content-rating = [{ id = "language-humor", value = "mild" }]` | `Vec<{id:String, value:String}>` | [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-content_rating)<br>Used in:<br>`*.appdata.xml` |
| `description` <a name="description"></a> | A long description of your app. May contain some basic HTML tags (see spec).<br>Example:<br>`description = "<p>Lorem ipsum...</p>"` | `String` <br> (Basic&nbsp;HTML) | [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-description)<br>Used in:<br>`*.appdata.xml` |
| `flatpak-modules` <a name="flatpak-modules"></a> | A list of flatpak module definitions in yaml.<br>Example:<br> `flatpak-modules = [""" see cargo-gra/examples/complete """]` | `Vec<String>` | [Flatpak manifest reference](https://docs.flatpak.org/en/latest/manifests.html#modules)<br>Used in<br>`*.flatpak.yml` |
| `flatpak-runtime-version` <a name="flatpak-runtime-version"></a> | The version of the flatpak Gnome runtime to use.<br>Example:<br> `flatpak-runtime-version = "42"` | `String` | [Flatpak manifest reference](https://docs.flatpak.org/en/latest/manifests.html#basic-properties<br>Used in<br>`*.flatpak.yml` |
| `generic-name` <a name="generic-name"></a> | The generic name of your app. E.g. your clock app is called `clocky` but the displayed name in a distro is still `Clock`<br>Example:<br> `generic-name = "Clock"` | `String` | [Desktop file spec](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys)<br>Used in<br>`*.desktop` |
| `id` <a name="id"></a> | The unique identifier of your app.<br>Example:<br> `id = "org.example.TestApp"` | `String` | [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-id-generic)<br>Used in:<br>`*.flatpak.yml`<br>`*.desktop`<br>`*.appdata.xml` |
| `metadata-license` <a name="metadata-license"></a> | The license of the metadata xml file used to describe your app.<br> This may be a helpful link: https://techbase.kde.org/MetaInfo/DesktopApps#.3Cmetadata_license.2F.3E<br>Example:<br> `metadata-license = "CC0-1.0"` | `String` | [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-metadata_license)<br>Used in:<br>`*.appdata.xml` |
| `mimetype` <a name="mimetype"></a> | The license of the metadata xml file used to describe your app.<br> This may be a helpful link: https://developer-old.gnome.org/integration-guide/stable/mime.html.en<br>Example:<br> `mimetype = "image/png"` | `String` | [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-metadata_license)<br>Used in:<br>`*.desktop` |
| `permissions` <a name="permissions"></a> | A list of permissions your app will need. The values are the finish args for the flatpak build. The resulting flatpak container will request these permissions.<br>Example:<br>`permissions = ["share=network", "socket=wayland"]` | `Vec<String>` | [Flatpak spec](https://docs.flatpak.org/en/latest/sandbox-permissions.html#sandbox-permissions)<br>Used for:<br>`*.flatpak.yml` |
| `requires` <a name="requires"></a> | A list of screen and usage requirements. Important to notify users about the adaptiveness and inteded input method.<br>Example:<br> `requires = [ { display = ">360" }, { display = "<=1024" } ]` | `Vec<Requirement>` | [Freedesktop Menu spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations)<br>Used in:<br>`*.appdata.xml` |
| `recommends` <a name="recommends"></a> | A list of screen and usage recommendations. Important to notify users about the adaptiveness and inteded input method.<br>Example:<br> `recommends = [{ control = "pointer"}, { control = "keyboard"}, {control = "touch"}]` | `Vec<Recommendation>` | [Freedesktop Menu spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-relations)<br>Used in:<br>`*.appdata.xml` |
| `releases` <a name="releases"></a> | The release history of your app. This will be shown in store pages. Note: It may be useful to use conventional-commits and generate the CHANGELOG and release history based on your commit messages. Checkout [this project](https://gitlab.com/floers/karlender) to see how it can be done. <br>Example:<br>`releases = [{ version = "0.0.2", date = "2021-12-04", description = "The first version."}]`| `Vec<{ version:String, date:String, description:String}>`| [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-releases)<br>Used in:<br>`*.appdata.xml` | 
| `screenshots` <a name="screenshots"></a> | A list of screenshots. These screenshots will be displayed in the store page.<br>Example:<br> `screenshots = [{ type = "default", url = "https://..." }, { url = "https://..." }]` |`Vec<{type:String, url:String}>`| [AppStream spec](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-screenshots)<br>Used in:<br>`*.appdata.xml` |
| `summary` <a name="summary"></a> | A short description of your app. Will be shown in store pages and in distros search results.<br>Example:<br> `summary = "Time management made easy"` | `String` | [Specification](https://www.freedesktop.org/software/appstream/docs/chap-Metadata.html#tag-summary)<br>[Desktop file spec](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#recognized-keys)<br>Used in:<br>`*.appdata.xml`<br>`*.desktop (Comment)` |

## Section [settings] <a name="settings"></a>

Your app will most likely have some global, persisted settings. E.g. the window size and state may be persisted after your app is closed. These settings can be specified here as custom key-value pairs. Values may be numbers or strings but no objects.

### Example
```toml
[settings]
# The setting will be called `window-height` and have the initial value `600`
window-height = 600
```

The cargo-gra build will generate a gsettings schema here: `target/gra-gen/*.gschema.xml`. They can be installed globally during development via:

```
sudo cp target/gra-gen/*.gschema.xml /usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

Also they will be installed in the flatpak container in a flatpak build via `cargo gra flatpak`

## Section [actions] <a name="actions"></a>

Actions are useful for communication between parts of your app. [Read this for more info](https://gtk-rs.org/gtk4-rs/stable/latest/book/actions.html).

gtk-rust-app allows to specify global actions (Actions in the `app.` context) in the manifest. Doing so will register these actions and define usable constants.

Actions are key-value pairs where the key represents the action name. The value has the following type:
```
{
    type = String,
    accelerators: Vec<String>,
}
```
The type string **must** be a valid [Variant type string](https://gtk-rs.org/gtk-rs-core/stable/0.14/docs/glib/struct.VariantType.html#gvariant-type-strings).
The accelerators values are parsed as [described here](https://docs.gtk.org/gtk4/func.accelerator_parse.html).

*Note: Optionals (like `ms` for Option<String>) do not work ATM and I don't know why.*

### Example:
```toml
[actions]
quit = { accelerators = ["<primary>W"] }
```
