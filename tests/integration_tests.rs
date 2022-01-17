use std::{
    env::temp_dir,
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use fs_extra::dir::CopyOptions;
use regex::Captures;

#[test]
fn test_build_flatpaks() {
    let test_project = PathBuf::new().join("examples/simple");
    assert!(test_project.exists());

    Command::new("rm")
        .arg("-rf")
        .arg(test_project.join("target"))
        .output()
        .unwrap();
    assert!(!test_project.join("target").exists());

    Command::new("cargo")
        .current_dir(&test_project)
        .args(["build", "--release"])
        .output()
        .unwrap();

    assert_gra_gen_dir(&test_project);

    // replace gtk-rust-app dependency with repo version for this test
    // required because flatpak-temp setup runs cargo vendor and that
    // does not work with local dependencies
    let toml = test_project.join("Cargo.toml");
    let local_toml_content = read_to_string(&toml).unwrap();

    let match_local_dep = regex::Regex::new(r"(?m)^gtk-rust-app(.*)").unwrap();

    let test_toml_content = match_local_dep
        .replace_all(&local_toml_content, |caps: &Captures| {
            format!("# gtk-rust-app{}", &caps[1])
        })
        .to_string();

    let target_branch =
        std::env::var("CI_MERGE_REQUEST_SOURCE_BRANCH_NAME").unwrap_or("main".to_string());
    let match_repo_dep = regex::Regex::new(r"(?m)^# IT gtk-rust-app(.*)").unwrap();
    let test_toml_content = match_repo_dep
        .replace_all(&test_toml_content, |caps: &Captures| {
            format!("gtk-rust-app{}", &caps[1])
        })
        .replace("{BRANCH}", &target_branch)
        .to_string();
    println!("{}", test_toml_content);

    let mut f = File::create(&toml).unwrap();
    f.write_all(test_toml_content.as_bytes()).unwrap();
    drop(f);

    let r = gtk_rust_app::prepare_flatpak_temp(&test_project);

    // Reset to local version
    let mut f = File::create(&toml).unwrap();
    f.write_all(local_toml_content.as_bytes()).unwrap();

    let flatpak_temp = r.expect("Could not create flatpak-temp dir");

    assert!(flatpak_temp.exists());
    assert!(test_project
        .join("target/flatpak-temp/.cargo/config.toml")
        .exists());
    assert!(test_project
        .join("target/flatpak-temp/target/vendor")
        .exists());
    assert!(test_project
        .join("target/flatpak-temp/target/vendor/gtk-rust-app")
        .exists());
    assert!(test_project.join("target/flatpak-temp/po").exists());
    assert!(test_project.join("target/flatpak-temp/src").exists());
    assert!(test_project.join("target/flatpak-temp/Cargo.toml").exists());

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let flatpak_temp = PathBuf::new().join("examples/simple/target/flatpak-temp");
    let test_dir = temp_dir().join("gtk-rust-app-prod-flatpak-test");
    create_dir_all(&test_dir).unwrap();
    fs_extra::dir::copy(flatpak_temp, &test_dir, &options)
        .expect("Could not copy to temp test dir");

    run_flatpak_builder(&test_dir.join("flatpak-temp"));
    run_flatpak_bundle(&test_dir.join("flatpak-temp"));

    assert!(test_dir.join("example.flatpak").exists());
}

fn run_flatpak_builder(current_dir: &PathBuf) {
    let mut build_task = Command::new("flatpak-builder")
        .current_dir(&current_dir)
        .arg("--repo=../flatpak-build/repo")
        .arg("--state-dir=../flatpak-build/state")
        .arg("--force-clean")
        .arg("../flatpak-build/host")
        .arg("target/gra-gen/data/org.example.SimpleExample.dev.yml")
        .spawn()
        .unwrap();
    build_task.wait().unwrap();
}

fn run_flatpak_bundle(current_dir: &PathBuf) {
    let mut bundle_task = Command::new("flatpak")
        .current_dir(&current_dir)
        .arg("build-bundle")
        .arg("../flatpak-build/repo")
        .arg("../example.flatpak")
        .arg("org.example.SimpleExample")
        .spawn()
        .unwrap();
    bundle_task.wait().unwrap();
}

fn assert_gra_gen_dir(project_dir: &PathBuf) {
    let gra_gen_dir = project_dir.join("target/gra-gen");

    assert!(gra_gen_dir.join("assets/resources.gresource.xml").exists());

    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.64.png")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.128.png")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.appdata.xml")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.desktop")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.dev.yml")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.svg")
        .exists());
    assert!(gra_gen_dir
        .join("data/org.example.SimpleExample.yml")
        .exists());

    assert!(gra_gen_dir
        .join("locale/de/LC_MESSAGES/simple-example.mo")
        .exists());

    assert!(gra_gen_dir.join("po/de.pot").exists());
    assert!(gra_gen_dir.join("po/POTFILES.in").exists());

    assert!(gra_gen_dir.join("actions.rs").exists());
    assert!(gra_gen_dir.join("compiled.gresource").exists());
    assert!(gra_gen_dir.join("Makefile").exists());
    assert!(gra_gen_dir
        .join("org.example.SimpleExample.gschema.xml")
        .exists());
}
