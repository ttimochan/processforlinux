/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-24 18:23:33
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

        let media_metadata = if media_enable {
            get_media::get_media_metadata().unwrap_or_default()
        } else {
            get_media::MediaMetadata::default()
        };

        let process_name = match get_active_window::get_active_window_process_and_title() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Failed to get active window: {}", e);
                continue;
            }
        };

        if let Err(e) = report(
            &process_name,
            &media_metadata.title.as_deref().unwrap_or_default(),
            &media_metadata.artist.as_deref().unwrap_or_default(),
            &api_key,
            &api_url,
            report_time,
            log_enable,
        )
        .await
        {
            eprintln!("Failed to report: {}", e);
        }

        let report_interval_secs = report_time.to_string().parse::<u64>().unwrap_or(60);
        sleep(Duration::from_secs(report_interval_secs)).await;
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    media_artist: &str,
    api_key: &str,
    api_url: &str,
    report_time: i64,
    log_enable: bool,
) -> Result<(), Box<dyn Error>> {
    reportprocess::process_report(
        process_name,
        media_title,
        media_artist,
        api_key,
        api_url,
        report_time,
        log_enable,
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    run_loop().await;
}
