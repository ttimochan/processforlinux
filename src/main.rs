/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-19 21:02:55
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use std::{error::Error, io::Write, time::Duration};
use tokio::runtime::Runtime;
use tokio::time::sleep;

async fn run_loop() {
    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Failed to create runtime: {}", e);
            return;
        }
    };

    loop {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();

        let (api_url, api_key, report_time, media_enable, log_enable) = match get_env_file::init() {
            Ok(values) => values,
            Err(e) => {
                eprintln!("Failed to initialize environment: {}", e);
                break;
            }
        };

        let media_title = match media_enable.as_str() {
            "true" => match get_media::get_media_name::<dbus::Error>() {
                Some(title) => title,
                None => String::from("None"),
            },
            _ => String::from("None"),
        };

        let process_name = match get_active_window::get_active_window_process_and_title() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Failed to get active window: {}", e);
                continue;
            }
        };

        if let Err(e) = rt.block_on(report(
            &process_name,
            &media_title,
            &api_key,
            &api_url,
            &report_time,
            &log_enable,
        )) {
            eprintln!("Failed to report: {}", e);
        }

        let report_interval_secs = report_time.parse::<u64>().unwrap_or(60);
        sleep(Duration::from_secs(report_interval_secs)).await;
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    api_key: &str,
    api_url: &str,
    report_time: &str,
    log_enable: &str,
) -> Result<(), Box<dyn Error>> {
    if let Err(err) = reportprocess::process_report(
        &process_name,
        &media_title,
        &api_key,
        &api_url,
        &report_time,
        &log_enable,
    )
    .await
    {
        eprintln!("Error: {}", err);
        return Err(err.into());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    run_loop().await;
}
