use std::path::PathBuf;

pub fn validate_image_path(path: &str) -> bool {
    let path = PathBuf::from(path);
    path.exists() && path.is_file()
}

pub fn get_file_extension(path: &str) -> Option<String> {
    PathBuf::from(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

pub fn is_supported_image_format(path: &str) -> bool {
    if let Some(ext) = get_file_extension(path) {
        matches!(ext.as_str(), "png" | "jpg" | "jpeg")
    } else {
        false
    }
}
