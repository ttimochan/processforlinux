/*
 * @Author: timochan
 * @Date: 2023-07-17 13:51:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-22 10:41:09
 * @FilePath: /processforlinux/src/get_env_file.rs
 */
use clap::{App, Arg};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConfigError {}

fn read_config_values(
    config_path: &str,
) -> Result<(String, String, String, String, String), Box<dyn Error>> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let mut api_url = None;
    let mut api_key = None;
    let mut report_time = None;
    let mut media_enable = None;
    let mut log_enable = None;

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
                "REPORT_TIME" => report_time = Some(value.to_string()),
                "MEDIA_ENABLE" => media_enable = Some(value.to_string()),
                "LOG_ENABLE" => log_enable = Some(value.to_string()),
                _ => {
                    // Handle unknown or invalid key-value pairs
                    eprintln!("Unknown key-value pair: {}", trimmed_line);
                }
            }
        }
    }

    if let (Some(api_url), Some(api_key), Some(report_time), Some(media_enable), Some(log_enable)) =
        (api_url, api_key, report_time, media_enable, log_enable)
    {
        Ok((api_url, api_key, report_time, media_enable, log_enable))
    } else {
        Err(Box::new(ConfigError(
            "Failed to read config values".to_string(),
        )))
    }
}

pub fn init() -> Result<(String, String, String, String, String), Box<dyn Error>> {
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
    let config_path = env::current_dir()?.join(config_file);

    let (api_url, api_key, report_time, media_enable, log_enable) =
        read_config_values(config_path.to_str().unwrap())?;

    Ok((api_url, api_key, report_time, media_enable, log_enable))
}
