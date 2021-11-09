use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
};

use crate::ProjectDescriptor;

pub fn build_gettext(project_descriptor: &ProjectDescriptor, target: &Path) {
    let domain = &project_descriptor.package.name;
    let target_pot_dir = &target.join("po");
    std::fs::create_dir_all(&target_pot_dir).expect("Could not create target/po.");

    let potfiles = target_pot_dir.join("POTFILES.in");
    let mut potfiles_file =
        File::create(&potfiles).expect("Could not create target/po/POTFILES.in");

    match std::process::Command::new("grep")
        .args(&["-lr", "gettext", "src/"])
        .output()
    {
        Ok(output) => {
            if let Err(e) = potfiles_file.write_all(&output.stdout) {
                println!("cargo:warning={:?}", e)
            }
            if let Ok(e) = String::from_utf8(output.stderr) {
                if e.len() > 0 {
                    println!("cargo:warning={}", e);
                }
            }
        }
        Err(e) => {
            println!("cargo:warning={:?}", e);
            return;
        }
    }


    if let Err(e) = std::fs::create_dir_all(target_pot_dir) {
        println!("cargo:warning={:?}", e);
        return;
    }

    for line in read_lines("po/LINGUAS").unwrap() {
        if let Ok(line) = line {
            if !line.starts_with("#") {
                let locale = &line;

                let po_file = format!("po/{}.po", locale);
                let pot_file = target_pot_dir.join(format!("{}.pot", locale));

                let target_mo_dir = target.join("locale").join(locale).join("LC_MESSAGES");

                let target_mo = target
                    .join("locale")
                    .join(locale)
                    .join("LC_MESSAGES")
                    .join(format!("{}.mo", domain));

                if let Err(e) = std::fs::create_dir_all(target_mo_dir) {
                    println!("cargo:warning={:?}", e);
                }

                match std::process::Command::new("xgettext")
                                    .arg("-f")
                                    .arg(&potfiles)
                                    .arg("-o")
                                    .arg(&pot_file)
                                    .output() {
                    Err(e) => {
                        println!("cargo:warning={:?}", e);
                    }
                    Ok(output) => {
                        if output.stderr.len() > 0 {
                            println!("cargo:warning={}", String::from_utf8(output.stderr).unwrap_or("".into()))
                        }
                    },
                }
                match std::process::Command::new("msgmerge")
                                    .arg(&po_file)
                                    .arg(&pot_file)
                                    .arg("-U")
                                    .output() {
                    Err(e) => {
                        println!("cargo:warning={:?}", e);
                    }
                    Ok(output) => {
                        if output.stderr.len() > 0 {
                            println!("cargo:warning={}", String::from_utf8(output.stderr).unwrap_or("".into()))
                        }
                    },
                }
                match std::process::Command::new("msgfmt")
                                    .arg("-o")
                                    .arg(&target_mo)
                                    .arg(&po_file)
                                    .output() {
                    Err(e) => {
                        println!("cargo:warning={:?}", e);
                    }
                    Ok(output) => {
                        if output.stderr.len() > 0 {
                            println!("cargo:warning={}", String::from_utf8(output.stderr).unwrap_or("".into()))
                        }
                    },
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
