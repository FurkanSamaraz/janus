// src/main.rs

mod apply;
use apply::Version;
use regex::Regex;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let regex_ios_build = Regex::new(r#"CURRENT_PROJECT_VERSION\s+=\s+\d+;"#)?;
    let regex_ios_version = Regex::new(r#"MARKETING_VERSION\s+=\s+[\d.]+;"#)?;
    let regex_android_version_code = Regex::new(r#"versionCode\s+\d+"#)?;
    let regex_android_version_name = Regex::new(r#"versionName\s+"#)?;

    let mut version = Version {
        major: 1,
        minor: 0,
        patch: 0,
        build: 0,
    };

    let mut is_new_version = false;

    if let Ok(version_text) = fs::read_to_string(".version") {
        let lines: Vec<&str> = version_text.lines().collect();

        for line in lines {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_lowercase();
                let value = parts[1].trim();

                match key.as_str() {
                    "major" => version.major = value.parse::<u32>()?,
                    "minor" => version.minor = value.parse::<u32>()?,
                    "patch" => version.patch = value.parse::<u32>()?,
                    "build" => version.build = value.parse::<u32>()?,
                    _ => {}
                }
            }
        }
    } else {
        // .version dosyası bulunamazsa varsayılan değerler kullanılır.
        println!(".version file not found. Using default version values.");
    }

    let last_version = format!(
        "{}.{}.{}-{}",
        version.major, version.minor, version.patch, version.build
    );

    if args.len() > 1 {
        match args[1].as_str() {
            "apply" => println!("Not patching any version, just applying current one"),
            "patch" => {
                version.patch += 1;
                version.build += 1;
                is_new_version = true;
                println!("Patching version up to {}", version.patch);
            }
            "minor" => {
                version.minor += 1;
                version.patch = 0;
                version.build += 1;
                is_new_version = true;
                println!("Minor version up to {}", version.minor);
            }
            "major" => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
                version.build += 1;
                is_new_version = true;
                println!("Major version up to {}", version.major);
            }
            _ => eprintln!("Invalid argument \"{}\"", args[1]),
        }
    } else {
        println!("Not patching anything, just applying the current version set");
    }

    let version_str = format!("{}.{}.{}", version.major, version.minor, version.patch);
    println!("New version is {} Build {}", version_str, version.build);

    apply::apply_package(&version_str)?;
    apply::apply_ios(&version, &version_str, &regex_ios_build, &regex_ios_version)?;
    apply::apply_android(&version, &version_str, &regex_android_version_code, &regex_android_version_name)?;
    apply::apply_new_version(".version", &version)?;

    if is_new_version {
        apply::apply_commit(&version_str, &last_version)?;
    }

    println!("Successful!");
    Ok(())
}
