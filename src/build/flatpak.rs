use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;
use toml::Value;

use crate::ProjectDescriptor;

pub fn build_flatpak() {
    let descriptor =
        parse_project_descriptor(Path::new("Cargo.toml")).expect("Could not find Cargo.toml");
    let target_dir = Path::new("target/flatpak");
    create_dir_all(&target_dir).expect("Could not create flatpak build directory.");

    create_desktop_file(&descriptor, &target_dir).expect("Could not create desktop file.");
    create_flatpak_yml(&descriptor, &target_dir).expect("Could not create flatpak yml.");
    create_app_descriptor_xml(&descriptor, &target_dir).expect("Could not create flatpak yml.");
    create_images(&descriptor, &target_dir).expect("Could not copy app images.");
}

fn create_images(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    std::fs::copy(
        Path::new("./assets/icon.64.png"),
        path.join(&format!("{}.64.png", &descriptor.app.id)),
    )?;
    std::fs::copy(
        Path::new("./assets/icon.128.png"),
        path.join(&format!("{}.128.png", &descriptor.app.id)),
    )?;
    std::fs::copy(
        Path::new("./assets/icon.svg"),
        path.join(&format!("{}.svg", &descriptor.app.id)),
    )?;
}

fn create_desktop_file(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    // let name = descriptor.app.package_name.split(".").last().unwrap();
    let template = include_str!("../../data/app.template.desktop");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.desktop", descriptor.app.package_name));
    let mut file = File::create(path)?;

    file.write_all(
        template
            .replace("{id}", &descriptor.app.id)
            .replace("{name}", &descriptor.package.name)
            .replace("{summary}", &descriptor.app.summary)
            .replace("{categories}", &descriptor.app.categories.join(";"))
            .as_bytes(),
    )?;
    Ok(())
}

fn create_flatpak_yml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let name = descriptor.app.package_name.split(".").last().unwrap();
    let template = include_str!("../../data/flatpak.template.yml");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.yml", descriptor.app.id));
    let mut file = File::create(path)?;

    file.write_all(
        template
            .replace("{name}", name)
            .replace("{id}", &descriptor.app.id)
            .as_bytes(),
    )?;
    Ok(())
}

fn create_app_descriptor_xml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    // let name = descriptor.package.name;
    let template = include_str!("../../data/appdata.template.xml");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.appdata.xml", descriptor.app.id));
    let mut file = File::create(path)?;

    #[rustfmt::skip]
    file.write_all(
        template
            .replace("{name}", descriptor.package.name)
            .replace("{summary}", &descriptor.app.summary)
            .replace("{description}", &descriptor.app.description)
            .replace("{id}", &descriptor.app.id)
            .replace(
                "{categories}",
                &descriptor
                    .app
                    .categories
                    .iter()
                    .map(|c| format!("<category>{}</category>\n", c))
                    .collect::<Vec<String>>()
                    .join(""),
            )
            .replace("{rating_violence_cartoon}",descriptor.app.rating_violence_cartoon.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_fantasy}",descriptor.app.rating_violence_fantasy.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_realistic}",descriptor.app.rating_violence_realistic.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_bloodshed}",descriptor.app.rating_violence_bloodshed.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_violence_sexual}",descriptor.app.rating_violence_sexual.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_alcohol}",descriptor.app.rating_drugs_alcohol.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_narcotics}",descriptor.app.rating_drugs_narcotics.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_drugs_tobacco}",descriptor.app.rating_drugs_tobacco.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_sex_nudity}",descriptor.app.rating_sex_nudity.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_sex_themes}",descriptor.app.rating_sex_themes.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_profanity}",descriptor.app.rating_language_profanity.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_humor}",descriptor.app.rating_language_humor.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_language_discrimination",descriptor.app.rating_language_discrimination.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_chat}",descriptor.app.rating_social_chat.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_info}",descriptor.app.rating_social_info.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_audio}",descriptor.app.rating_social_audio.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_location}",descriptor.app.rating_social_location.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_social_contacts}",descriptor.app.rating_social_contacts.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_money_purchasing}",descriptor.app.rating_money_purchasing.as_ref().unwrap_or(&"none".into()))
            .replace("{rating_money_gambling}",descriptor.app.rating_money_gambling.as_ref().unwrap_or(&"none".into()))
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

    use crate::{build::flatpak::create_desktop_file, AppDescriptor, ProjectDescriptor};

    fn init_descriptor() -> ProjectDescriptor {
        let mut settings = HashMap::new();
        settings.insert("test".into(), toml::Value::Integer(-20));

        ProjectDescriptor {
            package: PackageDescriptor {
                name: "app".into(),
                version: "0.1.0".into(),
            },
            app: AppDescriptor {
                id: "org.project.App".into(),
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
            actions: HashMap::new(),
        }
    }

    fn get_temp() -> PathBuf {
        let mut tmp = temp_dir();
        tmp.push("gtk-app-framework-tests");
        create_dir_all(&tmp).unwrap();
        return tmp;
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
