use std::{collections::HashMap, fs::{File, create_dir_all}, io::{Read, Write}, path::{Path, PathBuf}};

use serde::Deserialize;
use toml::Value;

pub fn build_flatpak() {
    let descriptor =
        parse_project_descriptor(Path::new("Cargo.toml")).expect("Could not find Cargo.toml");
    let target_dir = Path::new("target/flatpak");
    create_dir_all(&target_dir).expect("Could not create flatpak build directory.");
    create_desktop_file(&descriptor, &target_dir).expect("Could not create desktop file.");
    create_flatpak_yml(&descriptor, &target_dir).expect("Could not create flatpak yml.");
    create_app_descriptor_xml(&descriptor, &target_dir).expect("Could not create flatpak yml.");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct FlatpakDescriptor {
    package_name: String,
    summary: String,
    description: String,
    categories: Vec<String>,

    screenshot: Option<String>,
    release_versions: Option<Vec<(String, String)>>,

    rating_violence_cartoon: Option<String>,
    rating_violence_fantasy: Option<String>,
    rating_violence_realistic: Option<String>,
    rating_violence_bloodshed: Option<String>,
    rating_violence_sexual: Option<String>,
    rating_drugs_alcohol: Option<String>,
    rating_drugs_narcotics: Option<String>,
    rating_drugs_tobacco: Option<String>,
    rating_sex_nudity: Option<String>,
    rating_sex_themes: Option<String>,
    rating_language_profanity: Option<String>,
    rating_language_humor: Option<String>,
    rating_language_discrimination: Option<String>,
    rating_social_chat: Option<String>,
    rating_social_info: Option<String>,
    rating_social_audio: Option<String>,
    rating_social_location: Option<String>,
    rating_social_contacts: Option<String>,
    rating_money_purchasing: Option<String>,
    rating_money_gambling: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProjectDescriptor {
    settings: HashMap<String, Value>,
    flatpak: FlatpakDescriptor,
}

fn parse_project_descriptor(path: &Path) -> std::io::Result<ProjectDescriptor> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let project_descriptor: ProjectDescriptor = toml::from_str(&s).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Could not parse project descriptor: {:?}.", e),
        )
    })?;
    Ok(project_descriptor)
}

fn create_desktop_file(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let name = descriptor.flatpak.package_name.split(".").last().unwrap();
    let template = include_str!("../data/app.template.desktop");
    let mut path = PathBuf::from(path);
    path.push(format!("{}.desktop", descriptor.flatpak.package_name));
    let mut file = File::create(path)?;
    file.write_all(
        template
            .replace("{name}", name)
            .replace("{summary}", &descriptor.flatpak.summary)
            .replace("{categories}", &descriptor.flatpak.categories.join(";"))
            .replace("{package_name}", &descriptor.flatpak.package_name)
            .as_bytes(),
    )?;
    Ok(())
}

fn create_flatpak_yml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let name = descriptor.flatpak.package_name.split(".").last().unwrap();
    let template = include_str!("../data/flatpak.template.yml");
    let mut path = PathBuf::from(path);
    path.push(format!("{}.yml", descriptor.flatpak.package_name));
    let mut file = File::create(path)?;
    file.write_all(
        template
            .replace("{name}", name)
            .replace("{package_name}", &descriptor.flatpak.package_name)
            .as_bytes(),
    )?;
    Ok(())
}

fn create_app_descriptor_xml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let name = descriptor.flatpak.package_name.split(".").last().unwrap();
    let template = include_str!("../data/appdata.template.xml");
    let mut path = PathBuf::from(path);
    path.push(format!("{}.appdata.xml", descriptor.flatpak.package_name));
    let mut file = File::create(path)?;

    #[rustfmt::skip]
    file.write_all(
        template
            .replace("{name}", name)
            .replace("{description}", &descriptor.flatpak.description)
            .replace("{package_name}", &descriptor.flatpak.package_name)
            .replace(
                "{categories}",
                &descriptor
                    .flatpak
                    .categories
                    .iter()
                    .map(|c| format!("<category>{}</category>\n", c))
                    .collect::<Vec<String>>()
                    .join(""),
            )
            .replace("{rating_violence_cartoon}",descriptor.flatpak.rating_violence_cartoon.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_fantasy}",descriptor.flatpak.rating_violence_fantasy.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_realistic}",descriptor.flatpak.rating_violence_realistic.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_bloodshed}",descriptor.flatpak.rating_violence_bloodshed.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_sexual}",descriptor.flatpak.rating_violence_sexual.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_alcohol}",descriptor.flatpak.rating_drugs_alcohol.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_narcotics}",descriptor.flatpak.rating_drugs_narcotics.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_tobacco}",descriptor.flatpak.rating_drugs_tobacco.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_sex_nudity}",descriptor.flatpak.rating_sex_nudity.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_sex_themes}",descriptor.flatpak.rating_sex_themes.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_profanity}",descriptor.flatpak.rating_language_profanity.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_humor}",descriptor.flatpak.rating_language_humor.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_discrimination",descriptor.flatpak.rating_language_discrimination.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_chat}",descriptor.flatpak.rating_social_chat.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_info}",descriptor.flatpak.rating_social_info.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_audio}",descriptor.flatpak.rating_social_audio.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_location}",descriptor.flatpak.rating_social_location.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_contacts}",descriptor.flatpak.rating_social_contacts.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_money_purchasing}",descriptor.flatpak.rating_money_purchasing.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_money_gambling}",descriptor.flatpak.rating_money_gambling.as_ref().unwrap_or(&"none".into()))
            .as_bytes(),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        env::temp_dir,
        fs::{create_dir_all, File},
        io::{Read, Write},
        path::PathBuf,
    };

    use toml::Value;

    use crate::flatpak::{
        create_desktop_file, parse_project_descriptor, FlatpakDescriptor, ProjectDescriptor,
    };

    fn init_descriptor() -> ProjectDescriptor {
        let mut settings = HashMap::new();
        settings.insert("test".into(), toml::Value::Integer(-20));

        ProjectDescriptor {
            flatpak: FlatpakDescriptor {
                package_name: "org.project.App".into(),
                summary: "This is a test app".into(),
                description: "A long text block for this app.".into(),
                categories: vec!["GTK".into(), "App".into()],
                screenshot: None,
                release_versions: None,
                rating_violence_cartoon: None,
                rating_violence_fantasy: None,
                rating_violence_realistic: None,
                rating_violence_bloodshed: None,
                rating_violence_sexual: None,
                rating_drugs_alcohol: None,
                rating_drugs_narcotics: None,
                rating_drugs_tobacco: None,
                rating_sex_nudity: None,
                rating_sex_themes: None,
                rating_language_profanity: None,
                rating_language_humor: None,
                rating_language_discrimination: None,
                rating_social_chat: None,
                rating_social_info: None,
                rating_social_audio: None,
                rating_social_location: None,
                rating_social_contacts: None,
                rating_money_purchasing: None,
                rating_money_gambling: None,
            },
            settings,
        }
    }

    fn get_temp() -> PathBuf {
        let mut tmp = temp_dir();
        tmp.push("gtk-app-framework-tests");
        create_dir_all(&tmp).unwrap();
        return tmp;
    }

    #[test]
    fn test_parse_toml_file() {
        let mut tmp = get_temp();
        tmp.push("test.toml");
        let mut file = File::create(&tmp).unwrap();

        file.write_all(
            r#"
[package]
name = "app"
version = "0.0.1"
edition = "2018"

[flatpak]
package-name = "org.project.App"
summary = "A simple App"
description = "A long text block for this app."
categories = ["GTK", "App", "Awesome"]
screenshot = "https://test"
release-versions = [ ["1.0.0", "01.01.1991"] ]
rating-violence-cartoon = "some"

[settings]
window-width = -1
window-height = -1
window-maximized = false

[dependencies]
log = "0.4.11"
env_logger = "0.9"

gettext-rs = "0.7"
gtk = { version = "0.3", package = "gtk4" }
libadwaita = "0.1.0-alpha-6"

[dependencies.gdk4]
version = "0.3"

[dependencies.glib]
version = "*"

[build-dependencies]
gtk-app-framework = { path = "../gtk-app-framework"}
"#
            .as_bytes(),
        )
        .unwrap();

        let descriptor = parse_project_descriptor(tmp.as_path()).unwrap();

        assert_eq!(descriptor.flatpak.package_name, "org.project.App");
        assert_eq!(descriptor.flatpak.summary, "A simple App");
        assert_eq!(
            descriptor.flatpak.description,
            "A long text block for this app."
        );
        assert_eq!(
            descriptor.flatpak.release_versions,
            Some(vec![("1.0.0".into(), "01.01.1991".into())])
        );
        assert_eq!(
            descriptor.flatpak.rating_violence_cartoon,
            Some("some".into())
        );
        assert_eq!(
            descriptor.flatpak.categories,
            vec![
                String::from("GTK"),
                String::from("App"),
                String::from("Awesome")
            ]
        );
        assert_eq!(
            descriptor.settings.get("window-width"),
            Some(&Value::Integer(-1))
        );
        assert_eq!(
            descriptor.settings.get("window-height"),
            Some(&Value::Integer(-1))
        );
        assert_eq!(
            descriptor.settings.get("window-maximized"),
            Some(&Value::Boolean(false))
        );
    }

    #[test]
    fn test_create_desktop_file() {
        let mut tmp = get_temp();
        tmp.push("test.desktop");

        let d = init_descriptor();

        create_desktop_file(&d, &tmp).unwrap();

        let mut s = String::new();
        File::open(tmp).unwrap().read_to_string(&mut s).unwrap();
        assert_eq!(
            s,
            r#"[Desktop Entry]
Name=App
GenericName=App
Comment=This is a test app
Categories=GTK;App
Icon=org.project.App
Exec=org.project.App
Terminal=false
Type=Application
"#
        )
    }
}
