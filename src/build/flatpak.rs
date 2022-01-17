// SPDX-License-Identifier: GPL-3.0-or-later

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

    let dev_flatpak_manifest = include_str!("../../data/flatpak-dev.template.yml");
    create_flatpak_yml(
        project_descriptor,
        &path,
        dev_flatpak_manifest,
        Some(".dev"),
    )
    .expect("Could not create flatpak yml.");

    let publish_flatpak_manifest = include_str!("../../data/flatpak.template.yml");
    create_flatpak_yml(project_descriptor, &path, publish_flatpak_manifest, None)
        .expect("Could not create flatpak yml.");

    if let Err(e) = create_app_descriptor_xml(project_descriptor, &path) {
        eprintln!("[gra] {}", e.to_string());
        return;
    }
    if let Err(e) = create_images(project_descriptor, &path) {
        eprintln!("{}", e.to_string());
    }
}

fn create_images(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let app_desc = &descriptor.app;

    let file64 = path.join(&format!("{}.64.png", &app_desc.id));
    println!("[gra] Generate {:?}", file64);
    std::fs::copy(Path::new("./assets/icon.64.png"), &file64)?;

    let file128 = path.join(&format!("{}.128.png", &app_desc.id));
    println!("[gra] Generate {:?}", file128);
    std::fs::copy(Path::new("./assets/icon.128.png"), &file128)?;

    let file_svg = path.join(&format!("{}.svg", &app_desc.id));
    println!("[gra] Generate {:?}", file_svg);
    std::fs::copy(Path::new("./assets/icon.svg"), &file_svg)?;
    Ok(())
}

fn create_desktop_file(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/app.template.desktop");

    let app_desc = &descriptor.app;

    let mut path = PathBuf::from(path);
    path.push(format!("{}.desktop", app_desc.id));

    println!("[gra] Generate {:?}", path);
    let mut file = File::create(path)?;

    let generic_name = app_desc
        .generic_name
        .as_ref()
        .unwrap_or(&descriptor.package.name);

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

fn create_flatpak_yml(
    descriptor: &ProjectDescriptor,
    path: &Path,
    template: &str,
    infix: Option<&str>,
) -> std::io::Result<()> {
    let app_desc = &descriptor.app;

    let mut path = PathBuf::from(path);
    path.push(format!("{}{}.yml", app_desc.id, infix.unwrap_or("")));

    println!("[gra] Generate {:?}", path);
    let mut file = File::create(path)?;

    let permissions = app_desc
        .permissions
        .iter()
        .map(|p| format!("- --{}", p))
        .collect::<Vec<String>>()
        .join("\n  ");
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
        format!(
            "{0}<{1}>\n{2}\n{0}</{1}>",
            " ".repeat(indentation),
            tagname,
            value
        )
    } else {
        format!("{0}<{1}>{2}</{1}>", " ".repeat(indentation), tagname, value)
    }
}

fn create_app_descriptor_xml(descriptor: &ProjectDescriptor, path: &Path) -> std::io::Result<()> {
    let template = include_str!("../../data/appdata.template.xml");

    let app_desc = &descriptor.app;

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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env::temp_dir, vec};

    use crate::AppDescriptor;

    use super::*;

    fn desc() -> ProjectDescriptor {
        ProjectDescriptor {
            package: crate::PackageDescriptor {
                name: String::from("example"),
                version: String::from("0.1.0"),
                authors: Some(vec![String::from("Foo Bar")]),
                homepage: Some(String::from("https://foo.bar")),
                license: None,
                repository: None,
            },
            app: AppDescriptor {
                id: String::from("org.example.Test"),
                generic_name: Some(String::from("Test")),
                summary: String::from("This is a test"),
                description: String::from("This is a test description"),
                categories: vec![String::from("GTK")],
                metadata_license: String::from("Foo"),
                screenshots: None,
                releases: None,
                content_rating: None,
                recommends: vec![],
                permissions: vec!["share=network".into(), "socket=x11".into()],
                resources: None,
            },
            actions: Some(HashMap::new()),
            settings: Some(HashMap::new()),
        }
    }

    #[test]
    fn test_flatpak_dev_manifest_generation() {
        let temp = temp_dir().join("test_flatpak_dev_manifest_generation");
        create_dir_all(&temp).unwrap();
        let dev_flatpak_manifest = include_str!("../../data/flatpak-dev.template.yml");
        create_flatpak_yml(&desc(), temp.as_path(), dev_flatpak_manifest, None)
            .expect("Could not generate org.example.Test.yml");

        let content = std::fs::read_to_string(temp.join("org.example.Test.yml")).unwrap();

        let unreplaced_tag = regex::Regex::new(r"\{.*\}").unwrap();
        assert!(!unreplaced_tag.is_match(&content));
    }

    #[test]
    fn test_flatpak_prod_manifest_generation() {
        let temp = temp_dir().join("test_flatpak_prod_manifest_generation");
        create_dir_all(&temp).unwrap();
        let manifest = include_str!("../../data/flatpak.template.yml");
        create_flatpak_yml(&desc(), temp.as_path(), manifest, None)
            .expect("Could not generate org.example.Test.yml");

        let mut content = std::fs::read_to_string(temp.join("org.example.Test.yml")).unwrap();

        let unreplaced_tag = regex::Regex::new(r"(?m)\{(.*)\}").unwrap();
        assert!(unreplaced_tag.is_match(&content));

        assert!(content.contains("{archive}"));
        content = content.replace("{archive}", "");

        assert!(unreplaced_tag.is_match(&content));
        assert!(content.contains("{sha256}"));
        content = content.replace("{sha256}", "");

        assert!(!unreplaced_tag.is_match(&content));
    }
}
