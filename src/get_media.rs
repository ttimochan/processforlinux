/*
 * @Author: timochan
 * @Date: 2023-07-17 15:23:40
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-20 15:23:01
 * @FilePath: /processforlinux/src/get_media.rs
 */
use dbus::arg::RefArg;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::{Connection, Proxy};

pub struct MediaMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
}

pub fn get_media_metadata() -> Option<MediaMetadata> {
    let media_player_identifiers = [
        "org.mpris.MediaPlayer2.yesplaymusic",
        "org.mpris.MediaPlayer2.netease-cloud-music",
    ];

    for &identifier in &media_player_identifiers {
        if let Ok(connection) = Connection::new_session() {
            let proxy_result: Result<Proxy<&Connection>, _> = Ok::<
                dbus::blocking::Proxy<'_, &dbus::blocking::Connection>,
                dbus::Error,
            >(connection.with_proxy(
                identifier,
                "/org/mpris/MediaPlayer2",
                std::time::Duration::from_millis(5000),
            ));

            let proxy = match proxy_result {
                Ok(proxy) => proxy,
                Err(_) => continue,
            };

            let metadata: std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>,
            > = match proxy.get("org.mpris.MediaPlayer2.Player", "Metadata") {
                Ok(metadata) => metadata,
                Err(_) => continue, // Try the next media player identifier.
            };

            let title = metadata
                .get("xesam:title")
                .and_then(|title| title.as_str())
                .map(|title_str| title_str.to_string());

            let artist = if let Some(artist_variant) = metadata.get("xesam:artist") {
                match artist_variant {
                    dbus::arg::Variant(boxed_value) => {
                        if let Some(artist_str) = boxed_value.as_str() {
                            Some(artist_str.to_string())
                        } else if let Some(artist_array) = boxed_value.as_iter() {
                            let artists: Vec<String> = artist_array
                                .filter_map(|a| a.as_str().map(String::from))
                                .collect();

                            if !artists.is_empty() {
                                let artists_str = artists.join(", ");
                                Some(artists_str)
                            } else {
                                println!("No artist information available.");
                                None
                            }
                        } else {
                            println!("Unknown artist format.");
                            None
                        }
                    }
                }
            } else {
                None
            };

            if title.is_some() || artist.is_some() {
                return Some(MediaMetadata { title, artist });
            }
        }
    }

    None // Return None if no valid media player connection or metadata is found.
}
