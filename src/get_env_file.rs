/*
 * @Author: timochan
 * @Date: 2023-07-17 13:51:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-17 14:39:50
 * @FilePath: /processforlinux/src/get_env_file.rs
 */
use clap::{App, Arg};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command};

fn read_config_values(config_path: &str) -> Option<(String, String, String)> {
    let file = File::open(config_path).ok()?;
    let reader = BufReader::new(file);

    let mut api_url = None;
    let mut api_key = None;
    let mut report_time = None;

    for line in reader.lines() {
        let line = line.ok()?;
        if line.starts_with("API_URL=") {
            api_url = Some(line.trim_start_matches("API_URL=").to_string());
        } else if line.starts_with("API_KEY=") {
            api_key = Some(line.trim_start_matches("API_KEY=").to_string());
        } else if line.starts_with("REPORT_TIME=") {
            report_time = Some(line.trim_start_matches("REPORT_TIME=").to_string());
        }
    }

    api_url.and_then(|api_url| {
        api_key.and_then(|api_key| {
            report_time.and_then(|report_time| Some((api_url, api_key, report_time)))
        })
    })
}

pub fn init() -> Result<(String, String, String), Box<dyn Error>> {
    let matches = App::new("Process Report For Linux")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets the config file path"),
        )
        .arg(
            Arg::with_name("daemon")
                .short("d")
                .long("daemon")
                .takes_value(false)
                .help("Run in background"),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(".env.process");
    let config_path = env::current_dir()?.join(config_file);

    let (api_url, api_key, report_time) = if let Some(path) = matches.value_of("config") {
        read_config_values(path).ok_or("Failed to read config values")?
    } else {
        read_config_values(config_path.to_str().unwrap())
            .or_else(|| read_config_values(".env.process"))
            .ok_or("Failed to read config values")?
    };

    if matches.is_present("daemon") {
        match Command::new(std::env::current_exe()?)
            .arg("-c")
            .arg(config_file)
            .arg("-d")
            .spawn()
        {
            Ok(_) => {
                println!("CLI is running in the background");
                exit(0);
            }
            Err(e) => {
                eprintln!("Failed to start the background process: {}", e);
                exit(1);
            }
        }
    }

    println!("API URL: {}", api_url);
    println!("API Key: {}", api_key);
    println!("Report Time: {}", report_time);
    Ok((api_url, api_key, report_time))
}
