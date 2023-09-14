// src/git.rs

pub fn find_last_commit() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT */
    let last_commit = String::from_utf8(Command::new("git").args(&["log", "--format=\"%h\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit)
}

pub fn find_last_commit_message() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT MESSAGE */
    let last_commit_message = String::from_utf8(Command::new("git").args(&["log", "--format=\"%s\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_message)
}

pub fn find_last_commit_date() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT DATE */
    let last_commit_date = String::from_utf8(Command::new("git").args(&["log", "--format=\"%cd\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_date)
}

pub fn find_last_commit_author() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT AUTHOR */
    let last_commit_author = String::from_utf8(Command::new("git").args(&["log", "--format=\"%an\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_author)
}

pub fn find_last_commit_email() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT EMAIL */
    let last_commit_email = String::from_utf8(Command::new("git").args(&["log", "--format=\"%ae\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_email)
}

pub fn find_last_commit_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT HASH */
    let last_commit_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%H\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_hash)
}

pub fn find_last_commit_short_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT SHORT HASH */
    let last_commit_short_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%h\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_short_hash)
}

pub fn find_last_commit_tree_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT TREE HASH */
    let last_commit_tree_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%T\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_tree_hash)
}

pub fn find_last_commit_tree_short_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT TREE SHORT HASH */
    let last_commit_tree_short_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%t\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_tree_short_hash)
}

pub fn find_last_commit_parent_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT PARENT HASH */
    let last_commit_parent_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%P\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_parent_hash)
}

pub fn find_last_commit_parent_short_hash() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT PARENT SHORT HASH */
    let last_commit_parent_short_hash = String::from_utf8(Command::new("git").args(&["log", "--format=\"%p\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_parent_short_hash)
}

pub fn find_last_commit_ref_names() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT REF NAMES */
    let last_commit_ref_names = String::from_utf8(Command::new("git").args(&["log", "--format=\"%D\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_ref_names)
}

pub fn find_last_commit_encoding() -> Result<String, Box<dyn std::error::Error>> { //* FIND LAST COMMIT ENCODING */
    let last_commit_encoding = String::from_utf8(Command::new("git").args(&["log", "--format=\"%e\"", "-n", "1"]).output()?.stdout,)?;

    Ok(last_commit_encoding)
}