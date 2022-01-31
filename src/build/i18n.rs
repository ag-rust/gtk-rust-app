// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
};

use crate::ProjectDescriptor;

pub fn build_gettext(project_descriptor: &ProjectDescriptor, target: &Path) {
    let id = &project_descriptor.app.id;
    let domain = &project_descriptor.package.name;
    let target_pot_dir = &target.join("po");
    std::fs::create_dir_all(&target_pot_dir).expect("Could not create target/po.");

    let potfiles = target_pot_dir.join("POTFILES.in");
    let mut potfiles_file =
        File::create(&potfiles).expect("Could not create target/po/POTFILES.in");

    let mut pofiles: Vec<String>;
    match std::process::Command::new("grep")
        .args(&["-lr", "-E", "gettext|translatable", "src/"])
        .output()
    {
        Ok(output) => {
            pofiles = String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
        }
        Err(e) => {
            eprintln!("[gra] {:?}", e);
            return;
        }
    }
    pofiles.push(
        target
            .join(format!("data/{}.appdata.xml", id))
            .to_str()
            .unwrap()
            .to_string(),
    );

    if let Err(e) = potfiles_file.write_all(pofiles.join("\n").as_bytes()) {
        eprintln!("[gra] {:?}", e);
    }

    if let Err(e) = std::fs::create_dir_all(target_pot_dir) {
        eprintln!("[gra] {:?}", e);
        return;
    }

    println!("[gra] Update po files");
    let lines = read_lines("po/LINGUAS");
    if lines.is_err() {
        eprintln!(
            "[gra] Can not read po/LINGUAS file: {}",
            lines.unwrap_err().to_string()
        );
        return;
    }
    for line in lines.unwrap() {
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
                    eprintln!("[gra] {:?}", e);
                }

                println!("[gra] Generate/Update translations {:?}", &po_file);

                println!("[gra] xgettext -f {:?} -o {:?}", &potfiles, &pot_file);
                match std::process::Command::new("xgettext")
                    .arg("-f")
                    .arg(&potfiles)
                    .arg("-o")
                    .arg(&pot_file)
                    .output()
                {
                    Err(e) => {
                        println!("[gra] {:?}", e);
                    }
                    Ok(output) => {
                        let o = String::from_utf8(output.stdout).unwrap();
                        if !o.trim().is_empty() {
                            println!("[gra] {}", o);
                        }
                        if output.stderr.len() > 0 {
                            for line in String::from_utf8(output.stderr)
                                .unwrap_or("".into())
                                .lines()
                            {
                                if line == "" || line.contains("'rs'") || line.contains("warning") {
                                    continue;
                                }
                                eprintln!("[gra] {}", line);
                            }
                        }
                    }
                }
                println!("[gra] msgmerge {:?} -U {:?}", &po_file, &pot_file);
                match std::process::Command::new("msgmerge")
                    .arg(&po_file)
                    .arg(&pot_file)
                    .arg("-U")
                    .output()
                {
                    Err(e) => {
                        eprintln!("[gra] {:?}", e);
                    }
                    Ok(output) => {
                        let o = String::from_utf8(output.stdout).unwrap();
                        if !o.trim().is_empty() {
                            println!("[gra] {}", o);
                        }
                        if output.stderr.len() > 0 {
                            let s = String::from_utf8(output.stderr).unwrap_or("".into());
                            if !s.contains("...") {
                                eprintln!("[gra] {}", s)
                            }
                        }
                    }
                }
                println!("[gra] msgfmt -o {:?} {:?}", &target_mo, &pot_file);
                match std::process::Command::new("msgfmt")
                    .arg("-o")
                    .arg(&target_mo)
                    .arg(&po_file)
                    .output()
                {
                    Err(e) => {
                        eprintln!("[gra] {:?}", e);
                    }
                    Ok(output) => {
                        let o = String::from_utf8(output.stdout).unwrap();
                        if !o.trim().is_empty() {
                            println!("[gra] {}", o);
                        }
                        if output.stderr.len() > 0 {
                            let s = String::from_utf8(output.stderr).unwrap_or("".into());
                            eprintln!("[gra] {}", s)
                        }
                    }
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
