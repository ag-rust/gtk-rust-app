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

    if descriptor.app.is_none() {
        eprintln!("[gra] Skip assets/icon.*.png file processing: Missing [app] section in Cargo.toml");
        return Ok(());
    }

    let app_desc = descriptor.app.as_ref().unwrap();

    let file64 = path.join(&format!("{}.64.png", &app_desc.id));
    println!("[gra] Generate {:?}", file64);
    std::fs::copy(
        Path::new("./assets/icon.64.png"),
        &file64,
    )?;

    let file128 = path.join(&format!("{}.128.png", &app_desc.id));
    println!("[gra] Generate {:?}", file128);
    std::fs::copy(
        Path::new("./assets/icon.128.png"),
        &file128,
    )?;

    let file_svg = path.join(&format!("{}.svg", &app_desc.id));
    println!("[gra] Generate {:?}", file_svg);
    std::fs::copy(
        Path::new("./assets/icon.svg"),
        &file_svg,
    )?;
    Ok(())
}

fn create_desktop_file(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/app.template.desktop");
    
    if descriptor.app.is_none() {
        eprintln!("[gra] Skip desktop file: Missing [app] section in Cargo.toml");
        return Ok(());
    }

    let app_desc = descriptor.app.as_ref().unwrap();

    let mut path = PathBuf::from(path);
    path.push(format!("{}.desktop", app_desc.id));
    
    println!("[gra] Generate {:?}", path);
    let mut file = File::create(path)?;

    let generic_name = app_desc.generic_name.as_ref().unwrap_or(&descriptor.package.name);

    file.write_all(
        template
            .replace("{id}", &app_desc.id)
            .replace("{name}", &descriptor.package.name)
            .replace("{generic_name}", generic_name)
            .replace("{summary}", &app_desc.summary)
            .replace("{categories}", &app_desc.categories.join(";"))
            .as_bytes(),
    )?;
    Ok(())
}

fn create_flatpak_yml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/flatpak.template.yml");
    
    if descriptor.app.is_none() {
        eprintln!("[gra] Skip flatpak.yml file: Missing [app] section in Cargo.toml");
        return Ok(());
    }

    let app_desc = descriptor.app.as_ref().unwrap();

    let mut path = PathBuf::from(path);
    path.push(format!("{}.yml", app_desc.id));
    
    println!("[gra] Generate {:?}", path);
    let mut file = File::create(path)?;

    let permissions = app_desc.permissions
        .iter()
        .map(|p| format!("- --{}", p))
        .collect::<Vec<String>>().join("\n  ");
    file.write_all(
        template
            .replace("{name}", &descriptor.package.name)
            .replace("{id}", &app_desc.id)
            .replace("{permissions}", &permissions)
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

fn to_tag(value: &String, tagname: &str, linebreaks: bool, indentation: usize) -> String {
    if linebreaks {
        format!("{0}<{1}>\n{2}\n{0}</{1}>", " ".repeat(indentation), tagname, value)
    } else {
        format!("{0}<{1}>{2}</{1}>", " ".repeat(indentation), tagname, value)
    }
}

fn create_app_descriptor_xml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/appdata.template.xml");
    
    if descriptor.app.is_none() {
        eprintln!("[gra] Skip flatpak.yml file: Missing [app] section in Cargo.toml");
        return Ok(())
    }

    let app_desc = descriptor.app.as_ref().unwrap();

    let mut path = PathBuf::from(path);
    path.push(format!("{}.appdata.xml", app_desc.id));

    println!("[gra] Generate {:?}", path);
    let mut file = File::create(path)?;

    file.write_all(
        template
            .replace("{id}", &app_desc.id)
            .replace("{name}", &descriptor.package.name)
            .replace("{summary}", &app_desc.summary)
            .replace("{description}", &app_desc.description)
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
            .replace("{metadata_license}", &app_desc.metadata_license)
            .replace(
                "{recommends}",
                &as_tag_list(Some(&app_desc.recommends), |re| match re {
                        crate::Recommend::Display(v) => to_tag(&v, "display", false, 8),
                        crate::Recommend::Control(v) => to_tag(&v, "control", false, 8),
                    }
                )
                .map(|s| to_tag(&s, "recommends", true, 4))
                .unwrap_or("".into())
            )
            .replace(
                "{categories}",
                &as_tag_list(Some(&app_desc.categories), |category| {
                    to_tag(category, "category", false, 8)
                })
                .map(|s| to_tag(&s, "categories", true, 4))
                .unwrap_or("".into()),
            )
            .replace(
                "{releases}",
                &as_tag_list(app_desc.releases.as_ref(), |r| {
                    format!("        <release version=\"{}\" date=\"{}\"><description>{}</description></release>", 
                        r.version, 
                        r.date, 
                        r.description
                        )
                })
                .map(|s| to_tag(&s, "releases", true, 4))
                .unwrap_or("".into()),
            )
            .replace(
                "{screenshots}",
                &as_tag_list(app_desc.screenshots.as_ref(), |s| {
                    format!("        <screenshot {}><image  type=\"source\">{}</image></screenshot>", 
                        s.type_.as_ref().map(|t| format!("type=\"{}\"", t)).unwrap_or("".into()), 
                        s.url
                    )
                })
                .map(|s| to_tag(&s, "screenshots", true, 4))
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
                , "developer_name", false, 0)
            )
            .replace(
                "{content_rating}",
                &as_tag_list(app_desc.content_rating.as_ref(), |c| {
                    format!("        <content_attribute type=\"{}\">{}</content_attribute>", c.id, c.value)
                })
                .map(|cr| format!("    <content_rating type=\"oars-1.0\">\n{}\n    </content_rating>", cr))
                .unwrap_or("".into())
            )
            .as_bytes(),
    )?;
    Ok(())
}
