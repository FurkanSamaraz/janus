// src/main.rs
mod apply;
mod base64;
mod bcrypt;
mod git_utils;
mod urid;
use apply::Version;
use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("janus")
        .version("1.0")
        .author("Your Name")
        .about("A utility for managing project versions")
        .subcommand(
            SubCommand::with_name("version").about("Display the Janus version and patch level"),
        )
        .subcommand(
            SubCommand::with_name("apply-package")
                .about("Apply version to package.json")
                .arg(Arg::with_name("version").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("apply-android")
                .about("Apply version to Android project")
                .arg(Arg::with_name("version").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("apply-ios")
                .about("Apply version to iOS project")
                .arg(Arg::with_name("version").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("apply-new-version")
                .about("Apply new version to .version file")
                .arg(Arg::with_name("path").required(true).index(1))
                .arg(Arg::with_name("version").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("apply-commit")
                .about("Apply a new commit")
                .arg(Arg::with_name("version").required(true).index(1))
                .arg(Arg::with_name("last-version").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("bcrypt")
                .about("Encrypt a value using bcrypt")
                .arg(Arg::with_name("value").required(true).index(1))
                .arg(
                    Arg::with_name("rounds")
                        .short("r")
                        .takes_value(true)
                        .help("Specify rounds"),
                ),
        )
        .subcommand(
            SubCommand::with_name("base64")
                .about("Base64 encoding")
                .arg(Arg::with_name("value").required(true).index(1))
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .takes_value(true)
                        .help("Base64 encode a file's contents"),
                ),
        )
        .subcommand(
            SubCommand::with_name("urid")
                .about("Maple URID operations")
                .subcommand(SubCommand::with_name("generate").about("Generate a new URID"))
                .subcommand(
                    SubCommand::with_name("convert")
                        .about("Convert a URID back into a SQL UUID")
                        .arg(Arg::with_name("urid").required(true).index(1)),
                ),
        )
        .subcommand(
            SubCommand::with_name("git")
                .about("Git utility operations")
                .subcommand(
                    SubCommand::with_name("log")
                        .about("Retrieve information about the last commit"),
                )
                .subcommand(SubCommand::with_name("commit").about("Commit changes"))
                .subcommand(
                    SubCommand::with_name("commit-message")
                        .about("Retrieve the commit message of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-date")
                        .about("Retrieve the commit date of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-author")
                        .about("Retrieve the author of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-email")
                        .about("Retrieve the email of the last commit author"),
                )
                .subcommand(
                    SubCommand::with_name("commit-hash")
                        .about("Retrieve the hash of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-short-hash")
                        .about("Retrieve the short hash of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-tree-hash")
                        .about("Retrieve the tree hash of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-tree-short-hash")
                        .about("Retrieve the short tree hash of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-parent-hashes")
                        .about("Retrieve the parent hashes of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-parent-short-hash")
                        .about("Retrieve the short parent hash of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-ref-names")
                        .about("Retrieve reference names of the last commit"),
                )
                .subcommand(
                    SubCommand::with_name("commit-encoding")
                        .about("Retrieve the encoding of the last commit"),
                ),
        )
        .get_matches();

    let regex_ios_build = Regex::new(r#"CURRENT_PROJECT_VERSION\s+=\s+\d+;"#)?;
    let regex_ios_version = Regex::new(r#"MARKETING_VERSION\s+=\s+[\d.]+;"#)?;
    let regex_android_version_code = Regex::new(r#"versionCode\s+\d+"#)?;
    let regex_android_version_name = Regex::new(r#"versionName\s+"[\d.]+"#)?;

    let mut version = Version {
        major: 1,
        minor: 0,
        patch: 0,
        build: 0,
    };

    let mut _is_new_version = false;

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
        println!(".version file not found. Using default version values.");
    }

    let last_version = format!(
        "{}.{}.{}-{}",
        version.major, version.minor, version.patch, version.build
    );

    match matches.subcommand() {
        ("version", _) => {
            apply::version_patch()?;
        }
        ("apply-package", Some(args)) => {
            let version_str = args.value_of("version").unwrap();
            apply::apply_package(version_str)?;
        }
        ("apply-android", Some(args)) => {
            let version_str = args.value_of("version").unwrap();
            apply::apply_android(
                &version,
                version_str,
                &regex_android_version_code,
                &regex_android_version_name,
            )?;
        }
        ("apply-ios", Some(args)) => {
            let version_str = args.value_of("version").unwrap();
            apply::apply_ios(&version, version_str, &regex_ios_build, &regex_ios_version)?;
        }
        ("apply-new-version", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let _version_str = args.value_of("version").unwrap();
            apply::apply_new_version(path, &version)?;
        }
        ("apply-commit", Some(args)) => {
            let version_str = args.value_of("version").unwrap();
            apply::apply_commit(version_str, &last_version)?;
        }
        ("bcrypt", Some(args)) => {
            let value = args.value_of("value").unwrap();
            let rounds = args.value_of("rounds").map(|r| r.parse::<u32>().unwrap());
            bcrypt::bcrypt(value, rounds)?;
        }
        ("base64", Some(args)) => {
            let value = args.value_of("value").unwrap();
            let file = args.value_of("file");
            base64::base64_encode(value, file)?;
        }
        ("urid", Some(subcommand)) => match subcommand.subcommand() {
            ("generate", _) => {
                urid::urid_generate()?;
            }
            ("convert", Some(args)) => {
                let urid = args.value_of("urid").unwrap();
                urid::urid_convert(urid)?;
            }
            _ => {
                // Geçersiz urid altkomut kullanımını ele alın
                println!("{}", matches.usage());
            }
        },
        ("git", Some(subcommand)) => match subcommand.subcommand() {
            ("commit", _) => {
                git_utils::find_last_commit()?;
            }
            ("commit_message", _) => {
                git_utils::find_last_commit_message()?;
            }
            ("commit_date", _) => {
                git_utils::find_last_commit_date()?;
            }
            ("commit_author", _) => {
                git_utils::find_last_commit_author()?;
            }
            ("commit_email", _) => {
                git_utils::find_last_commit_email()?;
            }
            ("commit_hash", _) => {
                git_utils::find_last_commit_hash()?;
            }
            ("commit_short_hash", _) => {
                git_utils::find_last_commit_short_hash()?;
            }
            ("commit_tree_hash", _) => {
                git_utils::find_last_commit_tree_hash()?;
            }
            ("commit_tree_short_hash", _) => {
                git_utils::find_last_commit_tree_short_hash()?;
            }
            ("commit_parent_hash", _) => {
                git_utils::find_last_commit_parent_hash()?;
            }
            ("commit_parent_short_hash", _) => {
                git_utils::find_last_commit_parent_short_hash()?;
            }
            ("commit_ref_names", _) => {
                git_utils::find_last_commit_ref_names()?;
            }
            ("commit_encoding", _) => {
                git_utils::find_last_commit_encoding()?;
            }
            _ => {
                // Geçersiz urid altkomut kullanımını ele alın
                println!("{}", matches.usage());
            }
        },
        _ => {}
    }

    println!("{}", matches.usage());
    Ok(())
}
