/*
 * @Author: timochan
 * @Date: 2023-07-17 13:51:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-24 17:59:14
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

struct UserConfig {
    api_url: String,
    api_key: String,
    report_time: i64,
    media_enable: bool,
    log_enable: bool,
}
fn read_config_values(
    config_path: &str,
) -> Result<(String, String, i64, bool, bool), Box<dyn Error>> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let mut user_config = UserConfig {
        api_url: "".to_string(),
        api_key: "".to_string(),
        report_time: 60,
        media_enable: false,
        log_enable: false,
    };

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('#') || trimmed_line.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed_line.split_once('=') {
            match key {
                "API_URL" => user_config.api_url = value.to_string(),
                "API_KEY" => user_config.api_key = value.to_string(),
                "REPORT_TIME" => user_config.report_time = value.parse::<i64>().unwrap_or_default(),
                "MEDIA_ENABLE" => {
                    user_config.media_enable = value.parse::<bool>().unwrap_or_default()
                }
                "LOG_ENABLE" => user_config.log_enable = value.parse::<bool>().unwrap_or_default(),
                _ => {
                    // Handle unknown or invalid key-value pairs
                    eprintln!("Unknown key-value pair: {}", trimmed_line);
                }
            }
        }
    }

    let (api_url, api_key, report_time, media_enable, log_enable) = (
        user_config.api_url,
        user_config.api_key,
        user_config.report_time,
        user_config.media_enable,
        user_config.log_enable,
    );
    
    Ok((api_url, api_key, report_time, media_enable, log_enable))
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
    let config_path = env::current_dir()?.join(config_file);

    let mut user_config = UserConfig {
        api_url: "".to_string(),
        api_key: "".to_string(),
        report_time: 60,
        media_enable: false,
        log_enable: false,
    };
    (
        user_config.api_url,
        user_config.api_key,
        user_config.report_time,
        user_config.media_enable,
        user_config.log_enable,
    ) = read_config_values(config_path.to_str().unwrap_or_default())?;

    Ok((
        user_config.api_url,
        user_config.api_key,
        user_config.report_time,
        user_config.media_enable,
        user_config.log_enable,
    ))
}
