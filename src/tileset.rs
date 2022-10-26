use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use crate::wfc::{Tile};

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub name: String,
    pub tiles: Vec<Tile>
}

impl Tileset {
    pub fn new(file_path: String) -> Self {
        let mut file = File::open(file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let map: Tileset = serde_json::from_str(data.as_str()).unwrap();
        return map;
    }
}