use std::path::Path;

pub fn get_path(path: &String) -> Result<&Path, String> {
    let path = Path::new(path);
    if !path.exists() {
        Err(format!(
            "Path '{}' does not exists or is not accessible.",
            path.display()
        ))
    } else {
        Ok(path)
    }
}

pub fn get_ext(path: &Path) -> Result<String, String> {
    if let Some(ext) = path.extension() {
        Ok(ext.to_string_lossy().to_ascii_lowercase())
    } else {
        Err(format!("Path '{}' has no extension.", path.display()))
    }
}
