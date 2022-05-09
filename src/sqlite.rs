// database
use crate::config::Termusic;
// use crate::track::Track;
// use rusqlite::{Connection, Result};
use crate::config::get_app_config_path;
// use crate::ui::model::Model;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct SqliteDB {
    conn: Connection,
    config: Termusic,
}

#[allow(unused)]
impl SqliteDB {
    pub fn new(config: &Termusic) -> Self {
        let mut db_path = get_app_config_path().expect("failed to get app configuration path");
        db_path.push("library.db");
        let conn = Connection::open(db_path).expect("open db failed");

        // USLT lyrics
        // lyric_frames: Vec<Lyrics>,
        // lyric_selected_index: usize,
        // parsed_lyric: Option<Lyric>,
        // picture: Option<Picture>,
        // album_photo: Option<String>,
        // file_type: Option<FileType>,
        conn.execute(
            "create table if not exists track(
             id integer primary key,
             artist   TEXT NOT NULL,
             album    TEXT NOT NULL,
             title    TEXT NOT NULL,
             file     TEXT NOT NULL UNIQUE,
             duration DOUBLE NOT NULL,
             name     TEXT NOT NULL,
             ext     TEXT NOT NULL
         )",
            [],
        )
        .expect("create table track failed");
        conn.execute(
            "create table if not exists directory(
             id integer primary key,
             name text not null,
             track_id integer not null references track(id)
         )",
            [],
        )
        .expect("creat table directory failed");
        Self {
            conn,
            config: config.clone(),
        }
    }
}
