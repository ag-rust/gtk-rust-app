use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppDescriptor {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub categories: Vec<String>,

    pub screenshot: Option<String>,
    pub release_versions: Option<Vec<(String, String)>>,

    pub rating_violence_cartoon: Option<String>,
    pub rating_violence_fantasy: Option<String>,
    pub rating_violence_realistic: Option<String>,
    pub rating_violence_bloodshed: Option<String>,
    pub rating_violence_sexual: Option<String>,
    pub rating_drugs_alcohol: Option<String>,
    pub rating_drugs_narcotics: Option<String>,
    pub rating_drugs_tobacco: Option<String>,
    pub rating_sex_nudity: Option<String>,
    pub rating_sex_themes: Option<String>,
    pub rating_language_profanity: Option<String>,
    pub rating_language_humor: Option<String>,
    pub rating_language_discrimination: Option<String>,
    pub rating_social_chat: Option<String>,
    pub rating_social_info: Option<String>,
    pub rating_social_audio: Option<String>,
    pub rating_social_location: Option<String>,
    pub rating_social_contacts: Option<String>,
    pub rating_money_purchasing: Option<String>,
    pub rating_money_gambling: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageDescriptor {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionDescriptor {
    state: Option<String>,
    accelerators: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectDescriptor {
    pub package: PackageDescriptor,
    pub settings: HashMap<String, Value>,
    pub app: AppDescriptor,
    pub actions: HashMap<String, ActionDescriptor>,
}

pub fn parse_project_descriptor(path: &Path) -> std::io::Result<ProjectDescriptor> {
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

    use crate::{
        configs::{create_desktop_file, ProjectDescriptor},
        parse_project_descriptor, AppDescriptor,
    };

    fn init_descriptor() -> ProjectDescriptor {
        let mut settings = HashMap::new();
        settings.insert("test".into(), toml::Value::Integer(-20));

        ProjectDescriptor {
            package: PackageDescriptor {
                name: "example".into(),
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
    fn test_parse_toml_file() {
        let mut tmp = get_temp();
        tmp.push("test.toml");
        let mut file = File::create(&tmp).unwrap();

        file.write_all(
            r#"
            [package]
            name = "example"
            version = "0.1.0"
            edition = "2018"
            
            [app]
            id = "org.project.Example"
            summary = "A simple example app"
            description = "This example shows how to use rust to write gtk apps."
            categories = [ "GTK", "Example" ]
            releases = [ ["1.0.0", "01.01.1991"] ]
            
            [settings]
            window-width = 100
            window-height = 200
            window-maximized = false
            
            [actions]
            navigate = { state = "home" }
            show-help = { accelerators = ["F1"] }
            close = { accelerators = ["<Ctrl>W"] }"#
                .as_bytes(),
        )
        .unwrap();

        let descriptor = parse_project_descriptor(tmp.as_path()).unwrap();

        assert_eq!(descriptor.app.id, "org.project.App");
        assert_eq!(descriptor.app.summary, "A simple App");
        assert_eq!(
            descriptor.app.description,
            "A long text block for this app."
        );
        assert_eq!(
            descriptor.app.release_versions,
            Some(vec![("1.0.0".into(), "01.01.1991".into())])
        );
        assert_eq!(descriptor.app.rating_violence_cartoon, Some("some".into()));
        assert_eq!(
            descriptor.app.categories,
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
}
