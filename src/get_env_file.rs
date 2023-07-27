/*
 * @Author: timochan
 * @Date: 2023-07-17 13:51:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-27 20:16:39
 * @FilePath: /processforlinux/src/get_env_file.rs
 */
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
struct ConfigError(String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConfigError {}

struct UserConfig {
    api_url: String,
    api_key: String,
    watch_time: i64,
    media_enable: bool,
    log_enable: bool,
}

fn read_config_values(config_path: &str) -> Result<UserConfig, Box<dyn Error>> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let (mut api_url, mut api_key, mut watch_time, mut media_enable, mut log_enable) =
        (None, None, None, None, None);

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('#') || trimmed_line.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed_line.split_once('=') {
            match key {
                "API_URL" => api_url = Some(value.to_string()),
                "API_KEY" => api_key = Some(value.to_string()),
                "WATCH_TIME" => watch_time = Some(value.parse()?),
                "MEDIA_ENABLE" => media_enable = Some(value.parse()?),
                "LOG_ENABLE" => log_enable = Some(value.parse()?),
                _ => {
                    // Handle unknown or invalid key-value pairs
                    eprintln!("Unknown key-value pair: {}", trimmed_line);
                }
            }
        }
    }

    Ok(UserConfig {
        api_url: api_url.ok_or_else(|| ConfigError("API_URL not set".to_string()))?,
        api_key: api_key.ok_or_else(|| ConfigError("API_KEY not set".to_string()))?,
        watch_time: watch_time.ok_or_else(|| ConfigError("WATCH_TIME not set".to_string()))?,
        media_enable: media_enable
            .ok_or_else(|| ConfigError("MEDIA_ENABLE not set".to_string()))?,
        log_enable: log_enable.ok_or_else(|| ConfigError("LOG_ENABLE not set".to_string()))?,
    })
}

pub fn init() -> Result<(String, String, i64, bool, bool), Box<dyn Error>> {
    let matches = App::new("Process Report For Linux")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .default_value(".env.process") // Set default config file path
                .help("Sets the config file path"),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let config_path = std::env::current_dir()?.join(config_file);

    let user_config = read_config_values(config_path.to_str().unwrap())?;
    Ok((
        user_config.api_url,
        user_config.api_key,
        user_config.watch_time,
        user_config.media_enable,
        user_config.log_enable,
    ))
}
