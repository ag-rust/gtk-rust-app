// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::ProjectDescriptor;

pub fn build_actions(project_descriptor: &ProjectDescriptor, target: &Path) {
    let path = target.join("actions.rs");

    if project_descriptor.actions.is_none() {
        return;
    }

    let mut new_actions_file_content = String::new();
    for (name, _) in project_descriptor.actions.as_ref().unwrap() {
        new_actions_file_content.push_str(&format!(
            "pub const {}: &str = \"{}\";\n",
            name.to_uppercase().replace("-", "_"),
            name
        ));
    }

    match File::open(&path) {
        Ok(mut actions_file) => {
            let mut buf = Vec::new();
            actions_file
                .read_to_end(&mut buf)
                .expect("Could not read actions.rs file.");

            if buf != new_actions_file_content.as_bytes() {
                println!("[gra] Update {:?}", &path);
                let mut actions_file =
                    File::create(path).expect("Could not create actions.rs file.");
                actions_file
                    .write(new_actions_file_content.as_bytes())
                    .expect("Could not write to actions.rs file.");
            }
        }
        Err(_) => {
            println!("[gra] Create {:?}", &path);
            let mut actions_file = File::create(&path).expect("Could not create actions.rs file.");
            actions_file
                .write(new_actions_file_content.as_bytes())
                .expect("Could not write to actions.rs file.");
        }
    }
}
