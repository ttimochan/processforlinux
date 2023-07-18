/*
 * @Author: timochan
 * @Date: 2023-07-17 15:23:40
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-18 10:15:01
 * @FilePath: /processforlinux/src/get_media.rs
 */
use dbus::arg::RefArg;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::{Connection, Proxy};

pub fn get_media_name<E>() -> Option<String> {
    let media_player_identifiers = [
        "org.mpris.MediaPlayer2.yesplaymusic",
        "org.mpris.MediaPlayer2.netease-cloud-music",
    ];

    for &identifier in &media_player_identifiers {
        if let Ok(connection) = Connection::new_session() {
            let proxy_result: Result<Proxy<&Connection>, _> = Ok::<
                dbus::blocking::Proxy<'_, &dbus::blocking::Connection>,
                E,
            >(connection.with_proxy(
                identifier,
                "/org/mpris/MediaPlayer2",
                std::time::Duration::from_millis(5000),
            ));

            let proxy = match proxy_result {
                Ok(proxy) => proxy,
                Err(_) => continue, // Try the next media player identifier.
            };

            let metadata: std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>,
            > = match proxy.get("org.mpris.MediaPlayer2.Player", "Metadata") {
                Ok(metadata) => metadata,
                Err(_) => continue, // Try the next media player identifier.
            };

            if let Some(title) = metadata.get("xesam:title") {
                if let Some(title_str) = title.as_str() {
                    return Some(title_str.to_string());
                }
            }
        }
    }

    None // Return None if no valid media player connection is found.
}
