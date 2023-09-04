// src/apply.rs

use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: u32,
}
impl Version {}

pub fn apply_package(version_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading package.json contents");
    let package_path = Path::new("package.json");
    let mut package_obj: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(package_path)?)?;

    package_obj["version"] = serde_json::Value::String(version_str.to_string());

    println!(
        "Applying new version \"{}\" to package manifest",
        version_str
    );

    println!("Writing new package.json");
    fs::write(package_path, serde_json::to_string_pretty(&package_obj)?)?;

    Ok(())
}

pub fn apply_ios(
    version: &Version,
    version_str: &str,
    regex_ios_build: Regex,
    regex_ios_version: Regex,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading IOS project.pbxproj");
    let manifest_path = Path::new("ios/MapleMars.xcodeproj/project.pbxproj");
    let manifest_backup_path = Path::new("ios/MapleMars.xcodeproj/project.pbxproj.bak");

    println!("Backing up project file");
    fs::copy(manifest_path, manifest_backup_path)?;

    let mut manifest_text = fs::read_to_string(manifest_path)?;

    manifest_text = regex_ios_build
        .replace_all(
            &manifest_text,
            format!("CURRENT_PROJECT_VERSION = {};", version.build).as_str(),
        )
        .to_string();

    manifest_text = regex_ios_version
        .replace_all(
            &manifest_text,
            format!("MARKETING_VERSION = {};", version_str).as_str(),
        )
        .to_string();

    println!("Writing new manifest file");
    fs::write(manifest_path, &manifest_text)?;

    Ok(())
}

pub fn apply_new_version(path: &str, version: &Version) -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing new .version file");
    let text = format!(
        "MAJOR={}\nMINOR={}\nPATCH={}\nBUILD={}",
        version.major, version.minor, version.patch, version.build
    );
    fs::write(path, &text)?;

    Ok(())
}

pub fn apply_commit(
    version_str: &str,
    last_version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let tag_name = format!("v{}", last_version);
    let last_id = String::from_utf8(
        Command::new("git")
            .args(&["log", "--format=\"%h\"", "-n", "1"])
            .output()?
            .stdout,
    )?;

    println!(
        "Tagging last commit ID \"{}\" as \"{}\" for previous release",
        last_id, tag_name
    );

    Command::new("git")
        .args(&[
            "tag",
            "-a",
            &tag_name,
            "-m",
            &format!("Version ready for release {}", tag_name),
            &last_id,
        ])
        .output()?;
    Command::new("git")
        .args(&["push", "origin", &tag_name])
        .output()?;

    let commit_msg = format!("Version bump to {}", version_str);
    Command::new("git").args(&["add", "."]).output()?;
    Command::new("git")
        .args(&["commit", "-m", &commit_msg])
        .output()?;

    println!("Applied new commit \"{}\"", commit_msg);

    Ok(())
}
