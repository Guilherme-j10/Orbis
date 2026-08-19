pub const MAGIC: [u8; 4] = *b"ORB1";
pub const VERSION: u8 = 1;
pub const MAP_FILE_NAME: &'static str = "font_map.orb";

pub const FILE_ICON: &[u8] = include_bytes!("../../assets/file.svg");
pub const FILE_PLUS_ICON: &[u8] = include_bytes!("../../assets/file-plus-corner.svg");
pub const FOLDER_CLOSE_ICON: &[u8] = include_bytes!("../../assets/folder.svg");
pub const FOLDER_OPEN_ICON: &[u8] = include_bytes!("../../assets/folder-open.svg");
pub const FOLDER_PLUS_ICON: &[u8] = include_bytes!("../../assets/folder-plus.svg");