use std::{fs::File, io::Write, path::Path};

use crate::ProjectDescriptor;

pub fn build_makefile(_: &ProjectDescriptor, path: &Path) {
    let template = include_str!("../../data/Makefile");
    let file_path = path.join("Makefile");
    let mut file = File::create(file_path).expect("Could not create Makefile");
    file.write_all(template.as_bytes())
        .expect("Could not write to Makefile");
}
