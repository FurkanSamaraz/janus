use std::error::Error;
use std::process::Command;
use std::str;

pub fn run_git_log(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        Err(format!("Git command failed: {}", stderr).into())
    }
}

pub fn find_last_commit() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%h", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_message() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%s", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_date() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%cd", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_author() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%an", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_email() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%ae", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%H", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_short_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%h", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_tree_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%T", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_tree_short_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%t", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_parent_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%P", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_parent_short_hash() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%p", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_ref_names() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%D", "-n", "1"];
    run_git_log(args)
}

pub fn find_last_commit_encoding() -> Result<String, Box<dyn Error>> {
    let args = &["log", "--format=%e", "-n", "1"];
    run_git_log(args)
}
