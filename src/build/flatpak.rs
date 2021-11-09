use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::ProjectDescriptor;

pub fn build_flatpak(project_descriptor: &ProjectDescriptor, target: &Path) {
    let path = target.join("data");
    create_dir_all(&path).expect("Could not create data build directory.");

    create_desktop_file(project_descriptor, &path).expect("Could not create desktop file.");
    create_flatpak_yml(project_descriptor, &path).expect("Could not create flatpak yml.");
    create_app_descriptor_xml(project_descriptor, &path).expect("Could not create flatpak yml.");
    create_images(project_descriptor, &path).expect("Could not copy app images.");
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
    Ok(())
}

fn create_desktop_file(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    // let name = descriptor.app.package_name.split(".").last().unwrap();
    let template = include_str!("../../data/app.template.desktop");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.desktop", descriptor.app.id));
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
    let template = include_str!("../../data/flatpak.template.yml");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.yml", descriptor.app.id));
    let mut file = File::create(path)?;

    file.write_all(
        template
            .replace("{name}", &descriptor.package.name)
            .replace("{id}", &descriptor.app.id)
            .as_bytes(),
    )?;
    Ok(())
}

fn as_tag_list<T>(
    elements: Option<&Vec<T>>,
    to_tag: impl Fn(&T) -> String + 'static,
) -> Option<String> {
    elements.map(|v| {
        v.iter()
            .map(|t| to_tag(t))
            .collect::<Vec<String>>()
            .join("\n")
    })
}

fn to_tag(value: &String, tagname: &str, indentation: usize) -> String {
    format!("{0}<{1}>{2}</{1}>", " ".repeat(indentation), tagname, value)
}

fn create_app_descriptor_xml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/appdata.template.xml");

    let mut path = PathBuf::from(path);
    path.push(format!("{}.appdata.xml", descriptor.app.id));
    let mut file = File::create(path)?;

    file.write_all(
        template
            .replace("{id}", &descriptor.app.id)
            .replace("{name}", &descriptor.package.name)
            .replace("{summary}", &descriptor.app.summary)
            .replace("{description}", &descriptor.app.description)
            .replace(
                "{license}",
                &descriptor.package.license.as_ref().unwrap_or(&"".into()),
            )
            .replace(
                "{homepage}",
                descriptor.package.homepage.as_ref().unwrap_or(&"".into()),
            )
            .replace(
                "{repository}",
                &descriptor.package.repository.as_ref().unwrap_or(&"".into()),
            )
            .replace("{metadata_license}", &descriptor.app.metadata_license)
            .replace(
                "{control}",
                &as_tag_list(descriptor.app.control.as_ref(), |control| {
                    to_tag(control, "control", 8)
                })
                .unwrap_or("".into()),
            )
            .replace(
                "{display}",
                &as_tag_list(descriptor.app.display.as_ref(), |display| {
                    to_tag(display, "display", 8)
                })
                .unwrap_or("".into()),
            )
            .replace(
                "{categories}",
                &as_tag_list(Some(&descriptor.app.categories), |category| {
                    to_tag(category, "category", 8)
                })
                .unwrap_or("".into()),
            )
            .replace(
                "{releases}",
                &as_tag_list(descriptor.app.releases.as_ref(), |r| {
                    format!("        <release version=\"{}\" date=\"{}\"><description>{}</description></release>", r.version, r.date, r.description)
                })
                .unwrap_or("".into()),
            )
            .replace(
                "{screenshots}",
                &as_tag_list(descriptor.app.screenshots.as_ref(), |s| {
                    format!("        <screenshot type=\"{}\"><image  type=\"source\">{}</image></screenshot>", s.screenshot_type, s.url)
                })
                .unwrap_or("".into()),
            )
            .replace(
                "{author}",
                &to_tag(&descriptor.package.authors.as_ref().map(|authors| authors
                     .first()
                     .map(|name| 
                        name.split_once("<")
                            .map(|t| t.0.trim().to_string())
                            .unwrap_or(name.clone())
                    ).unwrap_or("".into())).unwrap_or("".into())
                , "developer_name", 0)
            )
            .replace(
                "{content_rating}",
                &as_tag_list(descriptor.app.content_rating.as_ref(), |c| {
                    format!("        <content_attribute type=\"{}\">{}</content_attribute>", c.id, c.value)
                })
                .map(|cr| format!("    <content_rating type=\"oars-1.0\">{}</content_rating>", cr))
                .unwrap_or("".into())
            )
            .as_bytes(),
    )?;
    Ok(())
}
