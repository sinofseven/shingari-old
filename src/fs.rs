use std::fs;
use std::path::PathBuf;

pub fn read_text(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("failed to read text: {e}"))
}

pub fn write_text(path: &PathBuf, text: &str) -> Result<(), String> {
    let dir = path.parent().ok_or(format!(
        "failed to resolve parent dir of file: path={}",
        path.display()
    ))?;

    fs::create_dir_all(dir).map_err(|e| {
        format!(
            "failed to create directory: path={}, err={}",
            dir.display(),
            e
        )
    })?;

    fs::write(path, text).map_err(|e| format!("failed to write text: {e}"))
}
