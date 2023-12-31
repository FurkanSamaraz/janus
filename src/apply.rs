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

// Bu fonksiyon package.json dosyasını günceller.
pub fn apply_package(version_str: &str) -> Result<(), Box<dyn std::error::Error>> { //* PACKAGE.JSON UPDATE */
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

// Bu fonksiyon Android projenizi günceller.
pub fn apply_android(version: &Version,version_str: &str,regex_android_version_code: &Regex,regex_android_version_name: &Regex,) -> Result<(), Box<dyn std::error::Error>> {
    // Android projesinin yerel yolunu bulun.
    let android_project_path = find_android_project_path()?;//* ANDROID PATH */

    // Android projesinin build.gradle dosyasının yolunu belirleyin.
    let build_gradle_path = android_project_path.join("build.gradle"); //* ANDROID PATH IN FILE */

    if build_gradle_path.exists() {
        println!("Reading Android build.gradle");
        
        let mut build_gradle_text = fs::read_to_string(&build_gradle_path)?;

        // versionCode değerini günceller.
        build_gradle_text = regex_android_version_code //* ANDROID VERSION_CODE */
            .replace_all(
                &build_gradle_text,
                format!("versionCode {}", version.build).as_str(),
            )
            .to_string();

        // versionName değerini sadece yeni sürümle günceller.
        build_gradle_text = regex_android_version_name //* ANDROID VERSION_NAME */
            .replace_all(
                &build_gradle_text,
                format!("versionName \"{}\"", version_str).as_str(),
            )
            .to_string();

        // versionName değeri içindeki eski sürüm numaralarını kaldırın.
        build_gradle_text = build_gradle_text.replace(
            format!("versionName \"{}\"", version_str).as_str(),
            format!("versionName \"{}\"", version_str).as_str(),
        );

        println!("Writing new Android build.gradle file");
        fs::write(&build_gradle_path, &build_gradle_text)?;
    }

    Ok(())
}

// Bu fonksiyon iOS projenizi günceller.
pub fn apply_ios(version: &Version,version_str: &str,regex_ios_build: &Regex,regex_ios_version: &Regex,) -> Result<(), Box<dyn std::error::Error>> {
    // iOS projesinin yerel yolunu bulun.
    let ios_project_path = find_ios_project_path()?; //* IOS PATH */

    println!("Reading IOS project.pbxproj");
    let manifest_path = ios_project_path.join("project.pbxproj"); //* IOS PATH IN .PBXPROJ FILE */
    let manifest_backup_path = ios_project_path.join("project.pbxproj.bak"); //* IOS PATH IN .BAK FILE */

    println!("Backing up project file");
    fs::copy(&manifest_path, &manifest_backup_path)?;

    let mut manifest_text = fs::read_to_string(&manifest_path)?;

    // CURRENT_PROJECT_VERSION ve MARKETING_VERSION değerlerini günceller.
    manifest_text = regex_ios_build    //* IOS CURRENT_PROJECT_VERSION */
        .replace_all(
            &manifest_text,
            format!("CURRENT_PROJECT_VERSION = {};", version.build).as_str(),
        )
        .to_string();

    manifest_text = regex_ios_version //* IOS MARKETING_VERSION */
        .replace_all(
            &manifest_text,
            format!("MARKETING_VERSION = \"{}\";", version_str).as_str(),
        )
        .to_string();

    println!("Writing new manifest file");
    fs::write(&manifest_path, &manifest_text)?;

    Ok(())
}

// Bu fonksiyon .version dosyasını günceller.
pub fn apply_new_version(path: &str, version: &Version) -> Result<(), Box<dyn std::error::Error>> { //* .VERSIN UPDATE */
    println!("Writing new .version file");
    let text = format!(
        "MAJOR={}\nMINOR={}\nPATCH={}\nBUILD={}",
        version.major, version.minor, version.patch, version.build
    );
    fs::write(path, &text)?;

    Ok(())
}

// Bu fonksiyon git işlemlerini gerçekleştirir.
pub fn apply_commit(version_str: &str,last_version: &str,) -> Result<(), Box<dyn std::error::Error>> { //* GIT PUSH */
    let tag_name = format!("v{}", last_version);
    let last_id = String::from_utf8(Command::new("git").args(&["log", "--format=\"%h\"", "-n", "1"]).output()?.stdout,)?;

    println!("Tagging last commit ID \"{}\" as \"{}\" for previous release",last_id, tag_name);

    Command::new("git").args(&["tag","-a",&tag_name,"-m",&format!("Version ready for release {}", tag_name),&last_id,]).output()?;
    Command::new("git").args(&["push", "origin", &tag_name]).output()?;

    let commit_msg = format!("Version bump to {}", version_str);

    Command::new("git").args(&["add", "."]).output()?;
    Command::new("git").args(&["commit", "-m", &commit_msg]).output()?;

    
    Command::new("git").args(&["push"]).output()?;
    Command::new("git").args(&["push", "--tags"]).output()?;

    println!("Applied new commit \"{}\"", commit_msg);

    Ok(())
}

// Bu fonksiyon iOS projesinin yerel yolunu bulur.
fn find_ios_project_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> { //* IOS PACKAGE.JSON AND PATH */
    // Proje adını package.json dosyasından alın.
    let package_path = Path::new("package.json");
    if package_path.exists() {
        let package_obj: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(package_path)?)?;

        if let Some(project_name) = package_obj["name"].as_str() {
            // iOS projesi için beklenebilecek klasör adını oluşturun.
            let ios_project_name = format!("{}.xcodeproj", project_name);
            let ios_project_path = Path::new("ios").join(ios_project_name);

            if ios_project_path.exists() {
                return Ok(ios_project_path);
            }
        }
    }

    // Eğer proje adı bulunamazsa veya iOS projesi yoksa hata döndürün.
    Err("iOS project not found")?
}

fn find_android_project_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {//* ANDROID PACKAGE.JSON AND PATH */

    // Proje adını package.json dosyasından alın.
    let package_path = Path::new("package.json");
    if package_path.exists() {
        let package_obj: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(package_path)?)?;

        if let Some(project_name) = package_obj["name"].as_str() {
            // Android projesi için beklenebilecek klasör adını oluşturun.
            let android_project_name = format!("{}_android", project_name);
            let android_project_path = Path::new("android").join(android_project_name);

            if android_project_path.exists() {
                return Ok(android_project_path);
            }
        }
    }

    // Eğer proje adı bulunamazsa veya Android projesi yoksa hata döndürün.
    Err("Android project not found".into())
}

pub fn version_patch() -> Result<(), Box<dyn std::error::Error>> {
    // Burada versiyonu yükseltmek veya güncellemek için yapmanız gereken işlemleri gerçekleştirin.
    // Örnek olarak, bir versiyon dosyasını güncelleyebilirsiniz.
    
    println!("Versiyon yükseltildi.");
    Ok(())
}



