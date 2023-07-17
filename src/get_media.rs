/*
 * @Author: timochan
 * @Date: 2023-07-17 15:23:40
 * @LastEditors: timochan
 * @LastEditTime: 2023-07-17 21:29:09
 * @FilePath: /processforlinux/src/get_media.rs
 */
// use dbus::arg::RefArg;
// use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
// use dbus::blocking::Connection;
//TODO: get media name and artist
// pub fn get_media_name() -> String {
//     let connection = Connection::new_session().unwrap();
//     let mut play_title = "";
//     let proxy = connection.with_proxy(
//         "org.mpris.MediaPlayer2.yesplaymusic",
//         "/org/mpris/MediaPlayer2",
//         std::time::Duration::from_millis(5000),
//     );

//     let metadata: std::collections::HashMap<
//         String,
//         dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>,
//     > = proxy
//         .get("org.mpris.MediaPlayer2.Player", "Metadata")
//         .unwrap();

//     if let Some(title) = metadata.get("xesam:title") {
//         if let Some(title_str) = title.as_str() {
//             play_title = title_str;
//         }
//     }
//     return play_title.to_string();
// }
