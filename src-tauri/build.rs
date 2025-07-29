fn main() {
    let (commit_date, commit_short_id, commit_long_id) = commit_information();
    println!("cargo:rustc-env=BUILD_TIMESTAMP_UTC={}", chrono::Utc::now().to_rfc3339());
    println!("cargo:rustc-env=LAST_COMMIT_DATE={}", commit_date.trim());
    println!("cargo:rustc-env=LAST_COMMIT_ID={}", commit_short_id.trim());
    println!("cargo:rustc-env=LAST_COMMIT_ID_LONG={}", commit_long_id.trim());
    tauri_build::build()
}

fn commit_information() -> (String, String, String) {
    // commit date as YYY-MM-DD
    let date = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=short"])
        .output()
        .expect("Failed to get commit date")
        .stdout;
    let date = String::from_utf8(date).expect("Failed to parse commit date");

    // commit id as short and long
    let short_id = std::process::Command::new("git")
        .args(["log", "-1", "--format=%h"])
        .output()
        .expect("Failed to get commit id")
        .stdout;
    let short_id = String::from_utf8(short_id).expect("Failed to parse commit id");

    let long_id = std::process::Command::new("git")
        .args(["log", "-1", "--format=%H"])
        .output()
        .expect("Failed to get commit id")
        .stdout;
    let long_id = String::from_utf8(long_id).expect("Failed to parse commit id");

    (date, short_id, long_id)
}
