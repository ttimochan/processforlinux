/*
 * @Author: timochan
 * @Date: 2023-07-17 15:23:40
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-27 20:03:14
 * @FilePath: /processforlinux/src/get_media.rs
 */
use dbus::arg::RefArg;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use dbus::blocking::Proxy;

#[derive(Clone, PartialEq)]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
}
impl Default for MediaMetadata {
    fn default() -> Self {
        MediaMetadata {
            title: None,
            artist: None,
        }
    }
}
mod constants {
    pub const MEDIA_PLAYER_IDENTIFIERS: [&str; 2] = [
        "org.mpris.MediaPlayer2.yesplaymusic",
        "org.mpris.MediaPlayer2.netease-cloud-music",
    ];
    pub const MPRIS_PLAYER_INTERFACE: &str = "org.mpris.MediaPlayer2.Player";
    pub const METADATA_PROPERTY: &str = "Metadata";
}
mod media {
    pub const TITLE_KEY: &str = "xesam:title";
    pub const ARTIST_KEY: &str = "xesam:artist";
}

pub fn get_media_metadata() -> Option<MediaMetadata> {
    for &identifier in &constants::MEDIA_PLAYER_IDENTIFIERS {
        if let Ok(connection) = Connection::new_session() {
            let proxy_result: Result<Proxy<&Connection>, dbus::Error> = Ok(connection.with_proxy(
                identifier,
                "/org/mpris/MediaPlayer2",
                std::time::Duration::from_millis(5000),
            ));

            let proxy = match proxy_result {
                Ok(proxy) => proxy,
                Err(_) => continue,
            };

            let metadata: std::collections::HashMap<String, dbus::arg::Variant<Box<dyn RefArg>>> =
                match proxy.get(
                    constants::MPRIS_PLAYER_INTERFACE,
                    constants::METADATA_PROPERTY,
                ) {
                    Ok(metadata) => metadata,
                    Err(_) => continue, // Try the next media player identifier.
                };

            let title = metadata
                .get(media::TITLE_KEY)
                .and_then(|title| title.as_str())
                .map(String::from);

            let artist = if let Some(artist_variant) = metadata.get(media::ARTIST_KEY) {
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
