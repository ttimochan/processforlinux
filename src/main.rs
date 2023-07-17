/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-17 17:37:00
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
        let (api_url, api_key, report_time) = get_env_file::init().unwrap();
        let media_title = get_media::get_media_name();
        let process_name = get_active_window::get_active_window_process_and_title().unwrap();

        rt.block_on(report(&process_name, &media_title, &api_key, &api_url));
        std::thread::sleep(std::time::Duration::from_secs(
            report_time.parse::<u64>().unwrap(),
        ));
    }
}
async fn report(process_name: &str, media_title: &str, api_key: &str, api_url: &str) {
    if let Err(err) =
        reportprocess::process_report(&process_name, &media_title, &api_key, &api_url).await
    {
        eprintln!("Error: {}", err);
    }
}
