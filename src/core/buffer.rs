#[derive(Debug, Default)]
pub struct FileBuffer {
    pub name: String,
    pub path_location: std::path::PathBuf,
    pub id: u32,
    pub had_content_saved: bool,
    pub is_open: bool,
    //pub content: Vec<>
}