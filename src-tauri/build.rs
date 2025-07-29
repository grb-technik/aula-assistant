fn main() {
    let git_info = commit_information();

    if git_info.is_some() {
        let (commit_date, commit_short_id, commit_long_id) = git_info.unwrap();
        println!("cargo:rustc-env=LAST_COMMIT_DATE={}", commit_date.trim());
        println!("cargo:rustc-env=LAST_COMMIT_ID={}", commit_short_id.trim());
        println!(
            "cargo:rustc-env=LAST_COMMIT_ID_LONG={}",
            commit_long_id.trim()
        );
    }

    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP_UTC={}",
        chrono::Utc::now().to_rfc3339()
    );

    tauri_build::build()
}

fn commit_information() -> Option<(String, String, String)> {
    // commit date as YYY-MM-DD
    let date = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=short"])
        .output()
        .ok()?
        .stdout;
    let date = String::from_utf8(date).ok()?.trim().to_string();
    if date.is_empty() {
        return None;
    }

    // commit id as short
    let short_id = std::process::Command::new("git")
        .args(["log", "-1", "--format=%h"])
        .output()
        .ok()?
        .stdout;
    let short_id = String::from_utf8(short_id).ok()?.trim().to_string();
    if short_id.is_empty() {
        return None;
    }

    // commit id as long
    let long_id = std::process::Command::new("git")
        .args(["log", "-1", "--format=%H"])
        .output()
        .ok()?
        .stdout;
    let long_id = String::from_utf8(long_id).ok()?.trim().to_string();
    if long_id.is_empty() {
        return None;
    }

    Some((date, short_id, long_id))
}
