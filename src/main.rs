/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-27 18:26:07
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use lazy_static::lazy_static;
use std::process::exit;
use std::{error::Error, io::Write, sync::Mutex, time::Duration};
use tokio::time::sleep;

type ApiVariables = (String, String, i64, bool, bool);
lazy_static! {
    static ref API_VARIABLES: Mutex<ApiVariables> =
        Mutex::new(get_env_file::init().unwrap_or_else(|err| {
            eprintln!("Failed to get env file: {}", err);
            exit(1);
        }));
}

async fn run_loop(
    api_key: &str,
    api_url: &str,
    report_time: i64,
    media_enable: bool,
    log_enable: bool,
) {
    loop {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();

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
            &media_metadata.title.unwrap_or_default(),
            &media_metadata.artist.unwrap_or_default(),
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
    let (api_url, api_key, report_time, media_enable, log_enable) = {
        let api_vars = API_VARIABLES.lock().unwrap();
        api_vars.clone()
    };
    run_loop(&api_key, &api_url, report_time, media_enable, log_enable).await;
}
