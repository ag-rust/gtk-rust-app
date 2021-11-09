use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
};

use crate::parse_project_descriptor;

pub fn build_gettext() {
    let target_pot_dir = "target/po".to_string();

    let project_descriptor =
        parse_project_descriptor(Path::new("Cargo.toml")).expect("Could not find Cargo.toml");

    let potfiles = format!("{}/POTFILES.in", target_pot_dir);
    let potfiles_file = File::create(potfiles).expect("Could not create target/po/POTFILES.in");

    match std::process::Command::new("grep")
        .args(&["-lr", "gettext", "*"])
        .output()
    {
        Err(e) => {
            println!("cargo:warning={:?}", e);
            return;
        }
        Ok(output) => {
            let potfiles_list = ouput.to_string();
            potfiles_file
                .write_all(potfiles_list.as_bytes())
                .expect("Could not write to target/po/POTFILES.in");
        }
    }

    if let Err(e) = std::fs::create_dir_all(target_pot_dir) {
        println!("cargo:warning={:?}", e);
        return;
    }

    for line in read_lines("po/LINGUAS").unwrap() {
        if let Ok(line) = line {
            if !line.starts_with("#") {
                let locale = line;

                let po_file = format!("po/{}.po", locale);
                let pot_file = format!("{}/{}.pot", target_pot_dir, locale);

                let target_mo_dir = format!("target/locale/{}/LC_MESSAGES", locale);
                let target_mo = format!("target/locale/{}/LC_MESSAGES/{}.mo", locale, domain);

                if let Err(e) = std::fs::create_dir_all(target_mo_dir) {
                    println!("cargo:warning={:?}", e);
                }

                if let Err(e) = std::process::Command::new("xgettext")
                    .arg("-f")
                    .arg(&potfiles)
                    .arg("-o")
                    .arg(&pot_file)
                    .output()
                {
                    println!("cargo:warning={:?}", e);
                }
                if let Err(e) = std::process::Command::new("msgmerge")
                    .arg(&po_file)
                    .arg(&pot_file)
                    .arg("-U")
                    .output()
                {
                    println!("cargo:warning={:?}", e);
                }
                if let Err(e) = std::process::Command::new("msgfmt")
                    .arg("-o")
                    .arg(&target_mo)
                    .arg(&po_file)
                    .output()
                {
                    println!("cargo:warning={:?}", e);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
