/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-17 17:32:09
 * @FilePath: /processforlinux/src/get_active_window.rs
*/
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
pub fn get_active_window_process_and_title() -> Result<String, Box<dyn Error>> {
    let mut window_title = String::new();
    let xprop_output = Command::new("xprop")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .stdout(Stdio::piped())
        .spawn()?;

    let xprop_stdout = xprop_output
        .stdout
        .ok_or("Failed to capture xprop stdout")?;

    let xprop_reader = BufReader::new(xprop_stdout);
    let mut active_window_id = String::new();
    for line in xprop_reader.lines() {
        let line = line?;
        if line.contains("_NET_ACTIVE_WINDOW(WINDOW)") {
            active_window_id = line.split_whitespace().nth(4).unwrap_or("").to_string();
            break;
        }
    }

    let xwininfo_output = Command::new("xwininfo")
        .arg("-id")
        .arg(active_window_id)
        .stdout(Stdio::piped())
        .spawn()?;

    let xwininfo_stdout = xwininfo_output
        .stdout
        .ok_or("Failed to capture xwininfo stdout")?;
    let xwininfo_reader = BufReader::new(xwininfo_stdout);

    for line in xwininfo_reader.lines() {
        let line = line?;
        if line.contains("xwininfo: Window id:") {
            let window_name_parts: Vec<&str> = line.split('"').collect();
            window_title = window_name_parts[1].to_string();
        }
    }
    let xwininfo_result = &window_title;
    let process_name = get_last_part(xwininfo_result).ok_or("Failed to get process name")?;

    if process_name == "Visual Studio Code" {
        return Ok("Code".to_string());
    }
    Ok(process_name)
}
fn get_last_part(input: &str) -> Option<String> {
    let separator = " - ";
    let last_part = input.rsplitn(2, separator).next().unwrap_or("").trim();

    if last_part.is_empty() {
        None
    } else {
        Some(String::from(last_part))
    }
}
