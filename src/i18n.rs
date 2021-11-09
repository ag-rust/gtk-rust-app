use std::io::BufRead;

pub fn init_gettext(domain: &str) {
    let textdomain = match std::env::var("TEXT_DOMAIN") {
        Ok(path) => gettextrs::TextDomain::new(domain)
            .skip_system_data_paths()
            .push(&path)
            .init(),
        Err(_) => gettextrs::TextDomain::new(domain).init(),
    };
    match textdomain {
        Ok(locale) => match locale {
            Some(_locale) => {
                // nothing to do
            },
            None => eprintln!("Warning: No locale was set! Probably /usr/share/locale/*/LC_MESSAGES does not contain a .mo file.")
        },
        Err(e) => match e {
            gettextrs::TextDomainError::InvalidLocale(locale) => eprintln!("Warning: Invalid locale {:?}", locale),
            gettextrs::TextDomainError::TranslationNotFound(locale) => match locale.as_str() {
                "en" => {
                    // use default language
                },
                _ => error!("Warning: Could not find messages for locale {:?}", locale)
            },
            e => {
                error!("{:?}", e);
            }
        }
    };
}


pub fn build_gettext() {
    let domain = "gnome-lbry";
    
    let potfiles = format!("po/POTFILES.in");
    let target_pot_dir = format!("target/po");

    if let Err(e) = std::fs::create_dir_all(target_pot_dir) {
        println!("cargo:warning={:?}", e);
    }

    for line in read_lines("po/LINGUAS").unwrap() {
        if let Ok(line) = line {
            if !line.starts_with("#") {
                let locale = line;

                let po_file = format!("po/{}.po", locale);
                let pot_file = format!("target/po/{}.pot", locale);

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
