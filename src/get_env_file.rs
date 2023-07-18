/*
 * @Author: timochan
 * @Date: 2023-07-17 13:51:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-18 17:39:49
 * @FilePath: /processforlinux/src/get_env_file.rs
 */
use clap::{App, Arg};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_config_values(config_path: &str) -> Option<(String, String, String, String, String)> {
    let file = File::open(config_path).ok()?;
    let reader = BufReader::new(file);
    let mut api_url = None;
    let mut api_key = None;
    let mut report_time = None;
    let mut media_enable = None;
    let mut log_enable = None;

    for line_result in reader.lines() {
        let line = line_result.ok()?;
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
                _ => {}
            }
        }
    }

    if let (Some(api_url), Some(api_key), Some(report_time), Some(media_enable), Some(log_enable)) =
        (api_url, api_key, report_time, media_enable, log_enable)
    {
        Some((api_url, api_key, report_time, media_enable, log_enable))
    } else {
        None
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
                .help("Sets the config file path"),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(".env.process");
    let config_path = env::current_dir()?.join(config_file);

    let (api_url, api_key, report_time, media_enable, log_enable) =
        if let Some(path) = matches.value_of("config") {
            read_config_values(path).ok_or("Failed to read config values")?
        } else {
            read_config_values(config_path.to_str().unwrap())
                .or_else(|| read_config_values(".env.process"))
                .ok_or("Failed to read config values")?
        };
    Ok((api_url, api_key, report_time, media_enable, log_enable))
}
