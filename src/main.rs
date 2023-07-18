/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-18 11:44:32
 * @FilePath: /processforlinux/src/main.rs
 */
mod get_active_window;
mod get_env_file;
mod get_media;
mod reportprocess;

use tokio::runtime::Runtime;

fn main() {
    loop {
        let rt = Runtime::new().unwrap();
        let (api_url, api_key, report_time, media_enable, log_enable) =
            get_env_file::init().unwrap();

        let media_title = match media_enable.as_str() {
            "true" => match get_media::get_media_name::<dbus::Error>() {
                Some(title) => title,
                None => String::from("None"),
            },
            "false" => String::from("None"),
            _ => String::new(),
        };

        let process_name = get_active_window::get_active_window_process_and_title().unwrap();

        rt.block_on(report(
            &process_name,
            &media_title,
            &api_key,
            &api_url,
            &report_time,
            &log_enable,
        ));

        std::thread::sleep(std::time::Duration::from_secs(
            report_time.parse::<u64>().unwrap(),
        ));
    }
}

async fn report(
    process_name: &str,
    media_title: &str,
    api_key: &str,
    api_url: &str,
    report_time: &str,
    log_enable: &str,
) {
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
    }
}
