use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::ProjectDescriptor;

pub fn build_gschema_settings(project_descriptor: &ProjectDescriptor, target: &Path) {
    let settings_gschema = include_str!("../../data/gschema.template.xml");

    if project_descriptor.app.is_none() {
        eprintln!("[gra] Skip gsettings schema generation: Missing [app] section in Cargo.toml");
        return;
    }

    if project_descriptor.settings.is_none() {
        eprintln!(
            "[gra] Skip gsettings schema generation: Missing [settings] section in Cargo.toml"
        );
        return;
    }

    let app_desc = project_descriptor.app.as_ref().unwrap();
    let settings_desc = project_descriptor.settings.as_ref().unwrap();

    let mut path = PathBuf::from(target);
    path.push(format!("{}.gschema.xml", app_desc.id));

    println!("[gra] Create {:?}", path);
    let mut file =
        File::create(&path).expect(&format!("Could not create gsettings file {:?}.", &path));

    let mut keys = Vec::new();
    for (name, default_value) in settings_desc.iter() {
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
            "<key name=\"{}\" type=\"{}\">\n  <default>{}</default>\n</key>",
            name, value_type, default_value
        ));
    }

    if let Err(e) = file.write_all(
        settings_gschema
            .replace("{id}", &app_desc.id)
            .replace("{path}", &format!("/{}/", app_desc.id.replace(".", "/")))
            .replace("{keys}", &keys.join("\n"))
            .as_bytes(),
    ) {
        eprintln!("[gra] Could not write gsettings: {:?}", e);
    }
}
