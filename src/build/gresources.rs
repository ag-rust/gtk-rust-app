use std::{fs::File, io::Write, path::Path};

use crate::ProjectDescriptor;

pub fn build_gresources(project_descriptor: &ProjectDescriptor, target: &Path) {

    let target_name = target.file_name().unwrap().to_str().unwrap();
    let target = target.join("assets");
    std::fs::create_dir_all(&target).expect("Could not create target/assets dir");

    let resource_xml_path = target.join(format!("resources.gresource.xml"));

    let template = include_str!("../../data/gresource.template.xml");

    let mut file = File::create(&resource_xml_path).expect("Could not create gresource file.");

    let icons_path = project_descriptor
        .app
        .resources
        .clone()
        .unwrap_or("assets/icons".into());
    let icons_path = Path::new(&icons_path);
    
    let mut resources = Vec::new();
    if icons_path.exists() {
        for entry in std::fs::read_dir(&icons_path).unwrap() {
            let entry = entry.unwrap().path();
            let filename = entry.file_name().unwrap().to_string_lossy().to_string();
            resources.push(format!(
                "    <file preprocess=\"xml-stripblanks\" alias=\"icons/{0}\">../../{1}/{0}</file>",
                filename,
                icons_path.to_string_lossy()
            ));
        }
    }

    file.write_all(
        template
            .replace("{id}", &project_descriptor.app.id.replace(".", "/"))
            .replace("{resources}", &resources.join("\n"))
            .as_bytes(),
    )
    .expect("Could not write to gresources file");

    let outdir = std::env::var("OUT_DIR").unwrap();
    let mut out_dir = Path::new(&outdir);

    let mut above = 0;
    while !out_dir.ends_with("target") && out_dir.parent().is_some() {
        out_dir = out_dir.parent().unwrap();
        above += 1;
    }

    gdk4::gio::compile_resources(
        target.to_str().unwrap(),
        resource_xml_path.to_str().unwrap(),
        &format!("{}/../{}/assets/compiled.gresource", vec![".."; above].join("/"), target_name),
    );
}
