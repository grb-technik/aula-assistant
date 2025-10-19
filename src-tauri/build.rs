fn main() {
    if let Some((date, short_id, long_id)) = commit_information() {
        println!("cargo:rustc-env=LAST_COMMIT_DATE={}", date);
        println!("cargo:rustc-env=LAST_COMMIT_ID={}", short_id);
        println!("cargo:rustc-env=LAST_COMMIT_ID_LONG={}", long_id);
    } else {
        eprintln!("cargo:warning=Failed to retrieve Git commit information");
    }

    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP_UTC={}",
        chrono::Utc::now().to_rfc3339()
    );

    tauri_build::build()
}

fn commit_information() -> Option<(String, String, String)> {
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd|%h|%H", "--date=short"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let parts: Vec<&str> = stdout.trim().split('|').collect();

    if parts.len() != 3 {
        return None;
    }

    Some((
        parts[0].trim().to_string(), // date
        parts[1].trim().to_string(), // short id
        parts[2].trim().to_string(), // long id
    ))
}
