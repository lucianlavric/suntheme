use anyhow::Result;

use crate::config::Config;
use crate::sun_times::SunTimes;

pub fn run() -> Result<()> {
    let config = Config::load()?;

    println!("Fetching sun times for ({}, {})...\n",
        config.location.latitude,
        config.location.longitude
    );

    let sun_times = SunTimes::get_cached_or_fetch(
        config.location.latitude,
        config.location.longitude,
    )?;

    let sunrise = sun_times.sunrise_local();
    let sunset = sun_times.sunset_local();
    let current_mode = sun_times.current_mode();
    let (next_switch, next_mode) = sun_times.next_switch();

    println!("Today's Sun Times");
    println!("-----------------");
    println!("Sunrise: {}", sunrise.format("%H:%M:%S"));
    println!("Sunset:  {}", sunset.format("%H:%M:%S"));
    println!();
    println!("Current mode: {}", current_mode);
    println!("Next switch:  {} -> {} at {}",
        current_mode,
        next_mode,
        next_switch.format("%H:%M:%S")
    );

    Ok(())
}
