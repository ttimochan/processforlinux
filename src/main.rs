/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-19 16:24:11
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use std::{error::Error, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() {
    loop {
        match Runtime::new() {
            Ok(rt) => {
                let (api_url, api_key, report_time, media_enable, log_enable) =
                    match get_env_file::init() {
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
                    "false" => String::from("None"),
                    _ => String::new(),
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
                thread::sleep(Duration::from_secs(
                    report_time.parse::<u64>().unwrap_or(60),
                ));
            }
            Err(e) => {
                eprintln!("Failed to create runtime: {}", e);
                continue;
            }
        }
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
