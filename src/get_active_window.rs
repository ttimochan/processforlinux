/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-17 19:24:32
 * @FilePath: /processforlinux/src/get_active_window.rs
*/
use std::error::Error;
use std::process::Command;

pub fn get_active_window_process_and_title() -> Result<String, Box<dyn Error>> {
    let xprop_output = Command::new("xprop")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .output()?;

    let active_window_id = parse_xprop_output(&xprop_output.stdout)?;

    let xwininfo_output = Command::new("xwininfo")
        .arg("-id")
        .arg(&active_window_id)
        .output()?;

    let window_title = parse_xwininfo_output(&xwininfo_output.stdout)?;

    let process_name = get_last_part(&window_title).ok_or("Failed to get process name")?;

    if process_name == "Visual Studio Code" {
        return Ok("Code".to_string());
    }

    Ok(process_name)
}

fn parse_xprop_output(output: &[u8]) -> Result<String, Box<dyn Error>> {
    let xprop_output = String::from_utf8_lossy(output);
    let active_window_line = xprop_output
        .lines()
        .find(|line| line.contains("_NET_ACTIVE_WINDOW(WINDOW)"))
        .ok_or("Failed to find active window line")?;
    let active_window_id = active_window_line
        .split_whitespace()
        .nth(4)
        .ok_or("Failed to extract active window ID")?;
    Ok(active_window_id.to_string())
}

fn parse_xwininfo_output(output: &[u8]) -> Result<String, Box<dyn Error>> {
    let xwininfo_output = String::from_utf8_lossy(output);
    let window_title_line = xwininfo_output
        .lines()
        .find(|line| line.contains("xwininfo: Window id:"))
        .ok_or("Failed to find window title line")?;
    let window_name_parts: Vec<&str> = window_title_line.split('"').collect();
    let window_title = window_name_parts.get(1).unwrap_or(&"").to_string();
    Ok(window_title)
}

fn get_last_part(input: &str) -> Option<String> {
    let separator = " - ";
    let last_part = input.rsplit_once(separator)?.1.trim();

    if last_part.is_empty() {
        None
    } else {
        Some(last_part.to_string())
    }
}
