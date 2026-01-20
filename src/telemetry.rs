use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;

const TELEMETRY_ENDPOINT: &str = "https://api.countapi.xyz/hit/suntheme/installs";

fn telemetry_flag_path() -> Result<PathBuf> {
    Ok(Config::state_dir()?.join(".telemetry_sent"))
}

/// Check if telemetry has already been sent for this install
pub fn has_sent_telemetry() -> bool {
    telemetry_flag_path().map(|p| p.exists()).unwrap_or(false)
}

/// Mark telemetry as sent
fn mark_telemetry_sent() -> Result<()> {
    let path = telemetry_flag_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, "1")?;
    Ok(())
}

/// Send anonymous install ping (only once per install)
pub fn send_install_ping() {
    if has_sent_telemetry() {
        return;
    }

    // Fire and forget - don't block on this
    std::thread::spawn(|| {
        let _ = send_ping();
        let _ = mark_telemetry_sent();
    });
}

fn send_ping() -> Result<()> {
    // Simple GET request to increment counter
    let _ = reqwest::blocking::get(TELEMETRY_ENDPOINT)?;
    Ok(())
}

/// Get current install count (for display purposes)
pub fn get_install_count() -> Result<u64> {
    let response: serde_json::Value =
        reqwest::blocking::get("https://api.countapi.xyz/get/suntheme/installs")?.json()?;

    response["value"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("Could not parse install count"))
}
