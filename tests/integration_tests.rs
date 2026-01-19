use std::path::PathBuf;
use std::process::Command;

fn suntheme_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("release")
        .join("suntheme")
}

#[test]
fn test_help_command() {
    let output = Command::new(suntheme_bin())
        .arg("--help")
        .output()
        .expect("Failed to run suntheme");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("suntheme"));
    assert!(stdout.contains("sunrise/sunset"));
}

#[test]
fn test_version_command() {
    let output = Command::new(suntheme_bin())
        .arg("--version")
        .output()
        .expect("Failed to run suntheme");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("suntheme"));
}

#[test]
fn test_sun_command_without_config() {
    let output = Command::new(suntheme_bin())
        .arg("sun")
        .output()
        .expect("Failed to run suntheme");

    // Should fail without config
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stdout, stderr);

    // Either it works (config exists) or it tells user to run init
    assert!(
        combined.contains("Sunrise") || combined.contains("init") || combined.contains("Config")
    );
}

#[test]
fn test_status_command() {
    let output = Command::new(suntheme_bin())
        .arg("status")
        .output()
        .expect("Failed to run suntheme");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Suntheme Status") || stdout.contains("Daemon"));
}

#[test]
fn test_invalid_command() {
    let output = Command::new(suntheme_bin())
        .arg("invalidcommand")
        .output()
        .expect("Failed to run suntheme");

    assert!(!output.status.success());
}
