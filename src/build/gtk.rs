use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;
use toml::Value;

use crate::ProjectDescriptor;

pub fn build_gschema_settings(project_descriptor: &ProjectDescriptor, path: &Path) {
    let mut settings_gschema = include_str!("../../data/gschema.template.xml");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.gschema.xml", project_descriptor.app.id));

    let mut file = File::create(path)?;

    let mut keys = Vec::new();
    for (name, default_value) in project_descriptor.settings.iter() {
        let value_type = match default_value {
            toml::Value::String(_) => "s",
            toml::Value::Integer(_) => "i",
            toml::Value::Float(_) => "f",
            toml::Value::Boolean(_) => "b",
            toml::Value::Datetime(_) => "s",
            toml::Value::Array(_) => "s",
            toml::Value::Table(_) => "s",
        };

        keys.push(format!(
            "<key name="{}" type={}>\n  <default>{}</default>\n</key>",
            name, value_type, default_value
        ));
    }

    file.write_all(
        settings_gschema
            .replace("{id}", project_descriptor.app.id)
            .replace("{path}", project_descriptor.app.id.replace(".", "/"))
            .replace("{keys}", keys.join("\n"))
            .as_bytes(),
    )?;
}
