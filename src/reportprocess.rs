/*
 * @Author: timochan
 * @Date: 2023-07-17 13:50:34
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-18 11:47:40
 * @FilePath: /processforlinux/src/reportprocess.rs
 */
use chrono::Utc;
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn process_report(
    process_name: &str,
    media_title: &str,
    api_key: &str,
    api_url: &str,
    report_time: &str,
    log_enable: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let utc_now = Utc::now();
    let this_report_time = utc_now.format("%Y-%m-%d %H:%M:%S");
    let next_report_time = utc_now
        .checked_add_signed(chrono::Duration::seconds(
            report_time.parse::<i64>().unwrap(),
        ))
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();
    let payload = if media_title == "None" {
        json!({
            "timestamp": timestamp,
            "process": process_name,
            "key": api_key
        })
    } else {
        json!({
            "timestamp": timestamp,
            "process": process_name,
            "key": api_key,
            "media": {
                "title": media_title,
                "artist": "未知"
            }
        })
    };
    let client = Client::builder().build()?;
    let url = api_url;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 uacq"),
    );
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
        let parsed_response: serde_json::Value = serde_json::from_str(&response)?;
        println!("--------------------------------------------------");
        println!("This Report Time: {}", this_report_time);
        println!("Payload: {}", payload);
        println!("Response: {}", parsed_response);
        println!("Next Report Time: {}", next_report_time);
        println!("--------------------------------------------------");
    }

    Ok(())
}
