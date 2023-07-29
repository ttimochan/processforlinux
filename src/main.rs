/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-29 11:25:42
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use chrono::Utc;

use std::process::exit;
use std::{error::Error, time::Duration};
use tokio::time::sleep;

struct Config {
    api_url: String,
    api_key: String,
    watch_time: i64,
    media_enable: bool,
    log_enable: bool,
}

async fn run_loop(config: Config) {
    let mut last_time = Utc::now();
    let mut previous_process_name = String::new();
    let mut previous_media_metadata: get_media::MediaMetadata = get_media::MediaMetadata::default();
    loop {
        let utc_now = Utc::now();
        let media_metadata = if config.media_enable {
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

        let prev_process_name = previous_process_name.clone();
        let prev_media_metadata = previous_media_metadata.clone();

        if prev_process_name != process_name
            || prev_media_metadata != media_metadata
            || (utc_now - last_time).num_seconds() > 60
        {
            if let Err(e) = report(
                &process_name,
                &media_metadata.title.clone().unwrap_or_default(),
                &media_metadata.artist.clone().unwrap_or_default(),
                &config,
            )
            .await
            {
                eprintln!("Failed to report: {}", e);
            }

            previous_process_name = process_name;
            previous_media_metadata = media_metadata;
            last_time = utc_now;
        } else {
            if config.log_enable {
                let utc_now = Utc::now();
                let next_watch_time = utc_now
                    .checked_add_signed(chrono::Duration::seconds(config.watch_time))
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S");
                let utc_now = utc_now.format("%Y-%m-%d %H:%M:%S");
                println!("--------------------------------------------------");
                println!("This Watch Time : {}", utc_now);
                println!("No change in process or media metadata");
                println!("Next Watch Time : {}", next_watch_time);
                println!("--------------------------------------------------");
            }
        }
        let sleep_interval_secs = config.watch_time.to_string().parse::<u64>().unwrap_or(5);
        sleep(Duration::from_secs(sleep_interval_secs)).await;
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    media_artist: &str,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    reportprocess::process_report(
        process_name,
        media_title,
        media_artist,
        &config.api_key,
        &config.api_url,
        config.watch_time,
        config.log_enable,
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let (api_url, api_key, watch_time, media_enable, log_enable) = {
        let api_vars = get_env_file::init();
        match api_vars {
            Ok(api_vars) => api_vars,
            Err(e) => {
                eprintln!("Failed to get api variables: {}", e);
                exit(1);
            }
        }
    };
    let config = Config {
        api_url: api_url,
        api_key: api_key,
        watch_time: watch_time,
        media_enable: media_enable,
        log_enable: log_enable,
    };
    run_loop(config).await;
}
