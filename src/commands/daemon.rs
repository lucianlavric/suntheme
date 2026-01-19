use anyhow::{Context, Result};
use std::fs;

use crate::config::Config;
use crate::sun_times::SunTimes;
use crate::theme_switcher::ThemeSwitcher;

pub fn start() -> Result<()> {
    // Check if already running
    if is_running()? {
        println!("Daemon is already running.");
        return Ok(());
    }

    let config = Config::load()?;

    // Create state directory
    let state_dir = Config::state_dir()?;
    fs::create_dir_all(&state_dir)?;

    let pid_file = Config::pid_file()?;
    let log_file = Config::log_file()?;

    // Use daemonize to start the daemon
    let daemonize = daemonize::Daemonize::new()
        .pid_file(&pid_file)
        .chown_pid_file(true)
        .working_directory(&state_dir)
        .stdout(fs::File::create(&log_file)?)
        .stderr(fs::File::create(&log_file)?);

    match daemonize.start() {
        Ok(_) => {
            // We're now in the daemon process
            run_daemon_loop(config)?;
        }
        Err(e) => {
            anyhow::bail!("Failed to daemonize: {}", e);
        }
    }

    Ok(())
}

fn run_daemon_loop(config: Config) -> Result<()> {
    use chrono::Local;
    use std::thread;
    use std::time::Duration;

    let switcher = ThemeSwitcher::new(config.clone());

    loop {
        // Fetch sun times
        let sun_times = match SunTimes::get_cached_or_fetch(
            config.location.latitude,
            config.location.longitude,
        ) {
            Ok(times) => times,
            Err(e) => {
                eprintln!("Failed to get sun times: {}", e);
                thread::sleep(Duration::from_secs(60));
                continue;
            }
        };

        // Apply current theme based on time
        let current_mode = sun_times.current_mode();
        if let Err(e) = switcher.apply_theme(current_mode) {
            eprintln!("Failed to apply theme: {}", e);
        }

        // Calculate time until next switch
        let (next_switch, _next_mode) = sun_times.next_switch();
        let now = Local::now();

        let sleep_duration = if next_switch > now {
            (next_switch - now)
                .to_std()
                .unwrap_or(Duration::from_secs(60))
        } else {
            // If next switch is in the past (tomorrow), sleep until midnight + buffer
            Duration::from_secs(60)
        };

        // Add a small buffer and cap at reasonable maximum
        let sleep_duration = sleep_duration
            .checked_add(Duration::from_secs(5))
            .unwrap_or(sleep_duration);
        let sleep_duration = std::cmp::min(sleep_duration, Duration::from_secs(3600)); // Max 1 hour

        eprintln!(
            "[{}] Current mode: {}, sleeping for {:?} until next check",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            current_mode,
            sleep_duration
        );

        thread::sleep(sleep_duration);
    }
}

pub fn stop() -> Result<()> {
    let pid_file = Config::pid_file()?;

    if !pid_file.exists() {
        println!("Daemon is not running (no PID file).");
        return Ok(());
    }

    let pid_str = fs::read_to_string(&pid_file).context("Failed to read PID file")?;
    let pid: i32 = pid_str.trim().parse().context("Invalid PID in PID file")?;

    // Send SIGTERM to the process
    unsafe {
        if libc::kill(pid, libc::SIGTERM) == 0 {
            println!("Daemon stopped (PID: {}).", pid);
        } else {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() == Some(libc::ESRCH) {
                println!("Daemon was not running (stale PID file).");
            } else {
                anyhow::bail!("Failed to stop daemon: {}", err);
            }
        }
    }

    // Remove PID file
    let _ = fs::remove_file(&pid_file);

    Ok(())
}

pub fn status() -> Result<()> {
    let config = Config::load().ok();
    let running = is_running()?;

    println!("Suntheme Status");
    println!("---------------");

    if running {
        let pid = get_pid()?;
        println!("Daemon:  running (PID: {})", pid.unwrap_or(0));
    } else {
        println!("Daemon:  not running");
    }

    // Show current theme state
    if let Some(cfg) = &config {
        let switcher = ThemeSwitcher::new(cfg.clone());
        if let Ok(Some(mode)) = switcher.get_current_mode() {
            println!("Theme:   {}", mode);
        } else {
            println!("Theme:   unknown");
        }

        // Show sun times if available
        if let Ok(sun_times) =
            SunTimes::get_cached_or_fetch(cfg.location.latitude, cfg.location.longitude)
        {
            println!();
            println!("Sunrise: {}", sun_times.sunrise_local().format("%H:%M:%S"));
            println!("Sunset:  {}", sun_times.sunset_local().format("%H:%M:%S"));

            let (next_switch, next_mode) = sun_times.next_switch();
            println!(
                "Next:    {} at {}",
                next_mode,
                next_switch.format("%H:%M:%S")
            );
        }
    } else {
        println!();
        println!("Config not found. Run 'suntheme init' to set up.");
    }

    Ok(())
}

fn is_running() -> Result<bool> {
    let pid = get_pid()?;
    match pid {
        Some(p) => Ok(process_exists(p)),
        None => Ok(false),
    }
}

fn get_pid() -> Result<Option<i32>> {
    let pid_file = Config::pid_file()?;
    if !pid_file.exists() {
        return Ok(None);
    }

    let pid_str = fs::read_to_string(&pid_file)?;
    let pid: i32 = pid_str.trim().parse()?;
    Ok(Some(pid))
}

fn process_exists(pid: i32) -> bool {
    unsafe { libc::kill(pid, 0) == 0 }
}
