use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct GeocodedLocation {
    pub display_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
struct NominatimResult {
    display_name: String,
    lat: String,
    lon: String,
}

pub fn geocode_location(query: &str) -> Result<Vec<GeocodedLocation>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=5",
        urlencoding::encode(query)
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("suntheme/0.1.0")
        .build()?;

    let results: Vec<NominatimResult> = client
        .get(&url)
        .send()
        .with_context(|| "Failed to geocode location")?
        .json()
        .with_context(|| "Failed to parse geocoding response")?;

    if results.is_empty() {
        anyhow::bail!("No locations found for '{}'", query);
    }

    let locations: Vec<GeocodedLocation> = results
        .into_iter()
        .filter_map(|r| {
            let lat = r.lat.parse().ok()?;
            let lon = r.lon.parse().ok()?;
            Some(GeocodedLocation {
                display_name: r.display_name,
                latitude: lat,
                longitude: lon,
            })
        })
        .collect();

    Ok(locations)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunTimes {
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: ApiResults,
    status: String,
}

#[derive(Debug, Deserialize)]
struct ApiResults {
    sunrise: DateTime<Utc>,
    sunset: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedData {
    sun_times: SunTimes,
    cached_at: DateTime<Utc>,
}

impl SunTimes {
    pub fn fetch(latitude: f64, longitude: f64) -> Result<Self> {
        let url = format!(
            "https://api.sunrise-sunset.org/json?lat={}&lng={}&formatted=0",
            latitude, longitude
        );

        let response: ApiResponse = reqwest::blocking::get(&url)
            .with_context(|| "Failed to fetch sun times from API")?
            .json()
            .with_context(|| "Failed to parse API response")?;

        if response.status != "OK" {
            anyhow::bail!("API returned error status: {}", response.status);
        }

        let today = Local::now().date_naive();

        Ok(SunTimes {
            sunrise: response.results.sunrise,
            sunset: response.results.sunset,
            date: today,
        })
    }

    pub fn get_cached_or_fetch(latitude: f64, longitude: f64) -> Result<Self> {
        let cache_path = Config::cache_dir()?.join("sun_times.json");
        let today = Local::now().date_naive();

        // Try to load from cache
        if cache_path.exists() {
            if let Ok(content) = fs::read_to_string(&cache_path) {
                if let Ok(cached) = serde_json::from_str::<CachedData>(&content) {
                    if cached.sun_times.date == today {
                        return Ok(cached.sun_times);
                    }
                }
            }
        }

        // Fetch fresh data
        let sun_times = Self::fetch(latitude, longitude)?;

        // Cache the result
        let cache_dir = Config::cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        let cached = CachedData {
            sun_times: sun_times.clone(),
            cached_at: Utc::now(),
        };

        let content = serde_json::to_string_pretty(&cached)?;
        fs::write(&cache_path, content)?;

        Ok(sun_times)
    }

    pub fn sunrise_local(&self) -> DateTime<Local> {
        self.sunrise.with_timezone(&Local)
    }

    pub fn sunset_local(&self) -> DateTime<Local> {
        self.sunset.with_timezone(&Local)
    }

    pub fn is_daytime(&self) -> bool {
        let now = Utc::now();
        now >= self.sunrise && now < self.sunset
    }

    pub fn current_mode(&self) -> ThemeMode {
        if self.is_daytime() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        }
    }

    pub fn next_switch(&self) -> (DateTime<Local>, ThemeMode) {
        let now = Utc::now();
        if now < self.sunrise {
            (self.sunrise_local(), ThemeMode::Light)
        } else if now < self.sunset {
            (self.sunset_local(), ThemeMode::Dark)
        } else {
            // After sunset, next switch is tomorrow's sunrise
            // For simplicity, we'll just indicate it's after today's events
            (self.sunrise_local() + chrono::Duration::days(1), ThemeMode::Light)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ThemeMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "light" => Ok(ThemeMode::Light),
            "dark" => Ok(ThemeMode::Dark),
            _ => anyhow::bail!("Invalid theme mode: {}. Use 'light' or 'dark'.", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_as_str() {
        assert_eq!(ThemeMode::Light.as_str(), "light");
        assert_eq!(ThemeMode::Dark.as_str(), "dark");
    }

    #[test]
    fn test_theme_mode_opposite() {
        assert_eq!(ThemeMode::Light.opposite(), ThemeMode::Dark);
        assert_eq!(ThemeMode::Dark.opposite(), ThemeMode::Light);
    }

    #[test]
    fn test_theme_mode_display() {
        assert_eq!(format!("{}", ThemeMode::Light), "light");
        assert_eq!(format!("{}", ThemeMode::Dark), "dark");
    }

    #[test]
    fn test_theme_mode_from_str() {
        assert_eq!("light".parse::<ThemeMode>().unwrap(), ThemeMode::Light);
        assert_eq!("dark".parse::<ThemeMode>().unwrap(), ThemeMode::Dark);
        assert_eq!("LIGHT".parse::<ThemeMode>().unwrap(), ThemeMode::Light);
        assert_eq!("Dark".parse::<ThemeMode>().unwrap(), ThemeMode::Dark);
    }

    #[test]
    fn test_theme_mode_from_str_invalid() {
        assert!("invalid".parse::<ThemeMode>().is_err());
        assert!("".parse::<ThemeMode>().is_err());
    }
}
