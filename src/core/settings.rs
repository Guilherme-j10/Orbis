use std::io::{self, Write};

use directories::ProjectDirs;

use crate::{
    interfaces::app::FontMappingState,
    utils::constants::{MAGIC, MAP_FILE_NAME, VERSION},
};

#[derive(Debug)]
pub struct Settings<'a> {
    project_dirs: &'a ProjectDirs,
}

impl<'a> Settings<'a> {
    pub fn new(pd: &'a ProjectDirs) -> Self {
        Self { project_dirs: pd }
    }

    pub fn save(&self, data: &FontMappingState) -> Result<String, std::io::Error> {
        let local_dir = self.project_dirs.data_local_dir();
        let bind_chars_data = data.binded_char.borrow().clone();

        let payload = bincode::serialize(&bind_chars_data)
            .map_err(|e| std::io::Error::new(io::ErrorKind::InvalidData, e))?;

        let local_file = local_dir.join(MAP_FILE_NAME);
        let mut file = std::fs::File::create(&local_file)?;
        file.write_all(&MAGIC)?;
        file.write_all(&[VERSION])?;
        file.write_all(&payload)?;

        let local_save = local_file.to_string_lossy().to_string();
        Ok(local_save)
    }
}
