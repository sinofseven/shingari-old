use dirs::home_dir;
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf, String> {
    home_dir()
        .map(|p| p.join(".config/pesn"))
        .ok_or(format!("failed to get home directory"))
}

pub fn get_config_file() -> Result<PathBuf, String> {
    get_config_dir().map(|p| p.join("config.yml"))
}

pub fn get_pid_json_dir() -> Result<PathBuf, String> {
    get_config_dir().map(|p| p.join("pids"))
}

pub fn get_pid_json_file(pid: u32) -> Result<PathBuf, String> {
    get_pid_json_dir().map(|p| p.join(format!("{pid}.json")))
}
