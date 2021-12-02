use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::Deserialize;
use toml::Value;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename(serialize = "component"))]
pub struct ProjectDescriptor {
    pub package: PackageDescriptor,
    pub app: Option<AppDescriptor>,
    pub actions: Option<HashMap<String, ActionDescriptor>>,
    pub settings: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageDescriptor {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AppDescriptor {
    // appdata
    pub id: String,
    pub generic_name: Option<String>,
    pub summary: String,
    pub description: String,
    pub categories: Vec<String>,
    pub metadata_license: String,
    pub screenshots: Option<Vec<Screenshot>>,
    pub releases: Option<Vec<Release>>,
    pub content_rating: Option<Vec<ContentRating>>,
    pub recommends: Vec<Recommend>,
    pub permissions: Vec<String>,

    // misc
    pub resources: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Screenshot {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ContentRating {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Recommend {
    Display(String),
    Control(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionDescriptor {
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub accelerators: Option<Vec<String>>,
}

pub fn parse_project_descriptor(path: &Path) -> std::io::Result<ProjectDescriptor> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    parse_project_descriptor_str(&s)
}

pub fn parse_project_descriptor_str(s: &str) -> std::io::Result<ProjectDescriptor> {
    let project_descriptor: ProjectDescriptor = toml::from_str(s).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Could not parse project descriptor: {:?}.", e),
        )
    })?;
    Ok(project_descriptor)
}

pub fn parse_project_descriptor_bytes(s: &[u8]) -> std::io::Result<ProjectDescriptor> {
    let project_descriptor: ProjectDescriptor = toml::from_slice(s).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Could not parse project descriptor: {:?}.", e),
        )
    })?;
    Ok(project_descriptor)
}
