use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};

use sha2::Digest;

use crate::ProjectDescriptor;

pub fn build_gresources(project_descriptor: &ProjectDescriptor, target: &Path) {
    if project_descriptor.app.is_none() {
        eprintln!("[gra] Skip compiling gresources: Missing [app] section in Cargo.toml");
        return;
    }

    let assets = target.join("assets");
    std::fs::create_dir_all(&assets).expect("Could not create target/assets dir");

    let app_desc = project_descriptor.app.as_ref().unwrap();

    let resource_xml_path = assets.join(format!("resources.gresource.xml"));

    let template = include_str!("../../data/gresource.template.xml");

    let mut file;
    let old_hash;
    if resource_xml_path.exists() {
        file = File::open(&resource_xml_path).expect("Could not create gresource file.");

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .expect("Could not read old resources.gsresouce.xml");
        let mut hasher = sha2::Sha256::new();
        hasher.update(buf);
        old_hash = Some(hasher.finalize());
    } else {
        old_hash = None;
    };

    let icons_path = app_desc.resources.clone().unwrap_or("assets/icons".into());
    let icons_path = Path::new(&icons_path);

    let mut resources = Vec::new();
    if icons_path.exists() {
        for entry in std::fs::read_dir(&icons_path).unwrap() {
            let entry = entry.unwrap().path();
            let filename = entry.file_name().unwrap().to_string_lossy().to_string();
            resources.push(format!(
                "    <file preprocess=\"xml-stripblanks\" alias=\"icons/{0}\">{1}/{0}</file>",
                filename,
                icons_path.to_string_lossy()
            ));
        }
    }

    let svg_icon_path = Path::new("assets/icon.svg");
    if svg_icon_path.exists() {
        let filename = format!("{}", &app_desc.id);
        resources.push(format!(
            "    <file preprocess=\"xml-stripblanks\" alias=\"icons/{}.svg\">assets/icon.svg</file>",
            filename,
        ));
    }

    let new_file_content = template
        .replace("{id}", &app_desc.id.replace(".", "/"))
        .replace("{resources}", &resources.join("\n"));

    let mut hasher = sha2::Sha256::new();
    hasher.update(&new_file_content);
    let new_hash = hasher.finalize();

    if old_hash.is_some() && old_hash.unwrap() == new_hash {
        println!("[gra] Compiled gresources are up to date");
        return;
    }

    println!("[gra] Create gresources in {:?}", &resource_xml_path);
    file = File::create(&resource_xml_path).expect("Could not create gresource file.");
    file.write_all(new_file_content.as_bytes())
        .expect("Could not write to gresources file");

    let sourcedir_arg = Path::new(".").to_str().unwrap();
    let target_file = target.join("compiled.gresource");
    let target_arg = target_file.to_str().unwrap();
    let xml_file = std::fs::canonicalize(resource_xml_path).unwrap();
    let file_arg = xml_file.to_str().unwrap();

    println!(
        "[gra] glib-compile-resources --sourcedir {:?} --target {:?} {:?}",
        sourcedir_arg, target_arg, file_arg
    );
    let o = Command::new("glib-compile-resources")
        .current_dir(Path::new("."))
        .arg("--sourcedir")
        .arg(sourcedir_arg)
        .arg("--target")
        .arg(target_arg)
        .arg(file_arg)
        .output()
        .unwrap();
    if let Ok(o) = String::from_utf8(o.stdout) {
        if !o.trim().is_empty() {
            println!("{}", o);
        }
    }
    if let Ok(o) = String::from_utf8(o.stderr) {
        if !o.trim().is_empty() {
            eprintln!("{}", o);
        }
    }
}
