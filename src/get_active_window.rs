/*
 * @Author: timochan
 * @Date: 2023-07-17 11:48:02
 * @LastEditors: timochan
 * @LastEditTime: 2023-10-30 22:22:22
 * @FilePath: /processforlinux/src/get_active_window.rs
*/

/*
 * It seems that 'xprop' can get the title directly.
 */
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

enum WindowTitle {
    Code,
    WebStorm,
    Telegram,
    WeChat,
    Discord,
    Mail,
    QQ,
    Chrome,
    QQ音乐,
    NetEaseMusic,
    iTerm2,
    Typora,
    None,
}

impl std::fmt::Display for WindowTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WindowTitle::Code => write!(f, "Code"),
            WindowTitle::WebStorm => write!(f, "WebStorm"),
            WindowTitle::Telegram => write!(f, "Telegram"),
            WindowTitle::WeChat => write!(f, "WeChat"),
            WindowTitle::Discord => write!(f, "Discord"),
            WindowTitle::Mail => write!(f, "Mail"),
            WindowTitle::QQ => write!(f, "QQ"),
            WindowTitle::Chrome => write!(f, "Chrome"),
            WindowTitle::QQ音乐 => write!(f, "QQ音乐"),
            WindowTitle::NetEaseMusic => write!(f, "NetEaseMusic"),
            WindowTitle::iTerm2 => write!(f, "iTerm2"),
            WindowTitle::Typora => write!(f, "Typora"),
            WindowTitle::None => write!(f, "None"),
        }
    }
}

impl WindowTitle {
    fn from_string(s: &str) -> WindowTitle {
        match s {
            "code" => WindowTitle::Code,
            "jetbrains-webstorm" => WindowTitle::WebStorm,
            "telegram" => WindowTitle::Telegram, // TODO: Can't get the title of Telegram
            "wechat" => WindowTitle::WeChat,
            "discord" => WindowTitle::Discord, // TODO: Can't test
            "thunderbird" => WindowTitle::Mail,
            "kmail" => WindowTitle::Mail, // TODO: Can't get the title of KMail
            "qq" => WindowTitle::QQ,
            "google-chrome" => WindowTitle::Chrome,
            "chromium" => WindowTitle::Chrome,
            "thorium" => WindowTitle::Chrome,
            "qqmusic" => WindowTitle::QQ音乐,
            "music" => WindowTitle::NetEaseMusic,
            "yesplaymusic" => WindowTitle::NetEaseMusic,
            "yakuake" => WindowTitle::iTerm2, // TODO: Can't get the title of Yakuake
            "konsole" => WindowTitle::iTerm2, // TODO: Can't get the title of Konsole
            "typora" => WindowTitle::Typora,
            _ => WindowTitle::None,
        }
    }
}

pub fn get_active_window_process_and_title() -> Result<String, Box<dyn Error>> {
    let xprop_output = Command::new("xprop")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or("Failed to capture xprop stdout")?;

    let xprop_reader = BufReader::new(xprop_output);
    let mut window_id = String::new();
    for line in xprop_reader.lines() {
        let line = line?;
        if line.contains("_NET_ACTIVE_WINDOW(WINDOW)") {
            window_id = line.split_whitespace().nth(4).unwrap_or("").to_string();
            break;
        }
    }

    if window_id.is_empty() {
        return Err("Failed to get active window ID".into());
    }

    let xprop_output = Command::new("xprop")
        .arg("-id")
        .arg(&window_id)
        .arg("WM_CLASS")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or("Failed to capture xprop stdout")?;

    let xprop_reader = BufReader::new(xprop_output);
    for line in xprop_reader.lines() {
        let line = line?;
        if line.contains("WM_CLASS(STRING)") {
            let class_name = line.split('"').nth(1).unwrap_or("");
            println!("class_name: {}", class_name);
            let window_title_enum = WindowTitle::from_string(class_name);
            return Ok(window_title_enum.to_string());
        }
    }

    Err("Failed to get window class".into())
}
