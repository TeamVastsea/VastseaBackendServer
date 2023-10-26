use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use crate::news::NewsInfo;

impl NewsInfo {
    pub async fn get_body(&self) -> String {
        let filename = self._id.clone() + ".md";

        fs::create_dir_all("news/").unwrap();
        let mut file = match File::open("news/".to_string() + &filename) {
            Ok(a) => { a }
            Err(_) => { return "".to_string(); }
        };
        let md = &mut "".to_string();
        file.read_to_string(md).unwrap();
        return md.to_string();
    }

    pub async fn save_body(&self, message: String) -> Result<(), ()> {
        let filename = self._id.to_string() + ".md";

        fs::create_dir_all("news/").unwrap();
        let mut file = File::create("news/".to_string() + &filename).unwrap();
        file.write_all(message.as_bytes()).unwrap();
        return Ok(());
    }
}