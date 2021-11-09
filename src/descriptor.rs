use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Screenshot {
    #[serde(rename = "type")]
    pub screenshot_type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContentRating {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppDescriptor {
    pub id: String,
    pub summary: String,
    pub description: String,
    pub categories: Vec<String>,
    pub metadata_license: String,
    
    pub screenshots: Option<Vec<Screenshot>>,
    pub releases: Option<Vec<Release>>,
    pub content_rating: Option<Vec<ContentRating>>,

    pub resources: Option<String>,

    pub control: Option<Vec<String>>,
    pub display: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PackageDescriptor {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActionDescriptor {
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    pub accelerators: Option<Vec<String>>,
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
