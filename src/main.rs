/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-27 20:36:27
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use chrono::Utc;
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
    static ref PREVIOUS_PROCESS_NAME: Mutex<Option<String>> = Mutex::new(None);
    static ref PREVIOUS_MEDIA_METADATA: Mutex<Option<get_media::MediaMetadata>> = Mutex::new(None);
}

async fn run_loop(
    api_key: &str,
    api_url: &str,
    watch_time: i64,
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
        let prev_process_name = PREVIOUS_PROCESS_NAME.lock().unwrap().clone();
        let prev_media_metadata = PREVIOUS_MEDIA_METADATA.lock().unwrap().clone();

        if prev_process_name.as_ref() != Some(&process_name)
            || prev_media_metadata.as_ref() != Some(&media_metadata)
        {
            if let Err(e) = report(
                &process_name,
                &media_metadata.title.clone().unwrap_or_default(),
                &media_metadata.artist.clone().unwrap_or_default(),
                &api_key,
                &api_url,
                watch_time,
                log_enable,
            )
            .await
            {
                eprintln!("Failed to report: {}", e);
            }

            *PREVIOUS_PROCESS_NAME.lock().unwrap() = Some(process_name.clone());
            *PREVIOUS_MEDIA_METADATA.lock().unwrap() = Some(media_metadata.clone());
        } else {
            if log_enable {
                let utc_now = Utc::now();
                let next_watch_time = utc_now
                    .checked_add_signed(chrono::Duration::seconds(watch_time))
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S");

                println!("--------------------------------------------------");
                println!("No change in process or media metadata");
                println!("Next Watch Time : {}", next_watch_time);
                println!("--------------------------------------------------");
            }
        }
        let sleep_interval_secs = watch_time.to_string().parse::<u64>().unwrap_or(60);
        sleep(Duration::from_secs(sleep_interval_secs)).await;
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    media_artist: &str,
    api_key: &str,
    api_url: &str,
    watch_time: i64,
    log_enable: bool,
) -> Result<(), Box<dyn Error>> {
    reportprocess::process_report(
        process_name,
        media_title,
        media_artist,
        api_key,
        api_url,
        watch_time,
        log_enable,
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let (api_url, api_key, watch_time, media_enable, log_enable) = {
        let api_vars = API_VARIABLES.lock().unwrap();
        api_vars.clone()
    };
    run_loop(&api_key, &api_url, watch_time, media_enable, log_enable).await;
}
