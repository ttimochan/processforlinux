/*
 * @Author: timochan
 * @Date: 2023-07-17 13:50:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-19 21:09:57
 * @FilePath: /processforlinux/src/reportprocess.rs
 */
use chrono::Utc;
use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use serde_json::json;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 uacq";
const MEDIA_ARTIST_UNKNOWN: &str = "未知";
const CONTENT_TYPE: &str = "application/json";

pub async fn process_report(
    process_name: &str,
    media_title: &str,
    api_key: &str,
    api_url: &str,
    report_time: &str,
    log_enable: &str,
) -> Result<(), Box<dyn Error>> {
    let utc_now = Utc::now();
    let this_report_time = utc_now.format("%Y-%m-%d %H:%M:%S").to_string();

    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => {
            eprintln!("SystemTime before UNIX EPOCH!");
            return Ok(());
        }
    };

    let payload = match media_title {
        "None" => json!({
            "api_key": api_key,
            "process_name": process_name,
            "timestamp": timestamp,
        }),
        _ => json!({
            "timestamp": timestamp,
            "process": process_name,
            "key": api_key,
            "media": {
                "title": media_title,
                "artist": MEDIA_ARTIST_UNKNOWN,
            },
        }),
    };

    let client = Client::builder().build()?;
    let url = api_url;

    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE));
    headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

    let response = match process_name {
        "None" => "None".to_string(),
        _ => {
            client
                .post(url)
                .headers(headers)
                .body(payload.to_string())
                .send()
                .await?
                .text()
                .await?
        }
    };

    if log_enable == "true" {
        let next_report_time = utc_now
            .checked_add_signed(chrono::Duration::seconds(
                report_time.parse::<i64>().unwrap_or(60),
            ))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S");

        println!("--------------------------------------------------");
        println!("This Report Time: {}", this_report_time);
        println!("Response: {}", &response);
        println!("Next Report Time: {}", next_report_time);
        println!("--------------------------------------------------");
    }

    Ok(())
}
