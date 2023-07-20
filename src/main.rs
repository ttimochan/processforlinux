/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-20 12:35:16
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use std::{error::Error, io::Write, time::Duration};
use tokio::time::sleep;

async fn run_loop() {
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
        let (media_title, media_artist) = match media_enable.as_str() {
            "true" => match get_media::get_media() {
                Some((title, artist)) => (title, artist),
                None => (String::from("None"), String::new()),
            },
            _ => (String::from("None"), String::new()),
        };
        println!(
            "media_title: {} media_artlist: {:?}",
            media_title, media_artist
        );
        let process_name = match get_active_window::get_active_window_process_and_title() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Failed to get active window: {}", e);
                continue;
            }
        };

        if let Err(e) = report(
            &process_name,
            &media_title,
            &media_artist,
            &api_key,
            &api_url,
            &report_time,
            &log_enable,
        )
        .await
        {
            eprintln!("Failed to report: {}", e);
        }

        let report_interval_secs = report_time.parse::<u64>().unwrap_or(60);
        sleep(Duration::from_secs(report_interval_secs)).await;
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    media_artist: &str,
    api_key: &str,
    api_url: &str,
    report_time: &str,
    log_enable: &str,
) -> Result<(), Box<dyn Error>> {
    if let Err(err) = reportprocess::process_report(
        &process_name,
        &media_title,
        &media_artist,
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
