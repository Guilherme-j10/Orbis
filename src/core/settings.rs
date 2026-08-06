use std::{
    collections::HashMap,
    io::{self, Read, Write},
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::{
    dtos::app::FontMappingState,
    font_engine::font::OrbParts,
    utils::constants::{MAGIC, MAP_FILE_NAME, VERSION},
};

#[derive(Debug)]
pub struct Settings {
    local_file_mapping: PathBuf,
}

impl Settings {
    pub fn new(pd: &ProjectDirs) -> Self {
        let data_local_dir = pd.data_local_dir();
        match data_local_dir.try_exists() {
            Ok(result) => {
                if result == false {
                    std::fs::create_dir_all(data_local_dir).expect("Error in set local data dir");
                }
            }
            Err(e) => {
                panic!("Error in verificate local data dir: {e}");
            }
        }

        Self {
            local_file_mapping: data_local_dir.join(MAP_FILE_NAME),
        }
    }

    pub fn save(&self, data: &FontMappingState) -> Result<String, std::io::Error> {
        let bind_chars_data = data.binded_char.borrow().clone();

        let payload = bincode::serialize(&bind_chars_data)
            .map_err(|e| std::io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut file = std::fs::File::create(&self.local_file_mapping)?;
        file.write_all(&MAGIC)?;
        file.write_all(&[VERSION])?;
        file.write_all(&payload)?;

        let local_save = self.local_file_mapping.to_string_lossy().to_string();
        Ok(local_save)
    }

    pub fn load(&self) -> Result<HashMap<String, Vec<OrbParts>>, std::io::Error> {
        let mut file = std::fs::File::open(&self.local_file_mapping)?;
        let mut buffer_file = Vec::new();
        file.read_to_end(&mut buffer_file)?;

        if buffer_file.len() < 5 || buffer_file[0..4] != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "corrupt or invalid mapping file",
            ));
        }

        let version = buffer_file[4];
        if version != VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "this version of font mapping is not suported",
            ));
        }

        let font_mapping: HashMap<String, Vec<OrbParts>> = bincode::deserialize(&buffer_file[5..])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(font_mapping)
    }
}
